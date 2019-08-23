#!/bin/bash -ex
# Copyright 2019 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

cmake_args="-DCMAKE_INSTALL_PREFIX:PATH=$PWD/instdir"

for arg in "$@"
do
    case "$arg" in
        --asan-ubsan)
            export ASAN_OPTIONS=detect_stack_use_after_return=1
            export CC="clang -fsanitize=address -fsanitize=undefined"
            export CXX="clang++ -fsanitize=address -fsanitize=undefined"
            export CCACHE_CPP2=YES
            ;;
        --iwyu)
            cmake_args+=" -DOSMIFY_IWYU=ON"
            run_iwyu="y"
            ;;
        --tidy)
	    export CC=clang
	    export CXX=clang++
	    run_clang_tidy=run-clang-tidy
            export CCACHE_CPP2=1
            ;;
    esac
done

rm -rf workdir
rm -f compile_commands.json
mkdir workdir
cd workdir
cmake \
    $cmake_args \
    -DCMAKE_BUILD_TYPE=Debug \
    ..
if [ -n "$run_iwyu" ]; then
    make -j$(getconf _NPROCESSORS_ONLN) 2>&1 | tee log
    ! egrep 'should add|should remove' log || false
else
    make -j$(getconf _NPROCESSORS_ONLN)
fi
make install
cd ..
ln -s workdir/compile_commands.json .
if [ -n "$run_clang_tidy" ]; then
    # filter for tracked directories, i.e. implicitly filter out workdir
    directories="$(git ls-files|grep /|sed 's|/.*||'|sort -u|xargs echo|sed 's/ /|/g')"
    $run_clang_tidy -header-filter="^$PWD/(${directories})/.*"
fi
# Exclude workdir automatically.
ctags --c++-kinds=+p --fields=+iaS --extra=+q -R --totals=yes $(git ls-files|grep /|sed 's|/.*||'|sort -u)

# vim:set shiftwidth=4 softtabstop=4 expandtab:
