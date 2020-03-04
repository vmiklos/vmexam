#!/bin/bash -ex
#
# The build uses ccache, so sometimes the build completes pretty quickly,
# sometimes it takes quite some time. To run it exactly once a day, you can use
# something like:
#
# while true; do ~/git/vmexam/bash/sleep-until 04:00; ./up.sh; done
#

time (
    # This can act as a gate, only pull in changes in case $rebaseRemote built them successfully
    # already.
    rebaseRemote=$(git config libreoffice.rebaseRemote || true)
    if [ -z "$rebaseRemote" ]; then
        # Optimistic: all changes passed CI anyway.
        git pull -r
    else
        # Pessimistic: only update once 'make check' already passed in a sandbox locally.
        git fetch "$rebaseRemote"
        git rebase "$rebaseRemote"/master
    fi

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

# vim:set shiftwidth=4 expandtab:
