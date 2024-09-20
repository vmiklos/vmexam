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
        export CC="ccache gcc-12"
    fi
    if [ -z "$CXX" ]; then
        export CXX="ccache g++-12"
    fi
    ./autogen.sh \
        --prefix=$PWD/install \
        --enable-debug \
        --enable-experimental \
        --enable-cypress \
        --with-lo-path=$HOME/git/libreoffice/co-24.04/instdir \
        --with-lokit-path=$HOME/git/libreoffice/co-24.04/include \
        CFLAGS="-g -O0 $CFLAGS" \
        CXXFLAGS="-g -O0 $CXXFLAGS" \

    # Self-built poco:
    #    --with-poco-includes=$HOME/git/poco/install/include \
    #    --with-poco-libs=$HOME/git/poco/lib/Linux/x86_64 \
    # #if ENABLE_SUPPORT_KEY:
    #    --with-support-public-key=$HOME/downloads/vmiklos.pem \

    make -j$(getconf _NPROCESSORS_ONLN)
    make ctags
    make -C test check
) 2>&1 |tee log

exit ${PIPESTATUS[0]}

# vim:set shiftwidth=4 expandtab:
