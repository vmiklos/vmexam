#!/bin/bash

time sh -c "git pull -r && \
    autoreconf &&
    automake --add-missing &&
    ./configure --prefix=$PWD/install --enable-debug --with-lokit-path=$HOME/git/libreoffice/master/include \
    make clean && \
    make &&
    make tags" 2>&1 |tee log

# Would be nice to run 'make check' here, but we're not there yet, currently
# loolwsd has to be started manually first.

# vim:set shiftwidth=4 expandtab:
