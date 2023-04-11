#!/usr/bin/env python3
#
# Copyright 2019 Miklos Vajna
#
# SPDX-License-Identifier: MIT
#

import os
import unittest
from xml.dom import minidom


def parseFeed():
    rssPath = "workdir/osmose-%s.rss" % os.environ["USER"]
    return minidom.parse(rssPath)


def parseDescription(node):
    for line in node.firstChild.wholeText.split("\n"):
        if len(line):
            return line


class Test(unittest.TestCase):
    def test_none(self):
        """This test makes sure that there are no osmose issues assigned to
        the user at <http://osmose.openstreetmap.fr/en/byuser/>."""

        feed = parseFeed()
        issues = []
        for item in feed.getElementsByTagName("item"):
            for description in item.getElementsByTagName("description"):
                issues.append(parseDescription(description))
        self.assertEqual([], issues)


if __name__ == '__main__':
    unittest.main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
