#!/usr/bin/env python3
#
# Copyright 2022 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.
#
# Takes a HTML output from llvm-cov and shows uncovered lines.
#
# Usage example:
# $HOME/git/vmexam/llvm/coverage_show_missing.py target/llvm-cov/html/coverage$PWD/src/cron.rs.html

from html.parser import HTMLParser
import os
import sys

class Parser(HTMLParser):
    def __init__(self):
        HTMLParser.__init__(self)
        self.in_line_number = False
        self.line_number = ""
        self.in_uncovered_line = False
        self.uncovered_line = ""
        self.uncovered_lines = []

    def handle_starttag(self, tag, attrs):
        if tag == "td":
            for k, v in attrs:
                if k == "class" and v == "line-number":
                    self.in_line_number = True
                    self.line_number = ""
                elif k == "class" and v == "uncovered-line":
                    self.in_uncovered_line = True
                    self.uncovered_line = ""

    def handle_endtag(self, tag):
        if self.in_line_number:
            self.in_line_number = False
        elif self.in_uncovered_line:
            self.in_uncovered_line = False
            if self.uncovered_line == "0":
                self.uncovered_lines.append(self.line_number)

    def handle_data(self, data):
        if self.in_line_number:
            self.line_number += data
        elif self.in_uncovered_line:
            self.uncovered_line += data

def show_missing_for_path(path) -> None:
    relpath = path
    relpath = relpath.replace(os.getcwd(), "")
    relpath = relpath.replace("target/llvm-cov/html/coverage/", "")
    relpath = relpath.replace(".rs.html", ".rs")
    with open(path) as stream:
        parser = Parser()
        parser.feed(stream.read())
        for uncovered_line in sorted(set(parser.uncovered_lines)):
            print("{}:{}".format(relpath, uncovered_line))


if __name__ == "__main__":
    root = "target/llvm-cov/html/coverage/"
    if len(sys.argv) > 1:
        paths = sys.argv[1:]
    else:
        paths = sorted(set([os.path.join(dp, f) for dp, dn, filenames in os.walk(root) for f in filenames if f.endswith(".rs.html")]))
    for path in paths:
        show_missing_for_path(path)

# vim:set shiftwidth=4 softtabstop=4 expandtab:
