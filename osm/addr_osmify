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
import urllib.parse
import urllib.request


def query_turbo(query: str) -> str:
    """Send query to overpass turbo."""
    url = "http://overpass-api.de/api/interpreter"

    sock = urllib.request.urlopen(url, bytes(query, "utf-8"))
    buf = sock.read()
    sock.close()

    return buf.decode("utf-8")


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

    return buf.decode("utf-8")


def osmify(query: str) -> None:
    """Turn query into a coordinate + address string."""
    # Use nominatim to get the coordinates and the osm type/id.
    elements = json.loads(query_nominatim(query))
    if not elements:
        print("No results from nominatim")
        return

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
        print("No results from overpass")
        return

    element = elements[0]
    city = element['tags']['addr:city']
    housenumber = element['tags']['addr:housenumber']
    postcode = element['tags']['addr:postcode']
    street = element['tags']['addr:street']
    addr = "%s %s, %s %s" % (postcode, city, street, housenumber)

    # Print the result.
    print("geo:%s,%s (%s)" % (lat, lon, addr))


if len(sys.argv) > 1:
    osmify(sys.argv[1])
else:
    print("usage: addr-osmify <query>")
    print()
    print("e.g. addr-osmify 'Mészáros utca 58/a, Budapest'")

# vim:set shiftwidth=4 softtabstop=4 expandtab:
