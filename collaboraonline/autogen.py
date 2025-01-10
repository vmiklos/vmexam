#!/usr/bin/env python3

import os
import re
import subprocess

def main():
    cmdline = ["./autogen.sh"]
    with open("autogen.input") as stream:
        for line in stream.readlines():
            line = line.strip()
            if line.startswith("#"):
                continue
            line = os.path.expandvars(line)
            line = re.sub("\$[A-Za-z_][A-Za-z0-9_]*", "", line)
            cmdline.append(line)
    cmdline.append("--enable-option-checking=fatal")
    print("Running '{}'".format("' '".join(cmdline)))
    subprocess.run(cmdline, check=True)

if __name__ == "__main__":
    main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
