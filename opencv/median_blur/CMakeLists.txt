cmake_minimum_required(VERSION 2.8)
project(median_blur CXX)

find_package(OpenCV REQUIRED )

add_executable(average median_blur_main.cpp)

target_link_libraries(average ${OpenCV_LIBS})
