#!/usr/bin/env python
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

import re
import sys

# Inspired by http://www.unitconversion.org/unit_converter/typography.html
#
# Additionally:
# - supports UNO API's mm100 by default
# - supports OOXML's EMU by default
# - possible to extend

conv = {
        'inch': 914400, # "there are 914,400 EMUs per inch"
        'point': 914400/72, # EMU / point
        'twip': 914400/72/20, # EMU / twip

        'm': 360*100000, # EMU / m
        'cm': 360*1000, # EMU is defined as 1/360,000 of a centimeter
        'mm': 360*100, # EMU / mm
        'mm100': 360, # EMU / mm100

        'emu': 1, # EMU / EMU
    }

def convert(amount, fro, to):
    # convert to EMU
    emu = amount * conv[re.sub("s$", "", fro)]
    return emu / conv[re.sub("s$", "", to)]

def main(args):
    try:
        amount = float(args[1])
        fro = args[2]
        to = args[4]
    except IndexError:
        print "usage: tpconv <amount> <from> in <to>"
        return

    print convert(amount, fro, to)

if __name__ == '__main__':
    main(sys.argv)

# vim:set filetype=python shiftwidth=4 softtabstop=4 expandtab:
