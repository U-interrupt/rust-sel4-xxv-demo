cmake_minimum_required(VERSION 3.7.2)

set(project_dir "${CMAKE_CURRENT_LIST_DIR}/../../")
file(GLOB project_modules ${project_dir}/projects/*)
list(
    APPEND
        CMAKE_MODULE_PATH
        ${project_dir}/kernel
        ${project_dir}/tools/seL4/cmake-tool/helpers/
        ${project_dir}/tools/seL4/elfloader-tool/
        ${project_modules}
)

set(NANOPB_SRC_ROOT_FOLDER "${project_dir}/tools/nanopb" CACHE INTERNAL "")
set(OPENSBI_PATH "${project_dir}/tools/opensbi" CACHE STRING "OpenSBI Folder location")

include(application_settings)
include(${CMAKE_CURRENT_LIST_DIR}/easy-settings.cmake)

# Advanced mode OFF by default
set(SEL4_CONFIG_DEFAULT_ADVANCED ON)

# Platform check an setup
correct_platform_strings()

find_package(seL4 REQUIRED)
sel4_configure_platform_settings()

set(valid_platforms ${KernelPlatform_all_strings} ${correct_platform_strings_platform_aliases})
set_property(CACHE PLATFORM PROPERTY STRINGS ${valid_platforms})
if(NOT "${PLATFORM}" IN_LIST valid_platforms)
    message(FATAL_ERROR "Invalid PLATFORM selected: \"${PLATFORM}\"
        Valid platforms are: \"${valid_platforms}\"")
endif()

ApplyCommonReleaseVerificationSettings(${RELEASE} ${VERIFICATION})

if(SMP)
    if(NUM_NODES MATCHES "^[0-9]+$")
        set(KernelMaxNumNodes ${NUM_NODES} CACHE STRING "" FORCE)
    else()
        set(KernelMaxNumNodes 4 CACHE STRING "" FORCE)
    endif()
else()
    set(KernelMaxNumNodes 1 CACHE STRING "" FORCE)
endif()

if(UINTR)
    set(KernelRiscvUintr ON CACHE STRING "" FORCE)
endif()