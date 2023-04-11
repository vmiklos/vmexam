#!/bin/bash -ex
# Copyright 2019 Miklos Vajna
#
# SPDX-License-Identifier: MIT

rm -rf workdir
mkdir workdir
cd workdir
cmake -DCMAKE_BUILD_TYPE=Debug -DOSMIFY_ENABLE_GCOV=ON ..
make -j$(getconf _NPROCESSORS_ONLN)
make check
cd -
lcov --directory workdir --capture --output-file osmify.info
lcov --remove osmify.info '/usr/*' --output-file osmify.info

# Network traffic is intentionally mocked.
lcov --remove osmify.info $PWD/urllib.cxx --output-file osmify.info

genhtml -o coverage osmify.info
echo "Coverage report is now available at coverage/index.html."

# Prints line coverage.
# lcov-1.14+git.20180307.a5dd952-lp152.3.2.noarch doesn't have --fail-under-lines yet.
~/git/lcov/bin/lcov --summary osmify.info --fail-under-lines 100

# vim:set shiftwidth=4 softtabstop=4 expandtab:
