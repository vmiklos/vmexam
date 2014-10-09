#!/bin/bash -xe
# git clone git://sourceware.org/git/binutils-gdb.git
# invoke as ~/git/binutils-gdb/install/bin/gdb
cd ~/git/binutils-gdb
git reset --hard
git pull -r
git clean -x -d -f
CFLAGS="-g -O2" ./configure --prefix=$(pwd)/install
make -j8
make install
