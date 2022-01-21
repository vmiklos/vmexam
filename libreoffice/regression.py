#!/usr/bin/env python3
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

from xml.dom import minidom
import urllib.parse
import urllib.request
import sys
import time


def get_regression_count(name):
    url = "https://bugs.documentfoundation.org/buglist.cgi?"
    params = {
        "f1": "longdesc",
        "f2": "keywords",
        "o1": "substring",
        "o2": "anywords",
        "query_format": "advanced",
        "resolution": "---",
        "v1": "Adding Cc: to " + name,
        "v2": "regression",
        "ctype": "atom",
    }
    url += urllib.parse.urlencode(params)
    with urllib.request.urlopen(url) as stream:
        atom = stream.read()
    feed = minidom.parseString(atom)
    return len(feed.getElementsByTagName("entry"))


def main(argv):
    with open(argv[0], "r") as stream:
        for name in stream.readlines():
            stripped_name = name.strip()
            regression_count = get_regression_count(stripped_name)
            time.sleep(1)
            print("{};{}".format(stripped_name, regression_count))


if __name__ == '__main__':
    main(sys.argv[1:])

# vim:set shiftwidth=4 softtabstop=4 expandtab:
