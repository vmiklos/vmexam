#!/bin/bash -ex
#
# Builds core.git from scratch.
#

time (
    git pull -r
    if [ -e Makefile ]; then
        make distclean
    fi
    ./autogen.sh
    make check gb_SUPPRESS_TESTS=y || make check gb_SUPPRESS_TESTS=y
    if [ "$(git config libreoffice.bibisect)" == "true" ]; then
        sh ~/git/vmexam/libreoffice/daily.sh
    fi
    make tags
    (cd instdir && rm -rf user && ln -s $HOME/.config/libreofficedev/master/user)
    make check
    make vim-ide-integration
    style-check-files
) 2>&1 |tee log

exit ${PIPESTATUS[0]}

# vim:set shiftwidth=4 expandtab:
