#!/usr/bin/env python3
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
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
