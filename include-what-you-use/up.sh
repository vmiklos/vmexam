#!/bin/bash -ex

# cmake prefix path is meant to be valid for a self-built clang

git pull -r
rm -rf workdir instdir
mkdir -p workdir
cd workdir
cmake \
    -G 'Unix Makefiles' \
    -DCMAKE_INSTALL_PREFIX=$PWD/../instdir \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_PREFIX_PATH=$HOME/git/llvm/instdir \
    -DCMAKE_C_COMPILER="clang" \
    -DCMAKE_CXX_COMPILER="clang++" \
    ..
make -j8
make install
cd ../instdir
ln -s $HOME/git/llvm/instdir/lib

# vim:set shiftwidth=4 expandtab:
