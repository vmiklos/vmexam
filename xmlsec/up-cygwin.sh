#!/bin/bash -ex
#
# Copyright 2019 Miklos Vajna
#
# SPDX-License-Identifier: MIT
#
# This script is a wrapper around the libxmlsec nmake build system, so it works
# out of the box at least on my 64bit MSVC inside cygwin install.
#
# The 'Developer Command Prompt' is a good source of hints when some path
# changes.

lodir="c:/lo/master"
xml2dir="$lodir/workdir/UnpackedTarball/libxml2"
xsltdir="$lodir/workdir/UnpackedTarball/libxslt"
icudir="$lodir/workdir/UnpackedTarball/icu"

incpath=""
libpath=""

msvcbin="c:/Program Files (x86)/Microsoft Visual Studio/2017/Community/VC/Tools/MSVC/14.14.26428/bin/Hostx86/x86/"

incpath+=";c:/Program Files (x86)/Windows Kits/10/Include/10.0.17134.0/shared"
incpath+=";c:/Program Files (x86)/Windows Kits/10/Include/10.0.17134.0/ucrt"
incpath+=";c:/Program Files (x86)/Windows Kits/10/Include/10.0.17134.0/um"
libpath+=";c:/Program Files (x86)/Windows Kits/10/Lib/10.0.17134.0/ucrt/x86"
libpath+=";c:/Program Files (x86)/Windows Kits/10/Lib/10.0.17134.0/um/x86"

incpath+=";c:/Program Files (x86)/Microsoft Visual Studio/2017/Community/VC/Tools/MSVC/14.14.26428/include"
libpath+=";c:/Program Files (x86)/Microsoft Visual Studio/2017/Community/VC/Tools/MSVC/14.14.26428/lib/x86/"

incpath+=";$xml2dir/include"
libpath+=";$xml2dir/win32/bin.msvc"
incpath+=";$xsltdir"
libpath+=";$xsltdir/win32/bin.msvc"
incpath+=";$icudir/source/common"

git pull -r
git clean -x -d -f
export PATH="$(cygpath -u "$msvcbin/")":"$PATH"
cd win32
# The cruntime= used for libxml2 and xmlsec should match.
cscript configure.js crypto=mscng iconv=no static=no debug=yes cruntime=/MDd werror=yes
export LIB="$libpath"
export INCLUDE="$incpath"
nmake
cp $lodir/instdir/program/libxml2.dll binaries/
cp $lodir/instdir/program/libxslt.dll binaries/
cp $lodir/instdir/program/icuuc*.dll binaries/
cp $lodir/instdir/program/icudt*.dll binaries/

# vim:set shiftwidth=4 expandtab:
