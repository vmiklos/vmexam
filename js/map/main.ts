/*
 * Copyright 2024 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

import * as geojson from "geojson";
import * as L from "leaflet";

interface DescriptionSupplier {
    description: string | null;
}

function onEachFeature(
    map: L.Map,
    feature: geojson.Feature<geojson.GeometryObject, DescriptionSupplier>,
    layer: L.Layer
) {
    if (feature.properties == null || feature.properties.description == null) {
        return;
    }

    if (map != null) {
        const geometry = <geojson.MultiPoint>feature.geometry;
        L.popup()
            .setLatLng([geometry.coordinates[0][1], geometry.coordinates[0][0]])
            .setContent(feature.properties.description)
            .openOn(map);
    }

    layer.bindPopup(feature.properties.description);
}

document.addEventListener("DOMContentLoaded", async function () {
    const map = L.map("map");
    map.attributionControl.setPrefix(
        '<a href="https://leafletjs.com" title="A JavaScript library for interactive maps">Leaflet</a>'
    );

    L.tileLayer("http://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
        attribution:
            '&copy; <a href="https://www.openstreetmap.org/copyright" target="_blank">OpenStreetMap</a> contributors.',
    }).addTo(map);

    const urlParams = new URLSearchParams(window.location.search);
    let activityURLs: string[] = [];
    let activityURL = urlParams.get("activity");
    if (activityURL != null) {
        activityURLs = [activityURL];
    } else {
        activityURL = urlParams.get("a");
        if (activityURL != null) {
            activityURLs = [activityURL + ".json"];
        }
    }
    if (activityURL == null) {
        const collectionURL = urlParams.get("c") + ".json";
        const response = await window.fetch(collectionURL);
        const collection = await response.json();
        document.title = collection.title;
        for (const activity of collection.activities) {
            activityURLs.push(activity.id + ".json");
        }
    }
    let bounds: L.LatLngBounds | null = null;
    for (activityURL of activityURLs) {
        const response = await window.fetch(activityURL);
        const activity = await response.json();
        if (activityURLs.length == 1) {
            if (activity.features[0].properties) {
                const properties = activity.features[0].properties;
                if (properties.name != null) {
                    document.title = properties.name;
                }
            }
            const geoJSON = L.geoJSON(activity, {
                onEachFeature: (feature, layer) => onEachFeature(map, feature, layer),
            }).addTo(map);
            bounds = geoJSON.getBounds();
        } else {
            const geoJSON = L.geoJSON(activity, {
                onEachFeature: (feature, layer) => onEachFeature(null, feature, layer),
            }).addTo(map);
            if (bounds == null) {
                bounds = geoJSON.getBounds();
            } else {
                bounds.extend(geoJSON.getBounds());
            }
        }
    }
    map.fitBounds(bounds);
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
