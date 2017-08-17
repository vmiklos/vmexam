#!/bin/bash -ex
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

header_filter=""
for header in $(grep hxx$ .git/indented-files.cache)
do
    if [ -n "$header_filter" ]; then
        header_filter+="|"
    fi
    header_filter+="$PWD/$header"
done

if [ -n "$1" ]; then
    clang-tidy -header-filter="$header_filter" $1
else
    for object in $(grep cxx$ .git/indented-files.cache)
    do
        clang-tidy -header-filter="$header_filter" $object
    done
fi

# vim:set shiftwidth=4 softtabstop=4 expandtab:
