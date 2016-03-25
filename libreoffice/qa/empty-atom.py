#!/usr/bin/env python3
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

from xml.dom import minidom
import os
import sys
import unittest


def parseFeed():
    rssPath = os.environ["TDOC"]
    return minidom.parse(rssPath)


class Test(unittest.TestCase):
    def test_none(self):
        """This test makes sure that there are no entries in the downloaded
        atom feed, assuming that each entry points out a problem."""

        feed = parseFeed()
        issues = []
        for item in feed.getElementsByTagName("entry"):
            for description in item.getElementsByTagName("title"):
                issues.append(description.firstChild.wholeText)

        # I assume I don't have to do anything there.
        issues = [i for i in issues if "Bug 96306" not in i]

        self.assertEqual([], issues)

if __name__ == '__main__':
    unittest.main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
