#!/bin/bash -e
#
# Builds core.git from scratch.
#

BRANCH=$(git symbolic-ref HEAD|sed 's|refs/heads/||')
time (
    git pull -r
    if [ $BRANCH == master -a -e Makefile ]; then
        make distclean
    fi
    ./autogen.sh
    make check gb_SUPPRESS_TESTS=y || make check gb_SUPPRESS_TESTS=y
    make tags
    # distro/foo/bar -> bar
    (cd instdir && rm -rf user && ln -s $HOME/.config/libreofficedev/${BRANCH##*/}/user)
    make check
    make vim-ide-integration
    if [ $BRANCH == master ]; then
        style-check-files
    fi
) 2>&1 |tee log

exit ${PIPESTATUS[0]}

# vim:set shiftwidth=4 expandtab:
