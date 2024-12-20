#!/bin/bash -ex

# git clone https://gitbox.apache.org/repos/asf/openoffice.git
# tested in a centos:centos7 rootless container, build breaks with a non-ancient toolchain
# last successful commit: 64d8ff4dd8a1bddaf9f319bbeb82e1132f0035ee (Cleanup AutoText (SK), 2024-12-19)

time (
    git pull -r
    autoconf
    ./configure --disable-epm --without-junit --with-dmake-url=https://sourceforge.net/projects/oooextras.mirror/files/dmake-4.12.tar.bz2 --disable-odk
    make -f Makefile clean
    rm -rf install
    make -f Makefile
    (. ./*Env.Set.sh && cd instsetoo_native/util; LOCALINSTALLDIR=$(pwd)/install dmake openoffice_en-US PKGFORMAT=installed)
    mv instsetoo_native/util/install/ .
) 2>&1 |tee log

# vim:set shiftwidth=4 expandtab:
