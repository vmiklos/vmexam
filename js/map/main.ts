/*
 * Copyright 2024 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

import * as L from "leaflet";

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
    const activityURL = urlParams.get("activity");
    const response = await window.fetch(activityURL);
    const activity = await response.json();
    const geoJSON = L.geoJSON(activity).addTo(map);
    map.fitBounds(geoJSON.getBounds());
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
