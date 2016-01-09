#!/bin/sh

autoreconf --install --symlink
./configure "$@"

# vi:set shiftwidth=4 expandtab:
