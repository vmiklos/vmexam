#!/usr/bin/env python3
#
# Copyright (c) 2019 Miklos Vajna and contributors.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

"""The test_addr_osmify module covers the addr_osmify module."""

from typing import BinaryIO
from typing import Callable
from typing import Optional
import io
import os
import time
import unittest
import unittest.mock
import urllib
import addr_osmify


class TestMain(unittest.TestCase):
    """Tests main()."""
    def gen_urlopen(self, suffix: str) -> Callable[[str, Optional[bytes]], BinaryIO]:
        """Generates a mock for urllib.request.urlopen()."""
        def mock_urlopen_with_suffix(url: str, data: Optional[bytes] = None) -> BinaryIO:
            """Mocks urllib.request.urlopen()."""
            if data:
                data_path = os.path.join("mock", urllib.parse.quote_plus(url))
                data_path += suffix + ".expected-data"
                with open(data_path, "rb") as stream:
                    self.assertEqual(stream.read(), data)

            path = os.path.join("mock", urllib.parse.quote_plus(url))
            path += suffix
            with open(path, "rb") as stream:
                buf = io.BytesIO()
                buf.write(stream.read())
                buf.seek(0)
                # Make sure that the 100ms progressbar spins at least once.
                time.sleep(0.2)
                return buf
        return mock_urlopen_with_suffix

    def test_happy(self) -> None:
        """Tests the happy path."""
        with unittest.mock.patch('urllib.request.urlopen', self.gen_urlopen("-happy")):
            argv = ["", "Mészáros utca 58/a, Budapest"]
            with unittest.mock.patch('sys.argv', argv):
                buf = io.StringIO()
                with unittest.mock.patch('sys.stdout', buf):
                    addr_osmify.main()
                buf.seek(0)
                expected = "geo:47.490592,19.030662 (1016 Budapest, Mészáros utca 58/a)\n"
                self.assertEqual(buf.read(), expected)

    def test_prefer_buildings(self) -> None:
        """Tests that buildings are preferred in case of multiple results."""
        with unittest.mock.patch('urllib.request.urlopen', self.gen_urlopen("-prefer-buildings")):
            argv = ["", "Karinthy Frigyes út 18, Budapest"]
            with unittest.mock.patch('sys.argv', argv):
                buf = io.StringIO()
                with unittest.mock.patch('sys.stdout', buf):
                    addr_osmify.main()
                buf.seek(0)
                expected = "geo:47.47690895,19.0512550758533"
                expected += " (1111 Budapest, Karinthy Frigyes út 18)\n"
                self.assertEqual(buf.read(), expected)

    def test_no_buildings(self) -> None:
        """Similar to test_prefer_buildings(), but none of the results are a building."""
        with unittest.mock.patch('urllib.request.urlopen', self.gen_urlopen("-no-buildings")):
            argv = ["", "Karinthy Frigyes út 18, Budapest"]
            with unittest.mock.patch('sys.argv', argv):
                buf = io.StringIO()
                with unittest.mock.patch('sys.stdout', buf):
                    addr_osmify.main()
                buf.seek(0)
                expected = "geo:47.47690895,19.0512550758533"
                expected += " (1111 Budapest, Karinthy Frigyes út 18)\n"
                self.assertEqual(buf.read(), expected)

    def test_nominatim_noresults(self) -> None:
        """Tests the case when nominatim gives no results."""
        with unittest.mock.patch('urllib.request.urlopen', self.gen_urlopen("-no-result")):
            argv = ["", "Mészáros utca 58/a, Budapestt"]
            with unittest.mock.patch('sys.argv', argv):
                buf = io.StringIO()
                with unittest.mock.patch('sys.stdout', buf):
                    addr_osmify.main()
                buf.seek(0)
                expected = "No results from nominatim\n"
                self.assertEqual(buf.read(), expected)

    def test_overpass_noresults(self) -> None:
        """Tests the case when overpass gives no results."""
        with unittest.mock.patch('urllib.request.urlopen', self.gen_urlopen("-overpass-noresult")):
            argv = ["", "Mészáros utca 58/a, Budapest"]
            with unittest.mock.patch('sys.argv', argv):
                buf = io.StringIO()
                with unittest.mock.patch('sys.stdout', buf):
                    addr_osmify.main()
                buf.seek(0)
                expected = "No results from overpass\n"
                self.assertEqual(buf.read(), expected)

    def test_noargs(self) -> None:
        """Tests the case where there are not enough arguments."""
        argv = [""]
        with unittest.mock.patch('sys.argv', argv):
            buf = io.StringIO()
            with unittest.mock.patch('sys.stdout', buf):
                addr_osmify.main()
            buf.seek(0)
            self.assertTrue(buf.read().startswith("usage:"))


if __name__ == '__main__':
    unittest.main()
