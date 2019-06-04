cmake_minimum_required(VERSION 3.5)

set(CMAKE_CXX_STANDARD 17)

project(compile_options CXX)

add_executable(compile_options a.cpp b.cpp main.cpp)

target_compile_options(compile_options PRIVATE -Wall -Wextra)

set_source_files_properties(b.cpp PROPERTIES COMPILE_FLAGS -pedantic)
