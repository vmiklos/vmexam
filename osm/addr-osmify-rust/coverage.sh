#!/bin/bash -ex
# Copyright 2021 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

# Benefits of 'cargo tarpaulin':
# - does not require nightly-only -Zinstrument-coverage (see
# <https://github.com/rust-lang/rust/issues/79121> if it's still nightly-only)
# - does not require grcov
# - defines a way to exclude test code from coverage

cargo tarpaulin --out Lcov --target-dir $PWD/target-cov

genhtml -o coverage lcov.info
echo "Coverage report is now available at coverage/index.html."

# Prints line coverage.
# lcov-1.14+git.20180307.a5dd952-lp152.3.2.noarch doesn't have --fail-under-lines yet.
~/git/lcov/bin/lcov --summary lcov.info --fail-under-lines 100

# vim:set shiftwidth=4 softtabstop=4 expandtab:
