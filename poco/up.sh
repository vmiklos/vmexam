#!/bin/bash

time sh -c "git pull -r && \
    ./configure --cflags=\"$(cat configure.input)\" && \
    make clean && \
    make -j$(getconf _NPROCESSORS_ONLN)" 2>&1 |tee log

# vim:set shiftwidth=4 expandtab:
