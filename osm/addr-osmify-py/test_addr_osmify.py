#!/usr/bin/env python3
#
# Copyright (c) 2019 Miklos Vajna and contributors.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

"""The test_addr_osmify module covers the addr_osmify module."""

from typing import BinaryIO
from typing import Optional
import io
import os
import unittest
import unittest.mock
import urllib
import addr_osmify


def mock_urlopen(url: str, _data: Optional[bytes] = None) -> BinaryIO:
    """Mocks urllib.request.urlopen()."""
    path = os.path.join("mock", urllib.parse.quote_plus(url))
    with open(path, "rb") as stream:
        buf = io.BytesIO()
        buf.write(stream.read())
        buf.seek(0)
        return buf


class TestMain(unittest.TestCase):
    """Tests main()."""
    def test_happy(self) -> None:
        """Tests the happy path."""
        with unittest.mock.patch('urllib.request.urlopen', mock_urlopen):
            argv = ["", "Mészáros utca 58/a, Budapest"]
            with unittest.mock.patch('sys.argv', argv):
                buf = io.StringIO()
                with unittest.mock.patch('sys.stdout', buf):
                    addr_osmify.main()
                buf.seek(0)
                self.assertTrue(buf.read(), "foo")


if __name__ == '__main__':
    unittest.main()
