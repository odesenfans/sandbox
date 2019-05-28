cmake_minimum_required(VERSION 3.5)

set(CMAKE_CXX_STANDARD 11)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wextra")

project(median_blur CXX)

set(GTEST_DIR ${CMAKE_CURRENT_LIST_DIR}/dependencies/googletest)

add_subdirectory(${GTEST_DIR})

add_executable(median_blur_tests median_blur_tests.cpp)

target_link_libraries(
    median_blur_tests
    PRIVATE
    gtest
    gtest_main
)
