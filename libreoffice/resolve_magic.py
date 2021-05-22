#!/usr/bin/env python3
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

#
# Resolves a magic number if we know it was constructed from flags of an enum.
#
# Example:
#
# $ ./resolve_magic.py SwFrameInvFlags 0x13
# SwFrameInvFlags::InvalidatePrt | SwFrameInvFlags::InvalidateSize | SwFrameInvFlags::NextInvalidatePos
#
# Convert from C++: s/    \(.*\) = \(.*\),/    \2: "\1",/g
#


import sys

SwFrameInvFlags = {
    0x01: "InvalidatePrt",
    0x02: "InvalidateSize",
    0x04: "InvalidatePos",
    0x08: "SetCompletePaint",
    0x10: "NextInvalidatePos",
    0x20: "NextSetCompletePaint",
}

SwFlyFrameInvFlags = {
  0x01: "InvalidatePos",
  0x02: "InvalidateSize",
  0x04: "InvalidatePrt",
  0x08: "SetNotifyBack",
  0x10: "SetCompletePaint",
  0x20: "InvalidateBrowseWidth",
  0x40: "ClearContourCache",
  0x80: "UpdateObjInSortedList",
}

SwPageFrameInvFlags = {
  0x00: "NONE",
  0x01: "InvalidatePrt",
  0x02: "SetCompletePaint",
  0x04: "InvalidateNextPos",
  0x08: "PrepareHeader",
  0x10: "PrepareFooter",
  0x20: "CheckGrid",
  0x40: "InvalidateGrid",
}

SwSectionFrameInvFlags = {
  0x01: "InvalidateSize",
  0x10: "SetCompletePaint",
}

SwTabFrameInvFlags = {
    0x00: "NONE",
    0x02: "InvalidatePrt",
    0x04: "InvalidateIndNextPrt",
    0x08: "InvalidatePrevPrt",
    0x10: "SetIndNextCompletePaint",
    0x20: "InvalidateBrowseWidth",
    0x40: "InvalidatePos",
    0x80: "InvalidateNextPos",
}

SwContentFrameInvFlags = {
    0x00: "NONE",
    0x01: "SetCompletePaint",
    0x02: "InvalidatePos",
    0x04: "InvalidateSize",
    0x08: "InvalidateSectPrt",
    0x10: "InvalidateNextPrt",
    0x20: "InvalidatePrevPrt",
    0x40: "InvalidateNextPos",
    0x80: "SetNextCompletePaint",
};

def main():
    enum = sys.argv[1]
    magic = int(sys.argv[2], 16)

    flags = []
    enum_dict = globals()[enum]
    for key, value in enum_dict.items():
        if magic & key:
            flags.append(value)
    print(" | ".join(["%s::%s" % (enum, i) for i in flags]))


if __name__ == '__main__':
    main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
