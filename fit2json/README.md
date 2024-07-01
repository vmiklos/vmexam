# fit2json

Wrapper around [gpsbabel](https://www.gpsbabel.org/):

- converts .fit files to .json, so [Leaflet](https://leafletjs.com/) can show it

- take the activity name from a matching .meta.json file, if it's produced by
  [strava-backup](https://github.com/pR0Ps/strava-backup) >= 0.3.2

- include stats in the geojson, based on what gpsbabel provides in KML files
