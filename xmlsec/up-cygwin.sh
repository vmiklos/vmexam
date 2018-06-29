#!/bin/bash -ex
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
lodir="c:/lo/master"
xml2dir="$lodir/workdir/UnpackedTarball/xml2"
xsltdir="$lodir/workdir/UnpackedTarball/xslt"
icudir="$lodir/workdir/UnpackedTarball/icu"
sdkdir="c:/Program Files (x86)/Windows Kits/8.1"
sdkdir10="c:/Program Files (x86)/Windows Kits/10"

incpath="$msvcdir/include"
libpath="$msvcdir/lib"

incpath+=";$xml2dir/include"
libpath+=";$xml2dir/win32/bin.msvc"
incpath+=";$xsltdir"
libpath+=";$xsltdir/win32/bin.msvc"

incpath+=";$icudir/source/common"

incpath+=";$sdkdir/Include/um"
incpath+=";$sdkdir/Include/shared"
libpath+=";$sdkdir/Lib/winv6.3/um/x86"

incpath+=";$sdkdir10/Include/10.0.10240.0/ucrt"
libpath+=";$sdkdir10/Lib/10.0.10240.0/ucrt/x86"

git pull -r
git clean -x -d -f
export PATH="$(cygpath -u "$msvcdir/bin/")":"$PATH"
cd win32
cscript configure.js crypto=mscng iconv=no static=no debug=yes werror=yes
export LIB="$libpath"
export INCLUDE="$incpath"
nmake
cp $lodir/instdir/program/libxml2.dll binaries/
cp $lodir/instdir/program/libxslt.dll binaries/
cp $lodir/instdir/program/icuuc*.dll binaries/
cp $lodir/instdir/program/icudt*.dll binaries/

# vim:set shiftwidth=4 expandtab:
