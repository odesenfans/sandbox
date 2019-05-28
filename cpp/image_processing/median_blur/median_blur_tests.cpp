#include <gtest/gtest.h>
#include <stdint.h>
#include <iostream>

#include "modern/modern_implementation.h"

using namespace ::testing;

TEST(Modern, U8Image)
{
    constexpr int width = 5;
    constexpr int height = 5;
    constexpr int kernelsize = 3;

    std::vector<unsigned> img = {
        0, 1, 2, 3, 4,
        10, 11, 12, 13, 14,
        5, 6, 7, 8, 9,
        15, 16, 17, 18, 19,
        10, 11, 12, 13, 14,
    };

    for (int i = 0; i < width; ++i) {
        for (int j = 0; j < height; ++j) {
            std::cout << img[i * width + j] << " ";
        }
        std::cout << std::endl;
    }

    modern::median_blur(&img[0], width, height, kernelsize);

    for (int i = 0; i < width; ++i) {
        for (int j = 0; j < height; ++j) {
            std::cout << img[i * width + j] << " ";
        }
        std::cout << std::endl;
    }

    ASSERT_EQ(0, 0);
}
