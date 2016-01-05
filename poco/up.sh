#!/bin/bash

time sh -c "git pull -r && \
    ./configure --cflags=\"$(cat configure.input)\" && \
    make clean && \
    make -s -j$(getconf _NPROCESSORS_ONLN)" 2>&1 |tee log
if grep -q Werror= log; then
    echo "Found warnings to fix."
fi

# vim:set shiftwidth=4 expandtab:
