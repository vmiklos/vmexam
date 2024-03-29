#!/bin/bash -ex

time (
    git pull -r
    mkdir -p workdir
    cd workdir
    cmake \
        -G 'Unix Makefiles' \
        -DCMAKE_INSTALL_PREFIX=$PWD/../instdir \
        -DCMAKE_BUILD_TYPE=Release \
        -DLLVM_ENABLE_SPHINX=ON \
        -DCMAKE_C_COMPILER="gcc" \
        -DCMAKE_CXX_COMPILER="g++" \
        -DLLVM_ENABLE_PROJECTS="compiler-rt;clang;clang-tools-extra;lld" \
        ../llvm
    make -j$(getconf _NPROCESSORS_ONLN)
    make install
    make -j$(getconf _NPROCESSORS_ONLN) check-clang-tools
) 2>&1 |tee log
exit ${PIPESTATUS[0]}

# How to enable symbols for profiling: -DCMAKE_BUILD_TYPE=RelWithDebInfo -DCMAKE_C_FLAGS='-fno-omit-frame-pointer' -DCMAKE_CXX_FLAGS='-fno-omit-frame-pointer'
# How to enable shared libs: -DBUILD_SHARED_LIBS=ON
# When changing llvm itself: -DLLVM_ENABLE_ASSERTIONS=ON

# vim:set shiftwidth=4 expandtab:
