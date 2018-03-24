#!/bin/bash -ex
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

# To debug:
# ~/git/include-what-you-use/iwyu_tool.py -p . $object 2>&1 |tee out
# cat out | ~/git/vmexam/libreoffice/find-unneeded-includes

if [ "$1" == "sw-inc" ]; then
    for object in $(git ls-files|grep sw/inc/.*.hxx |grep -v /pch/)
    do
        ~/git/include-what-you-use/iwyu_tool.py -p . -a sw/source/core/doc/docnew.cxx $object 2>&1 | ~/git/vmexam/libreoffice/find-unneeded-includes
    done
    exit
fi

for object in $(cat .git/indented-files.cache .git/indented-files2.cache|grep cxx$)
do
    ~/git/include-what-you-use/iwyu_tool.py -p . $object 2>&1 | ~/git/vmexam/libreoffice/find-unneeded-includes
done

# vim:set shiftwidth=4 softtabstop=4 expandtab:
