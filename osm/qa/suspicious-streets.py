#!/usr/bin/env python3
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

# What it does:
# Tries to find streets which do have at least one house number, but suspicious
# as lots of house numbers are probably missing.

import json
import re
import sys
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
        normalizers = {}
        filtersJson = normalizersJson["filters"]
        for street in filtersJson.keys():
            l = []
            for r in filtersJson[street]["ranges"]:
                l.append(Range(int(r["start"]), int(r["end"]), r["isOdd"] == "true"))
            normalizers[street] = lambda n: n in Ranges([l])
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
    with open("housenumber-filters.json") as jsonSock:
        normalizersJson = json.load(jsonSock)
    unittest.main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
