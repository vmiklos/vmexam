#!/usr/bin/env python3
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#
# Lists your outgoing gerrit changes, showing if the change is waiting in the
# CI queue. Useful as otherwise the web browser would show this info in a huge
# tooltip that usually blinks + frequently reloads.
#

from html.parser import HTMLParser
import json
import re
import subprocess
import urllib.request


def getGerritChanges():
    ret = []

    p = subprocess.Popen(["ssh", "logerrit", "gerrit", "query", "--format=json", "status:open project:core owner:self"], stdout=subprocess.PIPE)
    lines = p.communicate()[0].decode('utf-8').splitlines()
    for line in lines:
        j = json.loads(line)
        if "url" in j.keys():
            ret.append(j)

    return ret


def downloadString(url):
    response = urllib.request.urlopen(url)
    data = response.read()
    return data.decode('utf-8')


def getCIChanges(changesHTML):
    class MyHTMLParser(HTMLParser):
        def __init__(self):
            HTMLParser.__init__(self)
            self.changes = []

        def handle_starttag(self, tag, attrs):
            if tag == "a":
                for attrKey, attrValue in attrs:
                    if attrKey == "tooltip":
                        match = re.match(".*(https://gerrit.libreoffice.org/[0-9]+).*", attrValue)
                        if match:
                            self.changes.append(match.group(1))

    parser = MyHTMLParser()
    parser.feed(changesHTML)
    return sorted(set(parser.changes))

gerrit = getGerritChanges()

changesHTML = downloadString("https://ci.libreoffice.org/ajaxBuildQueue")
ci = getCIChanges(changesHTML)

for gerritChange in gerrit:
    line = gerritChange["url"] + " for " + gerritChange["branch"] + ": "
    if gerritChange["url"] in ci:
        line += "change is in the CI queue"
    else:
        line += "change is not in the CI queue"
    print(line)

# vim:set shiftwidth=4 softtabstop=4 expandtab:
