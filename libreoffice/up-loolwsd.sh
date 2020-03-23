#!/bin/bash -ex

time (
    git pull -r
    # make distclean is broken
    if [ -e Makefile ]; then
        make clean
    fi
    ./autogen.sh

    # If sanitizers already set a CC/CXX, don't overwrite it.
    if [ -z "$CC" ]; then
        export CC="ccache $HOME/git/llvm/instdir/bin/clang"
    fi
    if [ -z "$CXX" ]; then
        export CXX="ccache $HOME/git/llvm/instdir/bin/clang++"
    fi
    ./configure \
        --prefix=$PWD/install \
        --enable-debug \
        --with-lo-path=$HOME/git/libreoffice/master/instdir \
        --with-lokit-path=$HOME/git/libreoffice/master/include \
        CFLAGS="-g -O0 $CFLAGS" \
        CXXFLAGS="-g -O0 $CXXFLAGS" \

    # Self-built poco:
    #    --with-poco-includes=$HOME/git/poco/install/include \
    #    --with-poco-libs=$HOME/git/poco/lib/Linux/x86_64 \
    # #if ENABLE_SUPPORT_KEY:
    #    --with-support-public-key=$HOME/downloads/vmiklos.pem \

    make -j$(getconf _NPROCESSORS_ONLN)
    make ctags
    make check
) 2>&1 |tee log

# vim:set shiftwidth=4 expandtab:
