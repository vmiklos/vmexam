#!/usr/bin/env python3
#
# Copyright 2021 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.
#

"""
This can run outside, on a host Linux machine:
- if you symlink the script script to e.g. ~/bin/winword, then it will inject the exe for you
- it'll try to map between the host and guest paths

You may want to customize:
- shared directory
- network drive letter
- guest IP
- guest port
"""

import json
import os
import sys
import urllib.request


ALIASES = {
    "acroread": "c:/program files (x86)/adobe/acrobat reader dc/reader/acrord32.exe",
    "excel": "c:/program files (x86)/microsoft office/root/office16/excel.exe",
    "powerpnt": "c:/program files (x86)/microsoft office/root/office16/powerpnt.exe",
    "winword": "c:/program files (x86)/microsoft office/root/office16/winword.exe",
}


def main() -> None:
    """Commandline interface to this module."""
    argv = sys.argv[1:]

    # If my name is an alias, inject it.
    my_name = sys.argv[0]
    command = ""
    for key, value in ALIASES.items():
        if not my_name.endswith(key):
            continue
        command = value
    if command:
        argv = [command] + argv

    # Map from host path to guest path.
    abs_argv = []
    for arg in argv:
        if os.path.exists(arg):
            abs_argv.append(os.path.abspath(arg))
        else:
            abs_argv.append(arg)
    abs_argv = [i.replace(os.path.join(os.environ["HOME"], "git"), 'z:') for i in abs_argv]

    payload_dict = {
        "command": abs_argv
    }
    payload = json.dumps(payload_dict)
    url = "http://192.168.122.216:8000/exec"
    with urllib.request.urlopen(url, bytes(payload, "utf-8")) as stream:
        buf = stream.read()
        print(str(buf, "utf-8"))


if __name__ == "__main__":
    main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
