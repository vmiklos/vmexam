#!/bin/bash -ex

time (
    git pull -r
    mkdir -p workdir
    cd workdir
    cmake \
        -G 'Unix Makefiles' \
        -DCMAKE_INSTALL_PREFIX=$PWD/../instdir \
        -DCMAKE_BUILD_TYPE=Release \
        -DLLVM_ENABLE_ASSERTIONS=ON \
        -DLLVM_ENABLE_SPHINX=ON \
        -DCMAKE_C_COMPILER="clang" \
        -DCMAKE_CXX_COMPILER="clang++" \
        -DLLVM_ENABLE_PROJECTS="compiler-rt;clang;clang-tools-extra" \
        ../llvm
    make -j8
    make install
    make -j8 check-clang-tools
) 2>&1 |tee log

# How to enable symbols for profiling: -DCMAKE_BUILD_TYPE=RelWithDebInfo -DCMAKE_C_FLAGS='-fno-omit-frame-pointer' -DCMAKE_CXX_FLAGS='-fno-omit-frame-pointer'
# How to enable shared libs: -DBUILD_SHARED_LIBS=ON

# vim:set shiftwidth=4 expandtab:
