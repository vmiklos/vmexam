#!/bin/bash -xe
# git clone git://sourceware.org/git/binutils-gdb.git
# invoke as ~/git/binutils-gdb/install/bin/gdb
cd ~/git/binutils-gdb
git reset --hard
git pull -r
git clean -x -d -f
CFLAGS="-g -O2" ./configure --prefix=$(pwd)/install --with-python=/usr/bin/python3
make -j8
make install
# enable e.g. STL pretty-printers
cd install/share/gdb
ln -s /usr/share/gdb/auto-load
