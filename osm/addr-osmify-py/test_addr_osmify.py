#!/usr/bin/env python3
#
# Copyright (c) 2019 Miklos Vajna and contributors.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

"""The test_addr_osmify module covers the addr_osmify module."""

from typing import BinaryIO
from typing import Callable
from typing import List
from typing import Optional
import io
import time
import unittest
import unittest.mock
import addr_osmify


class URLRoute:
    """Contains info about how to mock one URL."""
    # The request URL
    url: str
    # Path of expected POST data, empty for GET
    data_path: str
    # Path of expected result data
    result_path: str

    def __init__(self, url: str, data_path: str, result_path: str) -> None:
        self.url = url
        self.data_path = data_path
        self.result_path = result_path


class TestMain(unittest.TestCase):
    """Tests main()."""
    def mock_urlopen(self, routes: List[URLRoute]) -> Callable[[str, Optional[bytes]], BinaryIO]:
        """Generates a mock for urllib.request.urlopen()."""
        def mock_urlopen_with_route(url: str, data: Optional[bytes] = None) -> BinaryIO:
            """Mocks urllib.request.urlopen()."""
            for route in routes:
                if url != route.url:
                    continue

                if route.data_path:
                    with open(route.data_path, "rb") as stream:
                        self.assertEqual(stream.read(), data)

                with open(route.result_path, "rb") as stream:
                    buf = io.BytesIO()
                    buf.write(stream.read())
                    buf.seek(0)
                    # Make sure that the 100ms progressbar spins at least once.
                    time.sleep(0.2)
                    return buf
            self.fail("url missing from route list: '" + url + "'")
        return mock_urlopen_with_route

    def test_happy(self) -> None:
        """Tests the happy path."""
        nominatim_url = "http://nominatim.openstreetmap.org/search.php?"
        nominatim_url += "q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest&format=json"
        routes: List[URLRoute] = [
            URLRoute(url=nominatim_url,
                     data_path="",
                     result_path="mock/nominatim-happy.json"),
            URLRoute(url="http://overpass-api.de/api/interpreter",
                     data_path="mock/overpass-happy.expected-data",
                     result_path="mock/overpass-happy.json")
        ]
        with unittest.mock.patch('urllib.request.urlopen', self.mock_urlopen(routes)):
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
        nominatim_url = "http://nominatim.openstreetmap.org/search.php?"
        nominatim_url += "q=Karinthy+Frigyes+%C3%BAt+18%2C+Budapest&format=json"
        routes: List[URLRoute] = [
            URLRoute(url=nominatim_url,
                     data_path="",
                     result_path="mock/nominatim-prefer-buildings.json"),
            URLRoute(url="http://overpass-api.de/api/interpreter",
                     data_path="mock/overpass-prefer-buildings.expected-data",
                     result_path="mock/overpass-prefer-buildings.json")
        ]
        with unittest.mock.patch('urllib.request.urlopen', self.mock_urlopen(routes)):
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
        nominatim_url = "http://nominatim.openstreetmap.org/search.php?"
        nominatim_url += "q=Karinthy+Frigyes+%C3%BAt+18%2C+Budapest&format=json"
        routes: List[URLRoute] = [
            URLRoute(url=nominatim_url,
                     data_path="",
                     result_path="mock/nominatim-no-buildings.json"),
            URLRoute(url="http://overpass-api.de/api/interpreter",
                     data_path="mock/overpass-no-buildings.expected-data",
                     result_path="mock/overpass-no-buildings.json")
        ]
        with unittest.mock.patch('urllib.request.urlopen', self.mock_urlopen(routes)):
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
        nominatim_url = "http://nominatim.openstreetmap.org/search.php?"
        nominatim_url += "q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapestt&format=json"
        routes: List[URLRoute] = [
            URLRoute(url=nominatim_url,
                     data_path="",
                     result_path="mock/nominatim-no-result.json")
        ]
        with unittest.mock.patch('urllib.request.urlopen', self.mock_urlopen(routes)):
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
        nominatim_url = "http://nominatim.openstreetmap.org/search.php?"
        nominatim_url += "q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest&format=json"
        routes: List[URLRoute] = [
            URLRoute(url=nominatim_url,
                     data_path="",
                     result_path="mock/nominatim-overpass-noresult.json"),
            URLRoute(url="http://overpass-api.de/api/interpreter",
                     data_path="mock/overpass-noresult.expected-data",
                     result_path="mock/overpass-noresult.json")
        ]
        with unittest.mock.patch('urllib.request.urlopen', self.mock_urlopen(routes)):
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
