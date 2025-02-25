cmake_minimum_required(VERSION 3.25)
project(cpp-example-ws-server)

option(ENABLE_LTO "Enable link-time optimization" ON)

add_subdirectory(../../foxglove ../../foxglove-Release)

add_executable(example_server src/main.cpp)
set_property(TARGET example_server PROPERTY CXX_STANDARD 20)
set_property(TARGET example_server PROPERTY CXX_STANDARD_REQUIRED True)
target_link_libraries(example_server foxglove)

# if(ENABLE_LTO)
#   include(CheckIPOSupported)
#   check_ipo_supported()
#   set_property(TARGET example_server PROPERTY INTERPROCEDURAL_OPTIMIZATION TRUE)
#   if (CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
#     set_property(TARGET example_server PROPERTY LINKER_TYPE LLD)
#     target_link_options(example_server PRIVATE -Wl,-dead_strip -O3)
#   endif()
# endif()
