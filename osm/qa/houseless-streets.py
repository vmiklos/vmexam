#!/usr/bin/env python3
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

import os
import sys
import unittest

suffix = ""


class Finder:
    def __init__(self):
        sock = open("workdir/streets%s.csv" % suffix)
        first = True
        self.streets = []
        for line in sock.readlines():
            if first:
                first = False
                continue
            cols = line.strip().split("\t")
            if len(cols) < 2:
                continue
            # No idea why this shows up, manually checking it seems it's outside the area _and_ it has house numbers, too.
            if cols[1] != "Barackmag utca":
                self.streets.append(cols[1])
        self.streets = sorted(set(self.streets))
        sock.close()

        sock = open("workdir/street-housenumbers%s.csv" % suffix)
        first = True
        self.streetsWithHouses = []
        self.warnings = []
        for line in sock.readlines():
            if first:
                first = False
                continue
            cols = line.strip().split("\t")
            if len(cols[1]):
                # Ignore house numbers outside this area (ideally we shouldn't even get these).
                if cols[1] in self.streets:
                    self.streetsWithHouses.append(cols[1])
            else:
                self.warnings.append("WARNING: '%s': house number without addr:street, please fix!" % cols[2])
        self.streetsWithHouses = sorted(set(self.streetsWithHouses))
        sock.close()

        self.streetsWithoutHouses = [street for street in self.streets if street not in self.streetsWithHouses]
        assert len(self.streets) == len(self.streetsWithHouses) + len(self.streetsWithoutHouses)


class Test(unittest.TestCase):
    def test_none(self):
        """This test makes sure that there are no streets without house numbers
        in the downloaded area."""
        finder = Finder()

        self.assertEqual([], finder.warnings)
        self.assertEqual([], finder.streetsWithoutHouses)

if __name__ == '__main__':
    if "-s" in sys.argv:
        finder = Finder()
        print("%s streets in total." % len(finder.streets))
        print("%s streets have at least one house number." % len(finder.streetsWithHouses))
        print("%s streets have no house number." % len(finder.streetsWithoutHouses))
        print("Coverage is %s%%." % round(float(len(finder.streetsWithHouses)) * 100 / len(finder.streets)))
    else:
        if len(sys.argv) > 1:
            suffix = sys.argv[1]
            sys.argv = sys.argv[:1]
        unittest.main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
