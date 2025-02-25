cmake_minimum_required(VERSION 3.25)
project(foxglove-sdk)

option(ENABLE_LTO "Enable link-time optimization" ON)

include(FetchContent)

FetchContent_Declare(
  Corrosion
  GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
  GIT_TAG v0.5
)
FetchContent_MakeAvailable(Corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(MANIFEST_PATH Cargo.toml CRATES foxglove_sdk_cxxbridge)

corrosion_add_cxxbridge(foxglove_sdk_cxxbridge_cpp
  CRATE foxglove_sdk_cxxbridge
  # REGEN_TARGET <regen_target_name>
  FILES lib.rs
)

add_library(foxglove STATIC src/dummy.cpp)
set_property(TARGET foxglove PROPERTY CXX_STANDARD 20)
set_property(TARGET foxglove PROPERTY CXX_STANDARD_REQUIRED True)
target_include_directories(foxglove PUBLIC include)
target_link_libraries(foxglove PRIVATE foxglove_sdk_cxxbridge_cpp)

# if(ENABLE_LTO)
#   include(CheckIPOSupported)
#   check_ipo_supported()
#   corrosion_add_target_local_rustflags(foxglove_sdk_cxxbridge -Clinker-plugin-lto)
#   # set_property(TARGET foxglove_sdk_cxxbridge PROPERTY INTERPROCEDURAL_OPTIMIZATION TRUE)
#   # set_property(TARGET foxglove_sdk_cxxbridge_cpp PROPERTY INTERPROCEDURAL_OPTIMIZATION TRUE)
#   # target_link_options(foxglove_sdk_cxxbridge INTERFACE -fuse-ld=lld)
#   # target_link_options(foxglove_sdk_cxxbridge_cpp PRIVATE -fuse-ld=lld)
#   set_property(TARGET foxglove PROPERTY INTERPROCEDURAL_OPTIMIZATION TRUE)
#   # set_property(TARGET foxglove PROPERTY LINKER_TYPE LLD)
#   # target_link_options(foxglove PRIVATE -fuse-ld=lld)
# endif()
