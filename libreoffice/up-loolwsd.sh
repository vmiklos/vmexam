#!/bin/bash

time sh -c "git pull -r && \
    ./autogen.sh && \
    ./configure --prefix=$PWD/install --enable-debug --with-lo-path=$HOME/git/libreoffice/master/instdir --with-lokit-path=$HOME/git/libreoffice/master/include CC='ccache gcc' CXX='ccache g++' && \
    make clean && \
    make -j$(getconf _NPROCESSORS_ONLN) && \
    make ctags && \
    make check && \
    style-check-files"

# Self-built poco: --with-poco-includes=$HOME/git/poco/install/include --with-poco-libs=$HOME/git/poco/lib/Linux/x86_64

# Would be nice to append '2>&1 |tee log' at the end, that currently makes
# 'make check' fail for some reason.

# vim:set shiftwidth=4 expandtab:
