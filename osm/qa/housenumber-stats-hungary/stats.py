#!/usr/bin/env python3
#
# Copyright 2020 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.
#

import json
import time
import sys


def handle_progress(j):
    """Generates stats for a global progressbar."""
    ret = {}
    with open("ref.count", "r") as stream:
        num_ref = int(stream.read().strip())
    today = time.strftime("%Y-%m-%d")
    with open("%s.count" % today, "r") as stream:
        num_osm = int(stream.read().strip())
    percentage = round(num_osm * 100 / num_ref, 2)
    ret["date"] = today
    ret["percentage"] = percentage
    ret["reference"] = num_ref
    ret["osm"] = num_osm
    j["progress"] = ret

def handle_daily_new(j):
    """Shows # of new housenumbers / day."""
    day_in_sec = 86400
    ret = []
    prev_count = 0
    prev_day = ""
    for day_offset in range(3, -1, -1):
        day_delta = day_offset * day_in_sec
        day_timestamp = time.time() - day_delta
        day_tuple = time.localtime(day_timestamp)
        day = time.strftime("%Y-%m-%d", day_tuple)
        with open("%s.count" % day, "r") as stream:
            count = int(stream.read().strip())
        if prev_count:
            ret.append([prev_day, count - prev_count])
        prev_count = count
        prev_day = day
    j["daily"] = ret

def main() -> None:
    """Commandline interface to this module."""
    j = {}
    handle_progress(j)
    handle_daily_new(j)
    print(json.dumps(j))


if __name__ == "__main__":
    main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
