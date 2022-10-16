/*
 * Copyright 2020 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

import * as L from 'leaflet';

const center: L.LatLngTuple = [ 47.4744, 19.0045 ];
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
const tracks = [
    {url : 'sasad.json', color : 'green'},
    {url : 'gazdagret.json', color : 'green'},
    {url : 'sashegy.json', color : 'green'},
    {url : 'nemetvolgy.json', color : 'green'},
    {url : 'ormezo.json', color : 'green'},
    {url : 'farkasvolgy.json', color : 'green'},
    {url : 'magasut.json', color : 'green'},
    {url : 'farkasret.json', color : 'green'},
    {url : 'hosszuret.json', color : 'green'},
    {url : 'madarhegy.json', color : 'green'},
    {url : 'krisztinavaros.json', color : 'green'},
    {url : 'kissvabhegy.json', color : 'green'},
    {url : 'orbanhegy.json', color : 'green'},
    {url : 'svabhegy.json', color : 'green'},
    {url : 'martonhegy.json', color : 'green'},
    {url : 'szechenyihegy.json', color : 'green'},
    {url : 'istenhegy.json', color : 'green'},
    {url : 'kutvolgy.json', color : 'green'},
    {url : 'viranyos.json', color : 'green'},
    {url : 'csilleberc.json', color : 'green'},
    {url : 'zugliget.json', color : 'orange'},
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
