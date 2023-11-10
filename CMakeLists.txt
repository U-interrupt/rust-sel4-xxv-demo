cmake_minimum_required(VERSION 3.22.0)

include(settings.cmake)

project(xxv-demo NONE)

find_package(seL4 REQUIRED)
sel4_import_kernel()

# Elfloader settings that correspond to how Data61 sets its boards up.
find_package(elfloader-tool REQUIRED)
ApplyData61ElfLoaderSettings(${KernelPlatform} ${KernelSel4Arch})
elfloader_import_project()

find_package(musllibc REQUIRED)
find_package(util_libs REQUIRED)
find_package(seL4_libs REQUIRED)

# Need to disable GC sections as it causes our tests to be stripped sometimes
# This option is tested in the following musllibc_setup_build_environment_with_sel4runtime call.
set(UserLinkerGCSections OFF CACHE BOOL "" FORCE)
# This sets up environment build flags and imports musllibc and runtime libraries.
musllibc_setup_build_environment_with_sel4runtime()
sel4_import_libsel4()
util_libs_import_libraries()
sel4_libs_import_libraries()

# Set the roottask CNode size to be large enough for DTB, timer caps, etc
# Note that this works for the platforms that we have at the moment, and may
# need to be increased in the future
set(KernelRootCNodeSizeBits 13 CACHE INTERNAL "")

include(FetchContent)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.4.4 # Optionally specify a commit hash, version tag or branch here
)

set(Rust_CARGO_TARGET "riscv64imac-sel4" CACHE STRING "")
# Set any global configuration variables such as `Rust_TOOLCHAIN` before this line!
FetchContent_MakeAvailable(Corrosion)
corrosion_import_crate(
    MANIFEST_PATH "${CMAKE_SOURCE_DIR}/Cargo.toml"
    )
corrosion_set_env_vars(xxv-demo SEL4_PREFIX=path-to-sel4-install)
corrosion_set_linker(xxv-demo rust-lld)


# Build CPIO archive
include(cpio)
set(cpio_files "")
list(APPEND cpio_files "$<TARGET_FILE:xxv-demo>")
MakeCPIO(archive.o "${cpio_files}")

# Set this image as the rootserver
include(rootserver)
DeclareRootserver(xxv-demo)