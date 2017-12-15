#!/bin/bash -ex
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

for object in $(cat .git/indented-files.cache .git/indented-files2.cache|grep cxx$)
do
    ~/git/include-what-you-use/iwyu_tool.py -p . $object 2>&1 | ~/git/vmexam/libreoffice/find-unneeded-includes2
done

# vim:set shiftwidth=4 softtabstop=4 expandtab:
