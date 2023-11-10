#![no_std]
#![no_main]
#![feature(never_type)]
#![feature(alloc_error_handler)]

extern crate alloc;

pub mod axidma;
pub mod config;
mod mm;
pub mod net;
pub mod plic;
mod xxv_demo;

use core::sync::atomic::AtomicU16;

use lazy_static::*;
use riscv::register::time;
use sel4::{
    cap_type::{IRQHandler, MegaPage, Notification, PageTable, Untyped},
    BootInfo, CapRights, FrameSize, ObjectBlueprint, ObjectBlueprintArch, VMAttributes,
};
use sel4_logging::{log::debug, LevelFilter, Logger, LoggerBuilder};
use sel4_root_task::debug_print;
use sel4_root_task::root_task;

use crate::{
    config::{KERNEL_HEAP_SIZE, NET_DEVICE_BASE},
    xxv_demo::xxv_dma_example_sel4_irq,
};

const LOG_LEVEL: LevelFilter = LevelFilter::Trace;
lazy_static! {
    pub static ref HAS_INTR: [AtomicU16; config::CPU_NUM] =
        array_init::array_init(|_| AtomicU16::new(0));
}
static LOGGER: Logger = LoggerBuilder::const_default()
    .level_filter(LOG_LEVEL)
    .write(|s| debug_print!("{}", s))
    .build();

// fn clear_bss() {
//     extern "C" {
//         fn s_bss();
//         fn e_bss();
//         fn e_bss_ma();
//     }
//     (s_bss as usize..e_bss_ma as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
//     debug!(
//         "s_bss: {:#x?}, e_bss: {:#x?}, e_bss_ma: {:#x?}",
//         s_bss as usize, e_bss as usize, e_bss_ma as usize
//     );
// }

#[root_task]
fn main(bootinfo: &BootInfo) -> sel4::Result<!> {
    sel4::debug_println!("Hello, Xxv!");
    // clear_bss();
    LOGGER.set().unwrap();

    debug!("bootinfo: {:?}", bootinfo);
    debug!(
        "untyped region: {} - {}",
        bootinfo.untyped().start,
        bootinfo.untyped().end
    );
    for desc in bootinfo.untyped_list() {
        debug!(
            "untyped: paddr: {:x}, size_bits: {}, is_device: {}",
            desc.paddr(),
            desc.size_bits(),
            desc.is_device()
        );
    }

    sel4::debug_println!("Hello, Xxv?");
    let mut empty_slots = bootinfo.empty();
    for _ in 0..10 {
        let _ = empty_slots.next().unwrap();
    }
    let cnode = BootInfo::init_thread_cnode();

    // mega page size
    let blueprint = ObjectBlueprint::Untyped {
        size_bits: FrameSize::MEGA_BITS,
    };
    let retype_bits = 4;
    let retype_num = 1 << retype_bits;
    let slot = bootinfo.untyped().start
        + bootinfo
            .untyped_list()
            .iter()
            .position(|desc| {
                !desc.is_device()
                    && desc.size_bits() >= blueprint.physical_size_bits() + retype_bits
            })
            .unwrap();
    debug!("untyped slot: {}", slot);
    let untyped = BootInfo::init_cspace_local_cptr::<Untyped>(slot);
    let untyped_slot = empty_slots.next().unwrap();
    debug!("new untyped slot: {}", untyped_slot);
    // create more untyped memory objects
    untyped.untyped_retype(&blueprint, &cnode.relative_self(), untyped_slot, retype_num)?;
    for _ in 0..retype_num {
        let _ = empty_slots.next().unwrap();
    }

    // init page table
    let blueprint = ObjectBlueprint::Arch(ObjectBlueprintArch::PageTable);
    let page_table_slot = empty_slots.next().unwrap();
    debug!("page table slot: {}", page_table_slot);
    let page_table_untyped = BootInfo::init_cspace_local_cptr::<Untyped>(untyped_slot);
    page_table_untyped.untyped_retype(
        &blueprint,
        &cnode.relative_self(),
        page_table_slot,
        retype_num,
    )?;
    for _ in 0..retype_num {
        let _ = empty_slots.next().unwrap();
    }

    let blueprint = ObjectBlueprint::Arch(ObjectBlueprintArch::MegaPage);
    let heap_page_slot = empty_slots.next().unwrap();
    let heap_page_untyped_slot = untyped_slot + 1;
    let heap_page_untyped = BootInfo::init_cspace_local_cptr::<Untyped>(heap_page_untyped_slot);
    heap_page_untyped.untyped_retype(&blueprint, &cnode.relative_self(), heap_page_slot, 1)?;
    let heap_page = BootInfo::init_cspace_local_cptr::<MegaPage>(heap_page_slot);

    let l2_page_table = BootInfo::init_cspace_local_cptr::<PageTable>(page_table_slot);
    // identity map for heap space
    let vaddr = heap_page.frame_get_address().unwrap();
    debug!("heap_page paddr: {:#x}", vaddr);
    let root_page_table = BootInfo::init_thread_vspace();
    l2_page_table.page_table_map(root_page_table, vaddr, VMAttributes::DEFAULT)?;
    heap_page.frame_map(
        root_page_table,
        vaddr,
        CapRights::read_write(),
        VMAttributes::DEFAULT,
    )?;

    mm::init_heap(heap_page.frame_get_address().unwrap(), KERNEL_HEAP_SIZE);

    debug!("finding untyped for xxv eth");
    let (mut xxv_untyped, mut xxv_untyped_bits) =
        (BootInfo::init_cspace_local_cptr::<Untyped>(0), 0);
    for (i, desc) in bootinfo.untyped_list().iter().enumerate() {
        if desc.is_device()
            && desc.paddr() <= crate::config::NET_DEVICE_BASE
            && desc.paddr() + (1 << desc.size_bits()) > crate::config::NET_DEVICE_BASE
        {
            debug!(
                "xxv untyped: paddr: {:x}, size_bits: {}, is_device: {}",
                desc.paddr(),
                desc.size_bits(),
                desc.is_device()
            );
            xxv_untyped = BootInfo::init_cspace_local_cptr::<Untyped>(bootinfo.untyped().start + i);
            xxv_untyped_bits = desc.size_bits();
            break;
        }
    }

    let xxv_untype_slot = empty_slots.next().unwrap();
    let retype_bits = xxv_untyped_bits - FrameSize::MEGA_BITS;
    let retype_num = 1 << retype_bits;
    let blueprint = ObjectBlueprint::Untyped {
        size_bits: FrameSize::MEGA_BITS,
    };
    xxv_untyped.untyped_retype(
        &blueprint,
        &cnode.relative_self(),
        xxv_untype_slot,
        retype_num,
    )?;
    for _ in 0..retype_num {
        let _ = empty_slots.next().unwrap();
    }
    let xxv_frame_slot = empty_slots.next().unwrap();
    debug!("xxv_frame_slot: {}", xxv_frame_slot);

    for i in 0..retype_num {
        let blueprint = ObjectBlueprint::Arch(ObjectBlueprintArch::MegaPage);
        let xxv_frame_untyped = BootInfo::init_cspace_local_cptr::<Untyped>(xxv_untype_slot + i);
        xxv_frame_untyped.untyped_retype(
            &blueprint,
            &cnode.relative_self(),
            xxv_frame_slot + i,
            1,
        )?;
        let _ = empty_slots.next().unwrap();
        let xxv_frame = BootInfo::init_cspace_local_cptr::<MegaPage>(xxv_frame_slot + i);
        let paddr = xxv_frame.frame_get_address()?;
        if paddr <= NET_DEVICE_BASE && paddr + (1 << FrameSize::MEGA_BITS) > NET_DEVICE_BASE {
            debug!("xxv frame: paddr: {:x}", paddr);
            let vaddr = xxv_frame.frame_get_address().unwrap();
            let l2_page_table = BootInfo::init_cspace_local_cptr::<PageTable>(page_table_slot + 1);
            // identity map for heap space
            l2_page_table.page_table_map(root_page_table, vaddr, VMAttributes::DEFAULT)?;
            xxv_frame.frame_map(
                root_page_table,
                vaddr,
                CapRights::read_write(),
                VMAttributes::DEFAULT,
            )?;
            break;
        }
    }

    let unbadged_notification_slot = empty_slots.next().unwrap();
    let badged_notification_slot = empty_slots.next().unwrap();
    debug!("badged_notification_slot: {}", badged_notification_slot);

    let unbadged_notification =
        BootInfo::init_cspace_local_cptr::<Notification>(unbadged_notification_slot);

    let badged_notification =
        BootInfo::init_cspace_local_cptr::<Notification>(badged_notification_slot);

    let untyped = BootInfo::init_cspace_local_cptr::<Untyped>(untyped_slot + 2);
    let blueprint = sel4::ObjectBlueprint::Notification;

    untyped.untyped_retype(
        &blueprint,
        &cnode.relative_self(),
        unbadged_notification_slot,
        1,
    )?;

    let badge = 0x1337;

    cnode.relative(badged_notification).mint(
        &cnode.relative(unbadged_notification),
        sel4::CapRights::write_only(),
        badge,
    )?;
    debug!("badge minted");

    badged_notification.signal();

    debug!("badge signaled");
    let (_, observed_badge) = unbadged_notification.wait();

    sel4::debug_println!("badge = {:#x}", badge);
    assert_eq!(observed_badge, badge);

    let irq_ctrl = BootInfo::irq_control();
    let mm2s_irq_handler_slot = empty_slots.next().unwrap();
    let mm2s_irq_handler = BootInfo::init_cspace_local_cptr::<IRQHandler>(mm2s_irq_handler_slot);
    debug!("mm2s_irq_handler_slot: {}", mm2s_irq_handler_slot);
    irq_ctrl.irq_control_get(config::MM2S_IRQ, &cnode.relative(mm2s_irq_handler))?;

    let s2mm_irq_handler_slot = empty_slots.next().unwrap();
    let s2mm_irq_handler = BootInfo::init_cspace_local_cptr::<IRQHandler>(s2mm_irq_handler_slot);
    debug!("s2mm_irq_handler_slot: {}", s2mm_irq_handler_slot);
    irq_ctrl.irq_control_get(config::S2MM_IRQ, &cnode.relative(s2mm_irq_handler))?;

    let mm2s_irq_ntfn_slot = empty_slots.next().unwrap();
    let mm2s_irq_ntfn = BootInfo::init_cspace_local_cptr::<Notification>(mm2s_irq_ntfn_slot);
    let s2mm_irq_ntfn_slot = empty_slots.next().unwrap();
    let s2mm_irq_ntfn = BootInfo::init_cspace_local_cptr::<Notification>(s2mm_irq_ntfn_slot);
    assert_eq!(mm2s_irq_ntfn_slot + 1, s2mm_irq_ntfn_slot);
    debug!("s2mm_irq_ntfn_slot: {}", s2mm_irq_ntfn_slot);

    let blueprint = ObjectBlueprint::Notification;
    let untyped = BootInfo::init_cspace_local_cptr::<Untyped>(untyped_slot + 3);

    untyped.untyped_retype(&blueprint, &cnode.relative_self(), mm2s_irq_ntfn_slot, 2)?;
    mm2s_irq_handler.irq_handler_set_notification(mm2s_irq_ntfn)?;
    s2mm_irq_handler.irq_handler_set_notification(s2mm_irq_ntfn)?;

    xxv_dma_example_sel4_irq(
        mm2s_irq_handler,
        mm2s_irq_ntfn,
        s2mm_irq_handler,
        s2mm_irq_ntfn,
    );

    sel4::debug_println!("TEST_PASS");

    BootInfo::init_thread_tcb().tcb_suspend()?;
    unreachable!()
}

pub fn delay(ms: usize) {
    let start = time::read();
    while time::read() - start < config::CLOCK_FREQ * ms / 1000 {}
}
