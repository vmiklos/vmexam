#!/bin/bash

time sh -ce "git pull -r
    cd tools/clang
    git pull -r
    cd tools/extra
    git pull -r
    git svn rebase -l
    cd ../../../../workdir
    cmake -G 'Unix Makefiles' -DCMAKE_INSTALL_PREFIX=$(pwd)/instdir -DCMAKE_BUILD_TYPE=Release ..
    make -j8
    make -j8 check-clang-tools
    make install" 2>&1 |tee log

# vim:set shiftwidth=4 expandtab:
