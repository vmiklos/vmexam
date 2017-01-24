#!/usr/bin/env python3
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

# What it does:
# Tries to find streets which do have at least one house number, but suspicious
# as lots of house numbers are probably missing.

import sys
import re
import unittest


# A Ranges object contains an item if any of its Range objects contains it.
class Ranges:
    def __init__(self, items):
        self.items = items

    def __contains__(self, item):
        for i in self.items:
            if item in i:
                return True
        return False


# A range object represents an odd or even range of integer numbers.
class Range:
    def __init__(self, start, end, isOdd):
        self.start = start
        self.end = end
        self.isOdd = isOdd

    def __contains__(self, n):
        if self.isOdd != (n % 2 == 1):
            return False
        if n >= self.start and n <= self.end:
            return True
        return False


def getArea():
    if len(sys.argv) > 1:
        return sys.argv[1]
    else:
        return str()


def simplify(s):
    """ Handles normalization of a street name."""
    s = s.replace('Á', 'A').replace('á', 'a')
    s = s.replace('É', 'E').replace('é', 'e')
    s = s.replace('Í', 'I').replace('í', 'i')
    s = s.replace('Ó', 'O').replace('ó', 'o')
    s = s.replace('Ö', 'O').replace('ö', 'o')
    s = s.replace('Ő', 'O').replace('ő', 'o')
    s = s.replace('Ú', 'U').replace('ú', 'u')
    s = s.replace('Ü', 'U').replace('ü', 'u')
    s = s.replace('Ű', 'U').replace('ű', 'u')
    s = s.replace(' ', '_').lower()
    return s


def normalize(houseNumber, streetName):
    """Strips down string input to bare minimum that can be interpreted as an
    actual number. Think about a/b, a-b, and so on."""
    n = int(re.sub(r"([0-9]+).*", r"\1", houseNumber))
    normalizers = {
        # 1 -> 15, 2 -> 24
        "zajzon_utca": lambda n: n <= 24,
        # 1 -> 47, 51 -> 61, 2 -> 44, 50 -> 58
        "pannonhalmi_ut": lambda n: n in Ranges([Range(1, 47, isOdd=True),
                                                 Range(51, 61, isOdd=True),
                                                 Range(2, 44, isOdd=False),
                                                 Range(50, 58, isOdd=False)]),
    }
    if streetName in normalizers.keys():
        if not normalizers[streetName](n):
            return None
    return str(n)


def getHouseNumbersFromCsv(streetName):
    houseNumbers = []
    streetHouseNumbersSock = open("workdir/street-housenumbers%s.csv" % getArea())
    first = True
    for line in streetHouseNumbersSock.readlines():
        if first:
            first = False
            continue
        tokens = line.strip().split('\t')
        if len(tokens) < 3:
            continue
        if tokens[1] != streetName:
            continue
        houseNumber = normalize(tokens[2], simplify(streetName))
        if houseNumber:
            houseNumbers.append(houseNumber)
    streetHouseNumbersSock.close()
    return sorted(set(houseNumbers))


def getHouseNumbersFromLst(streetName):
    houseNumbers = []
    lstStreetName = simplify(streetName)
    prefix = lstStreetName + "_"
    sock = open("workdir/street-housenumbers-reference.lst")
    for line in sock.readlines():
        line = line.strip()
        if line.startswith(prefix):
            houseNumber = normalize(line.replace(prefix, ''), lstStreetName)
            if houseNumber:
                houseNumbers.append(houseNumber)
    sock.close()
    return sorted(set(houseNumbers))


def getOnlyInFirst(first, second):
    ret = []
    for i in first:
        if i not in second:
            ret.append(i)
    return ret


class Finder:
    def __init__(self):
        streetsSock = open("workdir/streets%s.csv" % getArea())
        streetNames = []
        firstStreet = True
        for streetLine in streetsSock.readlines():
            if firstStreet:
                firstStreet = False
                continue
            streetTokens = streetLine.strip().split('\t')
            if len(streetTokens) > 1:
                streetNames.append(streetTokens[1])
        streetsSock.close()
        streetNames = sorted(set(streetNames))

        results = []

        for streetName in streetNames:
            referenceHouseNumbers = getHouseNumbersFromLst(streetName)
            osmHouseNumbers = getHouseNumbersFromCsv(streetName)
            onlyInReference = getOnlyInFirst(referenceHouseNumbers, osmHouseNumbers)
            results.append((streetName, onlyInReference))

        # Sort by length.
        results.sort(key=lambda result: len(result[1]), reverse=True)

        for result in results:
            if len(result[1]):
                # House number, # of onlyInReference items.
                print("%s\t%s" % (result[0], len(result[1])))
                # onlyInReference items.
                print(result[1])

        self.suspiciousStreets = results


class Test(unittest.TestCase):
    def test_none(self):
        finder = Finder()

        self.assertEqual([], finder.suspiciousStreets)

if __name__ == '__main__':
    unittest.main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
