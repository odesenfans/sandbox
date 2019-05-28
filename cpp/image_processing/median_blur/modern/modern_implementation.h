/**
 * Single-threaded implementation of the median blur filter using modern C++.
 */

#ifndef DEF_MODERN_IMPLEMENTATION_H
#define DEF_MODERN_IMPLEMENTATION_H

namespace modern {

template <class T>
void median_blur(T* input, int width, int height, int kernelsize)
{
    // TODO Check if kernelsize is not pair
    const int length = kernelsize * kernelsize;

    // Temporary output image
    std::vector<T> output(width * height);

    // Vector used to store the elements to compute the median
    std::vector<T> median_elems(length);

    const int shift = (kernelsize - 1) / 2;

    for (int i = 0; i < width; ++i) {
        for (int j = 0; j < height; ++j) {
            if ((i >= shift) && (i < (width - shift)) && (j >= shift) && (j < (height - shift))) {
                const int start_row = i - shift;
                const int start_col = j - shift;

                // Get kernel centered around each point
                for (int r = 0; r < kernelsize; ++r) {
                    for (int c = 0; c < kernelsize; ++c) {
                        const int row = start_row + r;
                        const int col = start_col + c;
                        const int idx = kernelsize * r + c;

                        median_elems[idx] = input[row * width + col];
                    }
                }

                // Sort the vector
                std::sort(median_elems.begin(), median_elems.end());
                // Store the median element
                output[i * width + j] = median_elems[(length - 1) / 2];
            }
            else {
                output[i * width + j] = input[i * width + j];
            }
        }
    }

    // Copy the output vector back to the initial image
    // TODO check if this step can be avoided by modifying the API
    std::copy(output.begin(), output.end(), input);
} // namespace modern
} // namespace modern

#endif /* DEF_MODERN_IMPLEMENTATION_H */
