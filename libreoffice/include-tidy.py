#!/usr/bin/env python3
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

import glob
import os
import subprocess
import sys


def getIndentedPaths():
    ret = []

    for fileList in [".git/indented-files.cache", ".git/indented-files2.cache"]:
        sock = open(fileList)
        for line in sock.readlines():
            path = line.strip()
            if not path.endswith("cxx"):
                continue

            ret.append(path)

    return ret


def tidy(paths, assume=None):
    for path in paths:
        iwyu_tool = os.path.join(os.environ["HOME"], "git/include-what-you-use/iwyu_tool.py")
        find_unneeded_includes = os.path.join(os.environ["HOME"], "git/vmexam/libreoffice/find-unneeded-includes")

        invocation1 = [iwyu_tool, "-p", "."]
        if assume is not None:
            invocation1 += ["-a", assume]
        invocation1 += [path]
        invocation2 = [find_unneeded_includes]
        print(" ".join(invocation1) + " | " + " ".join(invocation2))
        p1 = subprocess.Popen(invocation1, stdout=subprocess.PIPE)
        p2 = subprocess.Popen(invocation2, stdin=p1.stdout, stdout=subprocess.PIPE)
        p1.stdout.close()
        sys.stdout.write(p2.communicate()[0].decode('utf-8'))
        if p2.returncode != 0:
            break

if __name__ == '__main__':
    area = None
    if len(sys.argv) > 1:
        area = sys.argv[1]
    if area == "sw-inc":
        tidy(glob.glob("sw/inc/*.hxx"), "sw/source/core/doc/docnew.cxx")
    else:
        tidy(getIndentedPaths())

# vim:set shiftwidth=4 softtabstop=4 expandtab:
