/*
 * Copyright 2020 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

import * as L from 'leaflet';

const center: L.LatLngTuple = [ 47.4744, 19.0045 ];
const zoom = 12;

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
const tracks = [
    // green: done, orange: in progress, red: to do.
    {url : 'budapest01.json', color : 'green'},
    {url : 'budapest11.json', color : 'green'},
    {url : 'budapest12.json', color : 'green'},
    {url : 'budapest22.json', color : 'orange'}, // urbalazs
    {url : 'budaors.json', color : 'green'},     // vasony
];

// Boilerplate below.

async function addRelations(map: L.Map)
{
    for (let i = 0; i < tracks.length; i += 1)
    {
        const track = tracks[i];
        const url = track.url;
        const response = await window.fetch(url);
        const relation = await response.json();
        // There are 2 features (boundary and center), we only care about
        // the first one.
        relation.features.pop();
        L.geoJSON(relation, {
             style : {
                 color : track.color,
                 opacity : 0.5,
                 weight : 5,
                 fillColor : track.color,
                 fillOpacity : 0.1
             }
         }).addTo(map);
    }
}

document.addEventListener('DOMContentLoaded', function() {
    const map = L.map('map').setView(center, zoom);

    L.tileLayer('http://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
         attribution :
             '&copy; <a href="https://www.openstreetmap.org/copyright" target="_blank">OpenStreetMap</a> contributors.',
     }).addTo(map);

    addRelations(map);
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
