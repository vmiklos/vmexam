#!/usr/bin/env python3
#
# Copyright 2019 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.
#

import sys
import unittest

suffix = ""


class Test(unittest.TestCase):
    def test_none(self):
        """This test makes sure that no addr:interpolation ways are used in the
        given area, as all addresses are described precisely instead."""

        ways = []
        sock = open("workdir/addr-interpolation%s.csv" % suffix)
        for line in sock.readlines():
            if line.startswith("@"):
                continue
            ways.append(line.strip())
        self.assertEqual([], ways)
        sock.close()

if __name__ == '__main__':
    if len(sys.argv) > 1:
        suffix = sys.argv[1]
        sys.argv = sys.argv[:1]
    unittest.main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
