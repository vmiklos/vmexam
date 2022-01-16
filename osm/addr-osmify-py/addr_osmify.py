#!/usr/bin/env python3
#
# Copyright 2019 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

"""
Takes an OSM way ID and turns it into a string that is readable and
e.g. OsmAnd can parse it as well.
"""

import json
import sys
import threading
import urllib.parse
import urllib.request
from typing import Dict
from typing import cast


def query_turbo(query: str) -> str:
    """Send query to overpass turbo."""
    url = "http://overpass-api.de/api/interpreter"

    sock = urllib.request.urlopen(url, bytes(query, "utf-8"))
    buf = sock.read()
    sock.close()

    return cast(str, buf.decode("utf-8"))


def query_nominatim(query: str) -> str:
    """Send query to nominatim."""
    url = "http://nominatim.openstreetmap.org/search.php?"
    params = {
        "q": query,
        "format": "json"
    }
    url += urllib.parse.urlencode(params)

    sock = urllib.request.urlopen(url)
    buf = sock.read()
    sock.close()

    return cast(str, buf.decode("utf-8"))


def osmify(query: str) -> str:
    """Turn query into a coordinate + address string."""
    # Use nominatim to get the coordinates and the osm type/id.
    elements = json.loads(query_nominatim(query))
    if not elements:
        return "No results from nominatim"

    if len(elements) > 1:
        # There are multiple elements, prefer buildings if possible.
        # Example where this is useful: 'Karinthy Frigyes út 18, Budapest'.
        buildings = [i for i in elements if "class" in i.keys() and i["class"] == "building"]
        if buildings:
            elements = buildings

    element = elements[0]
    lat = element["lat"]
    lon = element["lon"]
    object_type = element["osm_type"]
    object_id = element["osm_id"]

    # Use overpass to get the properties of the object.
    overpass_query = """[out:json];
(
    %s(%s);
);
out body;""" % (object_type, object_id)
    j = json.loads(query_turbo(overpass_query))
    elements = j["elements"]
    if not elements:
        return "No results from overpass"

    element = elements[0]
    city = element['tags']['addr:city']
    housenumber = element['tags']['addr:housenumber']
    postcode = element['tags']['addr:postcode']
    street = element['tags']['addr:street']
    addr = "%s %s, %s %s" % (postcode, city, street, housenumber)

    # Print the result.
    return "%s,%s (%s)" % (lat, lon, addr)


def worker(context: Dict[str, str]) -> None:
    """Wrapper around osmify() that has no return value."""
    context["out"] = osmify(context["in"])


def spinner(context: Dict[str, str], thread: threading.Thread) -> None:
    """Shows a spinner while osmify() is in progress."""
    spin_characters = "\\|/-"
    spin_index = 0
    while True:
        thread.join(timeout=0.1)
        if thread.is_alive():
            sys.stderr.write("\r [%s] " % spin_characters[spin_index])
            sys.stderr.flush()
            spin_index = (spin_index + 1) % len(spin_characters)
            continue

        sys.stderr.write("\r")
        sys.stderr.flush()
        print(context["out"])
        break


def main() -> None:
    """Commandline interface to this module."""
    if len(sys.argv) > 1:
        context = {"in": sys.argv[1]}
        thread = threading.Thread(target=worker, args=(context,))
        thread.start()
        spinner(context, thread)
    else:
        print("usage: addr-osmify <query>")
        print()
        print("e.g. addr-osmify 'Mészáros utca 58/a, Budapest'")


if __name__ == "__main__":
    main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
