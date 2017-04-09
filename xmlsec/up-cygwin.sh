#!/bin/bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#
# This script is a wrapper around the libxmlsec nmake build system, so it works
# out of the box at least on my 64bit MSVC inside cygwin install.
#
# The 'Developer Command Prompt' is a good source of hints when some path
# changes.

msvcdir="c:/Program Files (x86)/Microsoft Visual Studio 14.0/VC"
xml2dir="c:/lo/master/workdir/UnpackedTarball/xml2"
icudir="c:/lo/master/workdir/UnpackedTarball/icu"
sdkdir="c:/Program Files (x86)/Windows Kits/8.1"
sdkdir10="c:/Program Files (x86)/Windows Kits/10"

incpath="$msvcdir/include"
libpath="$msvcdir/lib"

incpath+=";$xml2dir/include"
libpath+=";$xml2dir/win32/bin.msvc"

incpath+=";$icudir/source/common"

incpath+=";$sdkdir/Include/um"
incpath+=";$sdkdir/Include/shared"
libpath+=";$sdkdir/Lib/winv6.3/um/x86"

incpath+=";$sdkdir10/Include/10.0.10240.0/ucrt"
libpath+=";$sdkdir10/Lib/10.0.10240.0/ucrt/x86"

time sh -cex "git pull -r
    git clean -x -d -f
    export PATH='$(cygpath -u "$msvcdir/bin/"):$PATH'
    cd win32
    cscript configure.js crypto=mscrypto xslt=no iconv=no static=no debug=yes
    LIB='$libpath' INCLUDE='$incpath' '$msvcdir/bin/nmake.exe'
    cp $xml2dir/win32/bin.msvc/libxml2.dll binaries/
    cp $icudir/source/lib/icuuc58.dll binaries/
    cp $icudir/source/lib/icudt58.dll binaries/" 2>&1 |tee log

# vim:set shiftwidth=4 expandtab:
