#!/usr/bin/env python3
#
# Copyright 2019 Miklos Vajna
#
# SPDX-License-Identifier: MIT
#

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
            # No idea why this shows up, manually checking it seems it has house numbers.
            # The 3rd doesn't seem to have odd house numbers, so not tagged, but nothing to improve, either.
            # 4th is only a small part of the whole way in case of sashegy
            # filtering, and this part in fact doesn't have house numbers.
            if not (cols[1] in ("Barackmag utca", "Higany utca", "Gazdagréti út", "Harasztos út")):
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
                self.warnings.append("WARNING: '%s': house number without addr:street, please fix! (id=%s)" % (cols[2], cols[0]))
        self.streetsWithHouses = sorted(set(self.streetsWithHouses))
        sock.close()

        self.streetsWithoutHouses = [street for street in self.streets if street not in self.streetsWithHouses]
        assert len(self.streets) == len(self.streetsWithHouses) + len(self.streetsWithoutHouses)


class Test(unittest.TestCase):
    def test_none(self):
        """This test makes sure that there are no streets without house numbers
        in the downloaded area."""
        finder = Finder()

        self.maxDiff = None
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
