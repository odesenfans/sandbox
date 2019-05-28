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

    std::vector<unsigned> expected = {
        0, 1, 2, 3, 4,
        10, 6, 7, 8, 14,
        5, 11, 12, 13, 9,
        15, 11, 12, 13, 19,
        10, 11, 12, 13, 14,
    };

    modern::median_blur(&img[0], width, height, kernelsize);

    for (int i = 0; i < width; ++i) {
        for (int j = 0; j < height; ++j) {
            std::cout << img[i * width + j] << " ";
        }
        std::cout << std::endl;
    }

    for (size_t i = 0; i < img.size(); ++i) {
        EXPECT_EQ(expected[i], img[i]);
    }
}
