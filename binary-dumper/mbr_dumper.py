#!/usr/bin/env python3
#
# Copyright 2021 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

"""
Dumps a Master Boot Record.

See <https://wiki.osdev.org/MBR_(x86)> for a spec.
"""

from typing import cast
import struct
import sys


class BinaryStream:
    """Wrapper around struct.unpack()."""
    def __init__(self, byte_array: bytes) -> None:
        self.bytes = byte_array
        self.size = len(self.bytes)
        self.pos = 0

    def print_and_set(self, key: str, value: int) -> None:
        """Sets self.key to value, then prints value."""
        setattr(self, key, value)
        print('<%s value="%s"/>' % (key, hex(value)))

    def read_uint8(self) -> int:
        """Reads a little-endian unsigned byte."""
        num = struct.unpack("<B", self.bytes[self.pos:self.pos + 1])[0]
        self.pos += 1
        return cast(int, num)

    def read_uint16(self) -> int:
        """Reads a little-endian unsigned short."""
        num = struct.unpack("<H", self.bytes[self.pos:self.pos + 2])[0]
        self.pos += 2
        return cast(int, num)

    def read_uint32(self) -> int:
        """Reads a little-endian unsigned int."""
        num = struct.unpack("<I", self.bytes[self.pos:self.pos + 4])[0]
        self.pos += 4
        return cast(int, num)


class BootstrapRecord(BinaryStream):
    """Dumps the MBR Bootstrap (flat binary executable code)."""
    def __init__(self, parent: BinaryStream) -> None:
        BinaryStream.__init__(self, parent.bytes)
        self.parent = parent
        self.pos = parent.pos

    def dump(self) -> None:
        """Dumps the binary in XML format."""
        print('<bootstrap-record>')
        pos_orig = self.pos
        word_count = 0
        byte_array = []
        array_start_pos = self.pos
        while self.pos < pos_orig + 440:
            byte_array.append(self.read_uint16())
            word_count += 1
            if word_count == 8:
                offset = "%4.4X" % array_start_pos
                byte_string = " ".join(["%4.4X" % i for i in byte_array])
                print('<chunk offset="%s" bytes="%s"/>' % (offset, byte_string))
                word_count = 0
                byte_array = []
                array_start_pos = self.pos
        if word_count != 0:
            offset = "%4.4X" % array_start_pos
            byte_string = " ".join(["%4.4X" % i for i in byte_array])
            print('<chunk offset="%s" bytes="%s"/>' % (offset, byte_string))
        print('</bootstrap-record>')
        self.parent.pos = self.pos


class PartitionRecord(BinaryStream):
    """Dumps a partition table entry."""
    def __init__(self, parent: BinaryStream) -> None:
        BinaryStream.__init__(self, parent.bytes)
        self.parent = parent
        self.pos = parent.pos

    def dump(self) -> None:
        """Dumps the binary in XML format."""
        print('<partition-record>')
        self.print_and_set("drive_attributes", self.read_uint8())
        # See <https://wiki.osdev.org/Partition_Table>.
        self.print_and_set("starting_chs_1", self.read_uint8())
        self.print_and_set("starting_chs_2", self.read_uint8())
        self.print_and_set("starting_chs_3", self.read_uint8())
        self.print_and_set("partition_type", self.read_uint8())
        self.print_and_set("ending_chs_1", self.read_uint8())
        self.print_and_set("ending_chs_2", self.read_uint8())
        self.print_and_set("ending_chs_3", self.read_uint8())
        # start sector, 1 sector = 512 bytes
        self.print_and_set("lba", self.read_uint32())
        self.print_and_set("sector_count", self.read_uint32())
        print('</partition-record>')
        self.parent.pos = self.pos


class MBRStream(BinaryStream):
    """Toplevel record of an MBR byte array."""
    def __init__(self, byte_array: bytes) -> None:
        BinaryStream.__init__(self, byte_array)

    def dump(self) -> None:
        """Dumps the binary in XML format."""
        pos_orig = self.pos
        print('<stream type="MBR" size="%d">' % self.size)
        BootstrapRecord(self).dump()
        self.print_and_set("disk_id", self.read_uint32())
        self.print_and_set("reserved", self.read_uint16())
        for _ in range(4):
            PartitionRecord(self).dump()
        self.print_and_set("signature", self.read_uint16())
        print('</stream>')
        assert self.pos == pos_orig + 512


def main() -> None:
    """Commandline interface to this module."""
    if len(sys.argv) > 1:
        with open(sys.argv[1], "rb") as stream:
            mbr_stream = MBRStream(stream.read())
            print('<?xml version="1.0"?>')
            mbr_stream.dump()
    else:
        print("usage: mbr_dump.py <input>")


if __name__ == "__main__":
    main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
