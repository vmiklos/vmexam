# fit2json

Wrapper around [gpsbabel](https://www.gpsbabel.org/):

- converts .fit files to .json, so [Leaflet](https://leafletjs.com/) can show it

- take the activity name from a matching .meta.json file, if it's there

- include stats in the geojson, based on what gpsbabel provides in KML files
