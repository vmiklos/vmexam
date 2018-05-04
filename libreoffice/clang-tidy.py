#!/usr/bin/env python3
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

import multiprocessing
import os
import queue
import subprocess
import sys
import threading


def worker(q, failedFiles):
    while True:
        invocation = q.get()
        if invocation is None:
            break
        if not len(failedFiles):
            path = invocation[-1]
            print("[TIDY] " + path)
            retcode = subprocess.call(invocation)
            if retcode != 0:
                print("ERROR: '" + sys.argv[0] + " " + path + "' found warnings.")
                failedFiles.append(invocation)
        q.task_done()


def tidy(paths):
    headers = []

    fileLists = [".git/indented-files.cache", ".git/indented-files2.cache"]
    readPaths = False
    if not len(paths):
        readPaths = True

    for fileList in fileLists:
        sock = open(fileList)
        for fileLine in sock.readlines():
            path = fileLine.strip()
            if path.endswith(".hxx"):
                headers.append(os.path.abspath(path))
            if readPaths:
                if path.endswith(".cxx"):
                    # EPUBExportUIComponent.cxx: use-after-free false negative for ScopedVclPtrInstance
                    if not (("/qa/" in path) or ("EPUBExportUIComponent.cxx" in path)):
                        paths.append(path)

    headerFilter = "-header-filter=" + "|".join(headers)

    invocations = []
    for path in paths:
        invocation = ["clang-tidy", headerFilter, path]
        invocations.append(invocation)

    q = queue.Queue()
    threads = []
    numWorkerThreads = multiprocessing.cpu_count()
    failedFiles = []
    for i in range(numWorkerThreads):
        t = threading.Thread(target=worker, args=(q, failedFiles))
        t.start()
        threads.append(t)

    for invocation in invocations:
        q.put(invocation)

    # Block until all tasks are done.
    q.join()

    # Stop workers.
    for i in range(numWorkerThreads):
        q.put(None)
    for t in threads:
        t.join()

    returnCode = 0
    if len(failedFiles):
        returnCode = 1
    sys.exit(returnCode)


def main(argv):
    tidy(paths=argv)


if __name__ == '__main__':
    main(sys.argv[1:])

# vim:set shiftwidth=4 softtabstop=4 expandtab:
