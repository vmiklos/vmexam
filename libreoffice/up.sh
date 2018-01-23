#!/bin/bash -ex
#
# The build uses ccache, so sometimes the build completes pretty quickly,
# sometimes it takes quite somt time. To run it exactly once a day, you can use
# something like:
#
# while true; do ~/git/vmexam/bash/sleep-until 04:00; ./up.sh; done
#

git pull -r
if [ -e Makefile ]; then
    make distclean
fi
./autogen.sh 2>&1 |tee log
make build-nocheck 2>&1 |tee -a log
if [ "$(git config libreoffice.bibisect)" == "true" ]; then
    sh ~/git/vmexam/libreoffice/daily.sh
fi
make tags
(cd instdir && rm -rf user && ln -s $HOME/.config/libreofficedev/master/user)
make check 2>&1 |tee -a log
make vim-ide-integration
style-check-files

# vim:set shiftwidth=4 expandtab:
