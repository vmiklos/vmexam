#!/usr/bin/env python3
#
# Copyright 2019 Miklos Vajna
#
# SPDX-License-Identifier: MIT
#

import unittest
import sys

suffix = ""


class Test(unittest.TestCase):
    def test_none(self):
        """This test makes sure that no addr:interpolation ways are used in the
        given area, as all addresses are described precisely instead."""

        self.maxDiff = None
        ways = []
        with open("workdir/streets-nomaxspeed%s.csv" % suffix) as sock:
            for line in sock.readlines():
                if line.startswith("@"):
                    continue
                tokens = line.strip().split('\t')
                if len(tokens) < 2 or (not len(tokens[0])) or (not len(tokens[1])):
                    continue
                ways.append(tokens[1])
            ways = sorted(set(ways))
            self.assertEqual([], ways)

if __name__ == '__main__':
    if len(sys.argv) > 1:
        suffix = sys.argv[1]
    unittest.main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
