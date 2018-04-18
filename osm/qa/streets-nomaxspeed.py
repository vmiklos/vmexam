#!/usr/bin/env python3
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
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
