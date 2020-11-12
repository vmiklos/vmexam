/*
 * Copyright 2020 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

import domready = require('domready');
import KML from 'ol/format/KML';
import VectorLayer from 'ol/layer/Vector';
import VectorSource from 'ol/source/Vector';
import {Fill, Stroke, Style} from 'ol/style';
import {Map, View} from 'ol';
import {OSM} from 'ol/source';
import {Tile} from 'ol/layer';
import {fromLonLat} from 'ol/proj';

const center = [ 19.0045, 47.4744 ];
const zoom = 14;

// Generate KML with overpass, see <http://overpass-turbo.eu/s/gLa> for an
// example, or if that gets broken:
// [out:json][timeout:25];
// (
//   relation(2806937);
// );
// out body;
// >;
// out skel qt;

// List of areas which were not 100% and at least once I pulled it up to 100%.
// Not claiming I did all the work there. :-)
const green = [ 0, 128, 0 ];
const orange = [ 255, 165, 0 ];
const tracks = [
    {url : "sasad.kml", color : green},
    {url : "gazdagret.kml", color : green},
    {url : "sashegy.kml", color : green},
    {url : "nemetvolgy.kml", color : green},
    {url : "ormezo.kml", color : green},
    {url : "farkasvolgy.kml", color : green},
    {url : "magasut.kml", color : green},
    {url : "farkasret.kml", color : green},
    {url : "hosszuret.kml", color : green},
    {url : "madarhegy.kml", color : green},
    {url : "krisztinavaros.kml", color : green},
    {url : "kissvabhegy.kml", color : green},
    {url : "orbanhegy.kml", color : green},
    {url : "svabhegy.kml", color : green},
    {url : "martonhegy.kml", color : green},
    {url : "szechenyihegy.kml", color : orange},
];

// Boilerplate below.

domready(function() {
    // Project from EPSG:4326 / WGS 1984 to Spherical Mercator.
    const projectedCenter = fromLonLat(center);
    const map = new Map({
        target : 'map',
        view : new View({center : projectedCenter, zoom : zoom})
    });
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const layer: any = new Tile({source : new OSM()});
    map.addLayer(layer);

    // Add the layers for the relations.
    tracks.forEach(function(track) {
        const color = track.color;
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const vector: any = new VectorLayer({
            source : new VectorSource({
                url : track.url,
                format : new KML({extractStyles : false}),
            }),
            style : new Style({
                stroke : new Stroke({
                    color : [ color[0], color[1], color[2], 0.5 ],
                    width : 5,
                }),
                fill : new Fill({
                    color : [ color[0], color[1], color[2], 0.1 ],
                }),
            }),
        });
        map.addLayer(vector);
    });
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
