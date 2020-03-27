#!/usr/bin/env python3
#
# Copyright 2020 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.
#

import json
import time
import sys


def main() -> None:
    """Commandline interface to this module."""
    with open("ref.count", "r") as stream:
        num_ref = int(stream.read().strip())
    today = time.strftime("%Y-%m-%d")
    with open("%s.count" % today, "r") as stream:
        num_osm = int(stream.read().strip())
    percentage = round(num_osm * 100 / num_ref, 2)
    j = {}
    j["date"] = today
    j["percentage"] = percentage
    j["reference"] = num_ref
    j["osm"] = num_osm
    print(json.dumps(j))


if __name__ == "__main__":
    main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
