#! /bin/bash

DEPS_DIR=dependencies

rm -rf ${DEPS_DIR}
mkdir -p ${DEPS_DIR}
cd ${DEPS_DIR}

git clone --depth=1 https://github.com/google/googletest.git
