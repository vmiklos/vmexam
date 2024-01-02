#!/bin/bash -e

# Dependencies:
# zypper in ccache
# zypper in libcap-devel
# zypper in libcap-progs
# zypper in pam-devel
# zypper in poco-devel
# zypper in python3-polib

time (
    git pull -r
    # make distclean is broken
    if [ -e Makefile ]; then
        make clean
    fi

    # If sanitizers already set a CC/CXX, don't overwrite it.
    if [ -z "$CC" ]; then
        export CC="ccache clang"
    fi
    if [ -z "$CXX" ]; then
        export CXX="ccache clang++"
    fi
    ./autogen.sh \
        --prefix=$PWD/install \
        --enable-debug \
        --with-lo-path=$HOME/git/libreoffice/core/instdir \
        --with-lokit-path=$HOME/git/libreoffice/core/include \
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

exit ${PIPESTATUS[0]}

# vim:set shiftwidth=4 expandtab:
