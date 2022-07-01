#!/bin/bash
#
# Copyright 2020 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.
#

# I run this as ~/git/email/mirror.sh.

export PATH=$HOME/bin:$PATH

cd $(dirname $0)
while true
do
    for repo in ged2dot osm-gimmisn plees-tracker odfsig
    do
        cd $repo-mirror.git
        git fetch -q github
        git push -q email
        cd - >/dev/null
    done
    sleep 60
done

# vim:set shiftwidth=4 softtabstop=4 expandtab:
