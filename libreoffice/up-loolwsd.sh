#!/bin/bash

time sh -c "git pull -r && \
    autoreconf &&
    automake --add-missing &&
    ./configure --prefix=$PWD/install --enable-debug --with-lokit-path=$HOME/git/libreoffice/master/include \
    make clean && \
    make &&
    make tags && \
    make check" 2>&1 |tee log

# vim:set shiftwidth=4 expandtab:
