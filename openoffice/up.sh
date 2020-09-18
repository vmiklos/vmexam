#!/bin/bash -ex

# git clone https://gitbox.apache.org/repos/asf/openoffice.git
# tested in a Ubuntu 14.04 chroot, build breaks with a non-ancient toolchain
# last successfull commit: 01d5ed08e552202ec8cdea6b6076e44a413a135b (Updated Asturian dictionary, 2020-09-18)

time (
    git pull -r
    autoconf
    ./configure --disable-epm -with-dmake-url=https://sourceforge.net/projects/oooextras.mirror/files/dmake-4.12.tar.bz2
    make -f Makefile clean
    rm -rf install
    make -f Makefile
    (. ./*Env.Set.sh && cd instsetoo_native/util; LOCALINSTALLDIR=$(pwd)/install dmake openoffice_en-US PKGFORMAT=installed)
    mv instsetoo_native/util/install/ .
) 2>&1 |tee log

# vim:set shiftwidth=4 expandtab:
