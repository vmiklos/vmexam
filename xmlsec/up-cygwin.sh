#!/bin/bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#
# This script is a wrapper around the libxmlsec nmake build system, so it work
# out of the box at least on my 32bit MSVC install.

msvcdir="c:/Program Files/Microsoft Visual Studio 14.0/VC"
xml2dir="c:/lo/master/workdir/UnpackedTarball/xml2"
icudir="c:/lo/master/workdir/UnpackedTarball/icu"
sdkdir="c:/Program Files/Windows Kits/8.1"
sdkinc10="c:/Program Files/Windows Kits/10/Include/10.0.10240.0/ucrt"
sdklib10="c:/Program Files/Windows Kits/10/Lib/10.0.10240.0/ucrt/x86"

myinc="/I\"$msvcdir/include\""
myinc+=" /I\"$xml2dir/include\""
myinc+=" /I\"$icudir/source/common\""
myinc+=" /I\"$sdkdir/Include/um\""
myinc+=" /I\"$sdkdir/Include/shared\""
myinc+=" /I\"$sdkinc10\""

time sh -ce "git pull -r
    git clean -x -d -f
    export PATH='$(cygpath -u "$msvcdir/bin/"):$PATH'
    cd win32
    cscript configure.js crypto=mscrypto xslt=no iconv=no static=no debug=yes
    sed -i -e 's|/I\$(INCPREFIX)|/I\$(INCPREFIX) $myinc|' Makefile
    LIB='$xml2dir/win32/bin.msvc;$msvcdir/lib;$sdkdir/Lib/winv6.3/um/x86;$sdklib10' '$msvcdir/bin/nmake.exe'
    cp $xml2dir/win32/bin.msvc/libxml2.dll binaries/
    cp $icudir/source/lib/icuucd58.dll binaries/
    cp $icudir/source/lib/icudtd58.dll binaries/" 2>&1 |tee log

# vim:set shiftwidth=4 expandtab:
