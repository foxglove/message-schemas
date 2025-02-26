cmake_minimum_required(VERSION 3.25)
project(foxglove-sdk)

include(CTest)
include(FetchContent)

FetchContent_Declare(
  Catch2
  GIT_REPOSITORY https://github.com/catchorg/Catch2.git
  GIT_TAG        v3.8.0
)
FetchContent_MakeAvailable(Catch2)
list(APPEND CMAKE_MODULE_PATH ${catch2_SOURCE_DIR}/extras)
include(Catch)

add_library(foxglove_c STATIC IMPORTED)
if(WIN32)
  set_property(TARGET foxglove_c PROPERTY IMPORTED_LOCATION_DEBUG ${CMAKE_CURRENT_SOURCE_DIR}/../../target/debug/foxglove_c.lib)
  set_property(TARGET foxglove_c PROPERTY IMPORTED_LOCATION_RELEASE ${CMAKE_CURRENT_SOURCE_DIR}/../../target/release/foxglove_c.lib)
  set_property(TARGET foxglove_c PROPERTY IMPORTED_LOCATION_RELWITHDEBINFO ${CMAKE_CURRENT_SOURCE_DIR}/../../target/release/foxglove_c.lib)
else()
  set_property(TARGET foxglove_c PROPERTY IMPORTED_LOCATION_DEBUG ${CMAKE_CURRENT_SOURCE_DIR}/../../target/debug/libfoxglove_c.a)
  set_property(TARGET foxglove_c PROPERTY IMPORTED_LOCATION_RELEASE ${CMAKE_CURRENT_SOURCE_DIR}/../../target/release/libfoxglove_c.a)
  set_property(TARGET foxglove_c PROPERTY IMPORTED_LOCATION_RELWITHDEBINFO ${CMAKE_CURRENT_SOURCE_DIR}/../../target/release/libfoxglove_c.a)
endif()
target_include_directories(foxglove_c INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/../../c/foxglove/generated/include)

file(GLOB_RECURSE foxglove_srcs CONFIGURE_DEPENDS
    "include/*.hpp"
    "src/*.hpp"
    "src/*.cpp"
)

add_library(foxglove STATIC "${foxglove_srcs}")
set_property(TARGET foxglove PROPERTY CXX_STANDARD 20)
set_property(TARGET foxglove PROPERTY CXX_STANDARD_REQUIRED True)
target_include_directories(foxglove PUBLIC include)
target_link_libraries(foxglove PRIVATE foxglove_c)

file(GLOB_RECURSE foxglove_test_srcs CONFIGURE_DEPENDS
    "tests/*.hpp"
    "tests/*.cpp"
)
add_executable(tests "${foxglove_test_srcs}")
target_link_libraries(tests PRIVATE foxglove Catch2::Catch2WithMain)
catch_discover_tests(tests)
