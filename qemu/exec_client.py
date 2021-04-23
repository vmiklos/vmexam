#!/usr/bin/env python3
#
# Copyright 2021 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.
#

"""
This can run outside, on a host Linux machine:
- if you symlink the winword script to ~/bin, then it will work with native paths
- you can create similar wrapper scripts, e.g. s/winword/powerpnt/

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


def main():
    """Commandline interface to this module."""
    argv = sys.argv[1:]
    argv = [i.replace(os.path.join(os.environ["HOME"], "git"), 'z:') for i in argv]
    payload_dict = {
        "command": argv
    }
    payload = json.dumps(payload_dict)
    url = "http://192.168.122.216:8000/exec"
    with urllib.request.urlopen(url, bytes(payload, "utf-8")) as stream:
        buf = stream.read()
        print(str(buf, "utf-8"))


if __name__ == "__main__":
    main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
