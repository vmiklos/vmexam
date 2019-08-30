#!/bin/bash -ex
# Copyright 2019 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

rm -rf workdir
mkdir workdir
cd workdir
cmake -DCMAKE_BUILD_TYPE=Debug -DOSMIFY_ENABLE_GCOV=ON ..
make -j$(getconf _NPROCESSORS_ONLN)
make check
cd -
lcov --directory workdir --capture --output-file osmify.info
lcov --remove osmify.info '/usr/*' --output-file osmify.info
genhtml -o coverage osmify.info
echo "Coverage report is now available at coverage/index.html."

# vim:set shiftwidth=4 softtabstop=4 expandtab:
