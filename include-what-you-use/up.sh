#!/bin/bash -ex

# cmake prefix path is meant to be valid in a Travis env

git pull -r
mkdir -p workdir
cd workdir
cmake \
    -G 'Unix Makefiles' \
    -DCMAKE_INSTALL_PREFIX=$PWD/../instdir \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_PREFIX_PATH=/usr/lib/llvm-7 \
    ..
make -j8
make install

# vim:set shiftwidth=4 expandtab:
