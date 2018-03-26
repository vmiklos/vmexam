#!/usr/bin/env python3
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

import glob
import json
import multiprocessing
import os
import queue
import subprocess
import sys
import threading


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


def run_tool(assume, task_queue, failed_files):
    while True:
        invocation1, invocation2 = task_queue.get()
        if not len(failed_files):
            print(invocation1 + " | " + " ".join(invocation2))
            p1 = subprocess.Popen(invocation1, shell=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
            p2 = subprocess.Popen(invocation2, stdin=p1.stdout, stdout=subprocess.PIPE)
            p1.stdout.close()
            sys.stdout.write(p2.communicate()[0].decode('utf-8'))
            if p2.returncode != 0:
                failed_files.append(invocation1)
        task_queue.task_done()


def tidy(compileCommands, paths, assume=None):
    return_code = 0
    if assume:
        assumeAbs = os.path.abspath(assume)
    try:
        max_task = multiprocessing.cpu_count()
        task_queue = queue.Queue(max_task)
        failed_files = []
        for _ in range(max_task):
            t = threading.Thread(target=run_tool, args=(assume, task_queue, failed_files))
            t.daemon = True
            t.start()

        for path in sorted(paths):
            pathAbs = os.path.abspath(path)
            if assume:
                compileFile = assumeAbs
            else:
                compileFile = pathAbs
            matches = [i for i in compileCommands if i["file"] == compileFile]
            if not len(matches):
                print("WARNING: no compile commands for '" + path + "'")
                continue

            _, _, args = matches[0]["command"].partition(" ")
            if assume:
                args = args.replace(assumeAbs, "-x c++ " + pathAbs)
            find_unneeded_includes = os.path.join(os.path.dirname(__file__), "find-unneeded-includes")

            invocation1 = "include-what-you-use " + args
            invocation2 = [find_unneeded_includes]
            task_queue.put((invocation1, invocation2))

        task_queue.join()
        if len(failed_files):
            return_code = 1

    except KeyboardInterrupt:
        print('\nCtrl-C detected, goodbye.')
        os.kill(0, 9)

    sys.exit(return_code)

if __name__ == '__main__':
    with open("compile_commands.json", 'r') as compileCommandsSock:
        compileCommands = json.load(compileCommandsSock)

    area = None
    if len(sys.argv) > 1:
        area = sys.argv[1]
    if area == "sw-inc":
        tidy(compileCommands, glob.glob("sw/inc/*.hxx"), "sw/source/core/doc/docnew.cxx")
    else:
        tidy(compileCommands, getIndentedPaths())

# vim:set shiftwidth=4 softtabstop=4 expandtab:
