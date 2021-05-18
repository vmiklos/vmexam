#!/bin/bash -ex
# Copyright 2021 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

# 'cargo tarpaulin' can be replaced by -Zinstrument-coverage once it's no longer nightly-only:
# https://www.collabora.com/news-and-blog/blog/2021/03/24/rust-integrating-llvm-source-base-code-coverage-with-gitlab/

cargo tarpaulin --out Lcov

# Network traffic is intentionally mocked.
lcov --remove lcov.info $PWD/src/main.rs --output-file lcov.info

genhtml -o coverage lcov.info
echo "Coverage report is now available at coverage/index.html."

# Prints line coverage.
# lcov-1.14+git.20180307.a5dd952-lp152.3.2.noarch doesn't have --fail-under-lines yet.
~/git/lcov/bin/lcov --summary lcov.info --fail-under-lines 100

# vim:set shiftwidth=4 softtabstop=4 expandtab:
