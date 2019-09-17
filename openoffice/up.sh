#!/bin/bash -ex

# git clone https://gitbox.apache.org/repos/asf/openoffice.git
# tested in a Ubuntu 14.04 chroot, build breaks with a non-ancient toolchain
# last successfull commit: b2b6b138749e974ca2c6396b5ae0a0b78591c7d4 (Fixed typos, removed whitespace, 2019-09-16)

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
