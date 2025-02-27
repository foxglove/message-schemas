cmake_minimum_required(VERSION 3.25)
project(cpp-example-ws-server)

add_subdirectory(../../foxglove ../../foxglove-Release)

add_executable(example_server src/main.cpp)
set_property(TARGET example_server PROPERTY CXX_STANDARD 17)
set_property(TARGET example_server PROPERTY CXX_STANDARD_REQUIRED True)
target_link_libraries(example_server foxglove)
