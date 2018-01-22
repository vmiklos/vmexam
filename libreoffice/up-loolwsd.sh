#!/bin/bash -ex

git pull -r
./autogen.sh
./configure \
    --prefix=$PWD/install \
    --enable-debug \
    --with-lo-path=$HOME/git/libreoffice/master/instdir \
    --with-lokit-path=$HOME/git/libreoffice/master/include \
    CC="ccache $HOME/git/llvm/instdir/bin/clang" \
    CXX="ccache $HOME/git/llvm/instdir/bin/clang++" \
    CFLAGS='-g -O0' \
    CXXFLAGS='-g -O0' \

# Self-built poco:
#    --with-poco-includes=$HOME/git/poco/install/include \
#    --with-poco-libs=$HOME/git/poco/lib/Linux/x86_64 \

make clean
make -j$(getconf _NPROCESSORS_ONLN)
make ctags
make check

# vim:set shiftwidth=4 expandtab:
