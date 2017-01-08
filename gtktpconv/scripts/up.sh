#!/bin/bash

time sh -ce "git pull -r
    rm -rf workdir
    mkdir -p workdir
    cd workdir

    CC=clang CXX=clang++ cmake -DCMAKE_INSTALL_PREFIX=$(pwd)/instdir -DENABLE_WERROR=ON -DCMAKE_BUILD_TYPE=Debug  ..
    make -j8
    make check" 2>&1 |tee log

# vim:set shiftwidth=4 expandtab:
