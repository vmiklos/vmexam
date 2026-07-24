# gpx2osm

Converts [OSMTracker](https://github.com/labexp/osmtracker-android/wiki) .gpx files to
[JOSM](https://josm.openstreetmap.de/) .osm files.

The use-case is that you survey housenumbers, always using the "Text note" function of the app, then
you want to edit OpenStreetMap using JOSM, similar to how it was possible with .osm files from
[Keypad-Mapper](https://wiki.openstreetmap.org/wiki/Keypad-Mapper_3) in the past.

This is especially helpful since Keypad-Mapper was long deprecated, but it fully stopped working
with Android 17.

## Installation

```
cargo install --git https://github.com/vmiklos/vmexam gpx2osm
```

## Usage

```
gpx2osm input.gpx output.osm
```
