#!/usr/bin/env python3
#
# Copyright 2019 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.
#

"""Finds my old + experimental APIs."""

import subprocess
import time


def main() -> None:
  """Commandline interface to this module."""
  apis_bytes = subprocess.check_output(["git", "grep", "-n", "Experimental API", "public/"])
  apis = apis_bytes.decode("utf-8").strip().split("\n")
  author_date_loc = []
  for api in apis:
    tokens = api.split(":")
    path = tokens[0]
    line_num = tokens[1]
    blame_bytes = subprocess.check_output(["git", "blame", "--porcelain", "-L", line_num + "," + line_num, path])
    blame_lines = blame_bytes.decode("utf-8").strip().split("\n")
    date = 0
    author = ""
    for line in blame_lines:
      if line.startswith("author-time"):
        tokens = line.split(" ")
        date = int(tokens[1])
      elif line.startswith("author "):
        tokens = line.split(" ")
        author = tokens[1]
    author_date_loc.append((author, date, path + ":" + line_num))
  author_date_loc = sorted(author_date_loc, key=lambda x: x[1])
  today = time.time()
  for author, date, loc in author_date_loc:
    if author != "Miklos":
      continue
    # Year in seconds.
    if date >= today - 3 * 31536000:
      continue
    parsed_date = time.localtime(date)
    date_string = time.strftime("%Y-%m-%d", parsed_date)
    print("date: '"+date_string+"', loc: "+loc+"")


if __name__ == "__main__":
  main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
