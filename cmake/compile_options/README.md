# Modern ways to set compiler options in CMake

Tests the behavior of the `target_compile_options` CMake command.
Additionally, the `-pedantic` flag is added to one of the source files.

## How to

From the source directory, run the following commands:

```sh
mkdir build && cd build
cmake ..
make -j1 VERBOSE=1
```

The flags for `a.cpp` and `main.cpp` should be the same. `b.cpp` should be
compiled with the same flags and the pedantic flag as well.
