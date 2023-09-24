#!/bin/bash -ex

# git clone https://gitbox.apache.org/repos/asf/openoffice.git
# tested in a centos:centos7 rootless container, build breaks with a non-ancient toolchain
# last successfull commit: ce48dd1f26396c7ab9fed48d2df6a6b2bfcf6e06 (Maintenance cleanup, 2023-09-18)

time (
    git pull -r
    autoconf
    ./configure --disable-epm --without-junit --with-dmake-url=https://sourceforge.net/projects/oooextras.mirror/files/dmake-4.12.tar.bz2
    make -f Makefile clean
    rm -rf install
    make -f Makefile
    (. ./*Env.Set.sh && cd instsetoo_native/util; LOCALINSTALLDIR=$(pwd)/install dmake openoffice_en-US PKGFORMAT=installed)
    mv instsetoo_native/util/install/ .
) 2>&1 |tee log

# vim:set shiftwidth=4 expandtab:
