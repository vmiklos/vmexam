#!/bin/bash

time sh -ce "git pull -r
    mkdir -p workdir
    cd projects/compiler-rt
    git pull -r
    cd ../../tools/clang
    git pull -r
    cd tools/extra
    git pull -r
    git svn rebase -l
    cd ../../../../workdir
    cmake -G 'Unix Makefiles' -DCMAKE_INSTALL_PREFIX=$(pwd)/instdir -DCMAKE_BUILD_TYPE=Release -DLLVM_ENABLE_ASSERTIONS=ON -DLLVM_ENABLE_SPHINX=ON -DBUILD_SHARED_LIBS=ON ..
    make -j8
    make -j8 check-clang-tools
    make install
    ~/git/vmexam/llvm/llvm-style-check-files" 2>&1 |tee log

# How to enable symbols for profiling: -DCMAKE_BUILD_TYPE=RelWithDebInfo -DCMAKE_C_FLAGS='-fno-omit-frame-pointer' -DCMAKE_CXX_FLAGS='-fno-omit-frame-pointer'

# vim:set shiftwidth=4 expandtab:
