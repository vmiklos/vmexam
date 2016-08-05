#!/bin/bash

time sh -ce "git pull -r
    cd projects/compiler-rt
    git pull -r
    cd ../../tools/clang
    git pull -r
    cd tools/extra
    git pull -r
    git svn rebase -l
    cd ../../../../workdir
    cmake -G 'Unix Makefiles' -DCMAKE_INSTALL_PREFIX=$(pwd)/instdir -DCMAKE_BUILD_TYPE=Release -DLLVM_ENABLE_ASSERTIONS=ON -DLLVM_ENABLE_SPHINX=ON ..
    make -j8
    make -j8 check-clang-tools
    make install
    ~/git/vmexam/llvm/llvm-style-check-files" 2>&1 |tee log

# vim:set shiftwidth=4 expandtab:
