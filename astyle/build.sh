#!/bin/bash -xe
# git svn clone https://svn.code.sf.net/p/astyle/code/trunk/AStyle astyle
# invoke as ~/git/astyle/build/gcc/bin/astyle
cd ~/git/astyle
git reset --hard
git svn rebase
git clean -x -d -f
cd build/gcc
make -j8
