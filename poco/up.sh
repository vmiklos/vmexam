#!/bin/bash

parallelism=$(getconf _NPROCESSORS_ONLN)
parallelism=1
time sh -c "git pull -r && \
    ./configure --cflags=\"$(cat configure.input)\" && \
    make clean && \
    make -s -j$parallelism CXX='ccache g++'" 2>&1 |tee log
if grep -q Werror= log; then
    echo "Found warnings to fix."
fi

# vim:set shiftwidth=4 expandtab:
