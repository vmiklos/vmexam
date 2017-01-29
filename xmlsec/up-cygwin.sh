#!/bin/bash

XMLSEC_VERSION_MAJOR=$(grep ^XMLSEC_VERSION_MAJOR= configure.ac|sed 's/.*=//')
XMLSEC_VERSION_MINOR=$(grep ^XMLSEC_VERSION_MINOR= configure.ac|sed 's/.*=//')
XMLSEC_VERSION_SUBMINOR=$(grep ^XMLSEC_VERSION_SUBMINOR= configure.ac|sed 's/.*=//')
XMLSEC_VERSION="$XMLSEC_VERSION_MAJOR.$XMLSEC_VERSION_MINOR.$XMLSEC_VERSION_SUBMINOR"
XMLSEC_VERSION_INFO=`echo $XMLSEC_VERSION | awk -F. '{ printf "%d:%d:%d", $1+$2, $3, $2 }'`

msvcdir="c:/Program Files/Microsoft Visual Studio 12.0/VC"
xml2dir="c:/lo/master/workdir/UnpackedTarball/xml2"

myinc="/I\"$msvcdir/include\""
myinc+=" /I\"$xml2dir/include\""
myinc+=' /I"c:/lo/master/workdir/UnpackedTarball/icu/source/common"'
myinc+=' /I"c:/Program Files/Windows Kits/8.1/Include/um"'
myinc+=' /I"c:/Program Files/Windows Kits/8.1/Include/shared"'

time sh -ce "git pull -r
    git clean -x -d -f
    export PATH='$(cygpath -u "$msvcdir/bin/"):$PATH'
    cat include/xmlsec/version.h.in > include/xmlsec/version.h
    sed -i 's/@XMLSEC_VERSION_MAJOR@/${XMLSEC_VERSION_MAJOR}/' include/xmlsec/version.h
    sed -i 's/@XMLSEC_VERSION_MINOR@/${XMLSEC_VERSION_MINOR}/' include/xmlsec/version.h
    sed -i 's/@XMLSEC_VERSION_SUBMINOR@/${XMLSEC_VERSION_SUBMINOR}/' include/xmlsec/version.h
    sed -i 's/@XMLSEC_VERSION@/${XMLSEC_VERSION}/' include/xmlsec/version.h
    sed -i 's/@XMLSEC_VERSION_INFO@/${XMLSEC_VERSION_INFO}/' include/xmlsec/version.h
    cd win32
    cscript configure.js crypto=mscrypto xslt=no iconv=no static=no debug=yes
    sed -i -e 's|/I\$(INCPREFIX)|/I\$(INCPREFIX) $myinc|' Makefile
    LIB='$xml2dir/win32/bin.msvc;$msvcdir/lib;c:/Program Files/Windows Kits/8.1/Lib/winv6.3/um/x86' '$msvcdir/bin/nmake.exe'
    cp $xml2dir/win32/bin.msvc/libxml2.dll binaries/
    cp c:/lo/master/workdir/UnpackedTarball/icu/source/lib/icuuc56.dll binaries/
    cp c:/lo/master/workdir/UnpackedTarball/icu/source/lib/icudt56.dll binaries/" 2>&1 |tee log

# vim:set shiftwidth=4 expandtab:
