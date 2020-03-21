#!/usr/bin/env python3
#
# Copyright 2020 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.
#

import sys


def main() -> None:
    """Commandline interface to this module."""
    stream = open(sys.argv[1])
    first = True
    counter = 0
    for line in stream.readlines():
        if first:
            first = False
            continue
        tokens = line.strip().split('\t')
        if len(tokens) < 3:
            continue
        # Ignore if id, street or housenumber is missing.
        if not tokens[0] or not tokens[1] or not tokens[2]:
            continue
        counter += 1
    print(counter)


if __name__ == "__main__":
    main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
