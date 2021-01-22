/*
 * Copyright 2021 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

import * as geojson from 'geojson';
import * as L from 'leaflet';

// Show POI name on click.
function onEachFeature(feature: geojson.Feature<geojson.GeometryObject, any>,
                       layer: L.Layer)
{
    if (feature.properties == null || feature.properties.Name == null)
    {
        return;
    }

    layer.bindPopup(feature.properties.Name);
}

async function addPois(map: L.Map)
{
    const url = "pois.json";
    const response = await window.fetch(url);
    const pois = await response.json();
    const geoJSON =
        L.geoJSON(pois, {
             // Avoid default POI indicator, which would be a bitmap.
             pointToLayer : function(feature, latlng) {
                 return L.circleMarker(latlng, {
                     radius : 8,
                     fillColor : "#ff7800",
                     color : "#000",
                     weight : 1,
                     opacity : 1,
                     fillOpacity : 0.8
                 });
             },
             onEachFeature : onEachFeature
         }).addTo(map);
    map.fitBounds(geoJSON.getBounds());
}

document.addEventListener('DOMContentLoaded', function() {
    const map = L.map('map');

    // JSON for showing here on the map, XML for OSMAnd.
    L.tileLayer('http://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
         attribution :
             '&copy; <a href="https://www.openstreetmap.org/copyright" target="_blank">OpenStreetMap</a> contributors. (<a href="pois.json">json</a>, <a href="pois.xml">xml</a>)',
     }).addTo(map);

    addPois(map);
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
