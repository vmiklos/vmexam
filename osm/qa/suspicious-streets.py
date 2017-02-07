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


def normalize(houseNumbers, streetName):
    """Strips down string input to bare minimum that can be interpreted as an
    actual number. Think about a/b, a-b, and so on."""
    ret = []
    for houseNumber in houseNumbers.split('-'):
        try:
            n = int(re.sub(r"([0-9]+).*", r"\1", houseNumber))
        except ValueError:
            continue
        normalizers = {
            # Source for this data: survey.
            "zajzon_utca": lambda n: n <= 24,
            "pannonhalmi_ut": lambda n: n in Ranges([Range(1, 47, isOdd=True),
                                                     Range(51, 61, isOdd=True),
                                                     Range(2, 44, isOdd=False),
                                                     Range(50, 58, isOdd=False)]),
            "bodajk_utca": lambda n: n in Ranges([Range(1, 37, isOdd=True),
                                                  Range(2, 32, isOdd=False)]),
            "orseg_utca": lambda n: n in Ranges([Range(1, 29, isOdd=True),
                                                 Range(2, 40, isOdd=False)]),
            "sasadi_koz": lambda n: n in Ranges([Range(1, 1, isOdd=True),
                                                 Range(2, 4, isOdd=False)]),
            "eper_utca": lambda n: n in Ranges([Range(1, 77, isOdd=True),
                                                Range(2, 56, isOdd=False)]),
            "botfalu_koz": lambda n: n in Ranges([Range(1, 23, isOdd=True),
                                                  Range(2, 20, isOdd=False)]),
            "nagyida_koz": lambda n: n in Ranges([Range(1, 21, isOdd=True),
                                                  Range(2, 18, isOdd=False)]),
            "nagyszalonta_utca": lambda n: n in Ranges([Range(1, 57, isOdd=True),
                                                        Range(2, 68, isOdd=False)]),
            "ratkoc_koz": lambda n: n in Ranges([Range(1, 5, isOdd=True),
                                                 Range(2, 8, isOdd=False)]),
            # Not sure about the 4.
            "szent_kristof_utca": lambda n: n in Ranges([Range(1, 3, isOdd=True),
                                                         Range(2, 4, isOdd=False)]),
            # Not sure about the 12.
            "rozsato_utca": lambda n: n in Ranges([Range(1, 9, isOdd=True),
                                                         Range(2, 12, isOdd=False)]),
            "olt_utca": lambda n: n in Ranges([Range(1, 41, isOdd=True),
                                               Range(2, 40, isOdd=False)]),
            # Not sure about the 16.
            "rozmaring_utca": lambda n: n in Ranges([Range(3, 19, isOdd=True),
                                                     Range(2, 16, isOdd=False)]),
            # 13 could be 21, but it's in fact 13-21.
            "homonna_utca": lambda n: n in Ranges([Range(1, 13, isOdd=True),
                                                   Range(2, 20, isOdd=False)]),
        }
        if streetName in normalizers.keys():
            if not normalizers[streetName](n):
                continue
        ret.append(str(n))
    return ret


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
        houseNumbers += normalize(tokens[2], simplify(streetName))
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
            houseNumbers += normalize(line.replace(prefix, ''), lstStreetName)
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
            if len(onlyInReference):
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
