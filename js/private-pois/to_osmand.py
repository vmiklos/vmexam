#!/usr/bin/env python3

# From
# <https://stackoverflow.com/questions/64558458/how-to-convert-google-maps-geojson-to-gpx-retaining-location-names>.

import argparse
import json
import xml.etree.ElementTree as ET
from xml.dom import minidom


def ingestJson(geoJsonFilepath):
    poiList = []
    with open(geoJsonFilepath) as fileObj:
        data = json.load(fileObj)
        for f in data["features"]:
            poiList.append({'name': f["properties"]["Name"],
                            'lon': f["geometry"]["coordinates"][0],
                            'lat': f["geometry"]["coordinates"][1],
                            'link': f["properties"].get("Google Maps URL", '')})
    return poiList


def dumpGpx(gpxFilePath, poiList):
    gpx = ET.Element("gpx", version="1.1", creator="", xmlns="http://www.topografix.com/GPX/1/1")
    for poi in poiList:
        wpt = ET.SubElement(gpx, "wpt", lat=str(poi["lat"]), lon=str(poi["lon"]))
        ET.SubElement(wpt, "name").text = poi["name"]
        ET.SubElement(wpt, "link").text = poi["link"]
    xmlstr = minidom.parseString(ET.tostring(gpx)).toprettyxml(encoding="utf-8", indent="  ")
    with open(gpxFilePath, "wb") as f:
        f.write(xmlstr)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--inputGeoJsonFilepath', required=True)
    parser.add_argument('--outputGpxFilepath', required=True)
    args = parser.parse_args()

    poiList = ingestJson(args.inputGeoJsonFilepath)
    dumpGpx(args.outputGpxFilepath, poiList=poiList)


if __name__ == "__main__":
    main()
