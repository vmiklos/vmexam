#!/bin/bash

# Dependencies:
# zypper in ccache
# zypper in libcap-devel
# zypper in libcap-progs
# zypper in pam-devel
# zypper in poco-devel
# zypper in python3-polib

# Log both everything to ./log as well.

SAN=
case "$CC $CFLAGS $CXXFLAGS" in
    *-fsanitize*) SAN=1 ;;
esac

# Build engine/.
time (
    set -e
    cd engine
    BRANCH=$(git symbolic-ref HEAD|sed 's|refs/heads/||')
    git pull -r
    if [ "$BRANCH" == main -a -e Makefile -a -z "$SAN" ]; then
        make distclean
    fi
    ./autogen.sh
    make check gb_SUPPRESS_TESTS=y || make check gb_SUPPRESS_TESTS=y
    make tags
    # distro/foo/bar -> bar
    (cd instdir && rm -rf user && ln -s $HOME/.config/collaboraofficedev/${BRANCH##*/}/user)
    sed -i 's|^UserInstallation=.*|UserInstallation=$ORIGIN/..|' instdir/program/bootstraprc
    if [ -z "$SAN" ]; then
        make check
    fi
    make vim-ide-integration
)
if [ $? -ne 0 ]; then
    exit 1
fi

# Build the rest.
time (
    set -e
    if [ -e Makefile ]; then
        make distclean
    fi
    ./autogen.py
    make -j$(getconf _NPROCESSORS_ONLN)
    make -j$(getconf _NPROCESSORS_ONLN) -C test check SUPPRESS_TESTS=y
    # exclude engine/ and non-tracked directories automatically
    ctags --c++-kinds=+p --fields=+iaS --extra=+q -R --totals=yes $(git ls-files|grep /|sed 's|/.*||'|grep -v engine|sort -u)
    # because we'll run 'make check' only in sub-directories
    make presets-dir
    make -C browser check
    kill-wrapper 'make -C test check' 1200
    # make -C cypress_test check-desktop
    # make -C cypress_test check-mobile
    # make -C cypress_test check-multi
)
if [ $? -ne 0 ]; then
    exit 2
fi

# vim:set shiftwidth=4 expandtab:
