#!/bin/bash -ex
#
# Copyright 2020 Miklos Vajna
#
# SPDX-License-Identifier: MIT
#

#
# This script runs all the tests for CI purposes.
#

if [ -n "${GITHUB_WORKFLOW}" ]; then
    sudo apt install hyphen-hu
    sudo apt install libhyphen-dev
    sudo apt install libssl-dev
    sudo apt install libgexiv2-dev

    # Build from source: cargo install --version 0.4.6 cargo-llvm-cov
    # Binary install:
    wget https://github.com/cargo-bins/cargo-binstall/releases/download/v0.17.0/cargo-binstall-x86_64-unknown-linux-gnu.tgz
    tar -xvf cargo-binstall-x86_64-unknown-linux-gnu.tgz
    ./cargo-binstall --no-confirm --version 0.4.6 cargo-llvm-cov
fi
make -j$(getconf _NPROCESSORS_ONLN) check

# vim:set shiftwidth=4 softtabstop=4 expandtab:
