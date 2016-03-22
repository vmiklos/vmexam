#!/bin/bash

time sh -c "git pull -r && \
    autoreconf && \
    automake --add-missing && \
    ./configure --prefix=$PWD/install --enable-debug --with-lokit-path=$HOME/git/libreoffice/master/include CC='ccache gcc' CXX='ccache g++' && \
    make clean && \
    make -j$(getconf _NPROCESSORS_ONLN) && \
    make ctags && \
    style-check-files" 2>&1 |tee log

# Self-built poco: --with-poco-includes=$HOME/git/poco/install/include --with-poco-libs=$HOME/git/poco/lib/Linux/x86_64

# Would be nice to run 'make check' here, but we're not there yet, currently
# loolwsd has to be started manually first.

# vim:set shiftwidth=4 expandtab:
