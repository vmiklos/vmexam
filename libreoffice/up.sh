#!/bin/bash
#
# The build uses ccache, so sometimes the build completes pretty quickly,
# sometimes it takes quite somt time. To run it exactly once a day, you can use
# something like:
#
# while true; do ~/git/vmexam/bash/sleep-until 04:00; . up.sh; done
#

time sh -c "git pull -r && \
    ./autogen.sh && \
    make clean && \
    make build-nocheck &&
    sh ~/git/vmexam/libreoffice/daily.sh && \
    make tags && \
    (cd instdir && rm -rf user && ln -s $HOME/.config/libreofficedev/4/user) && \
    make check &&
    style-check-files" 2>&1 |tee log

# vim:set shiftwidth=4 expandtab:
