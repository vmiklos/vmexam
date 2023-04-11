#!/bin/bash
#
# Copyright 2020 Miklos Vajna
#
# SPDX-License-Identifier: MIT
#

# I run this as ~/git/email/mirror.sh.

export PATH=$HOME/bin:$PATH

cd $(dirname $0)
while true
do
    for repo in ged2dot osm-gimmisn plees-tracker odfsig turtle-cpm
    do
        cd $repo-mirror.git
        git fetch -q github
        git push -q email
        cd - >/dev/null
    done
    sleep 60
done

# vim:set shiftwidth=4 softtabstop=4 expandtab:
