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

    ./autogen.py
    make -j$(getconf _NPROCESSORS_ONLN)
    make -j$(getconf _NPROCESSORS_ONLN) -C test check SUPPRESS_TESTS=y
    make ctags
    kill-wrapper 'make -C test check' 1200
    # make -C cypress_test check-desktop
    # make -C cypress_test check-mobile
    # make -C cypress_test check-multi
) 2>&1 |tee log

exit ${PIPESTATUS[0]}

# vim:set shiftwidth=4 expandtab:
