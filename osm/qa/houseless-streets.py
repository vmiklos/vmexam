#!/usr/bin/env python3
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

import sys
import unittest


class Test(unittest.TestCase):
    def test_none(self):
        """This test makes sure that there are no streets without house numbers
        in the downloaded area."""
        sock = open("../workdir/streets.csv")
        first = True
        streets = []
        for line in sock.readlines():
            if first:
                first = False
                continue
            cols = line.strip().split("\t")
            if len(cols) < 2:
                continue
            streets.append(cols[1])
        streets = sorted(set(streets))
        sock.close()

        sock = open("../workdir/street-housenumbers.csv")
        first = True
        streetsWithHouses = []
        for line in sock.readlines():
            if first:
                first = False
                continue
            cols = line.strip().split("\t")
            if len(cols[1]):
                # Ignore house numbers outside this area (ideally we shouldn't even get these).
                if cols[1] in streets:
                    streetsWithHouses.append(cols[1])
            else:
                self.fail("'%s': house number without addr:street, please fix!" % cols[2])
        streetsWithHouses = sorted(set(streetsWithHouses))
        sock.close()

        streetsWithoutHouses = [street for street in streets if street not in streetsWithHouses]
        self.assertEqual(len(streets), len(streetsWithHouses) + len(streetsWithoutHouses))

        self.assertEqual([], streetsWithoutHouses)

if __name__ == '__main__':
    unittest.main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
