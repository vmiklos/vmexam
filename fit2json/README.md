# fit2json

Wrapper around [gpsbabel](https://www.gpsbabel.org/):

- converts .fit files to .json, so [Leaflet](https://leafletjs.com/) can show it

- take the activity name & stats from a matching .meta.json file, if it's produced by
  [strava-backup](https://github.com/pR0Ps/strava-backup) >= 0.3.2

This gives you a way to share unlisted (not fully public, neither fully private) activities with
your friends, which is not something Strava provides, unless those friends register on Strava & are
your followers.
