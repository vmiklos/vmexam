/*
 * Copyright 2024 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

interface RubikResult {
    ok: string;
    error: string;
}

document.addEventListener("DOMContentLoaded", async function () {
    const body = document.querySelector("body");
    const urlParams = new URLSearchParams(window.location.search);
    let lang = urlParams.get("lang");
    if (lang == null) {
        lang = "en";
    }
    const wideParam = urlParams.get("wide");
    let wide;
    if (wideParam != null) {
        wide = "&wide=" + wideParam;
    } else {
        wide = "";
    }
    const stateParam = urlParams.get("state");
    let state;
    if (stateParam != null) {
        state = "&state=" + stateParam;
    } else {
        state = "";
    }

    const megaminxParam = urlParams.get("megaminx");
    let megaminx;
    if (megaminxParam != null) {
        megaminx = "&megaminx=" + megaminxParam;
    } else {
        megaminx = "";
    }

    const colorsParam = urlParams.get("colors");
    let colors;
    if (colorsParam != null) {
        colors = "&colors=" + colorsParam;
    } else {
        colors = "";
    }

    const request = new Request(
        "https://share.vmiklos.hu/apps/rubik-scramble/?lang=" +
            lang +
            wide +
            state +
            megaminx +
            colors
    );
    const response = await window.fetch(request);
    const result = await (<Promise<RubikResult>>response.json());
    const scramble = document.createElement("div");
    scramble.style.fontSize = 'xxx-large';
    if (result.error === "") {
        scramble.innerText = result.ok;
    } else {
        scramble.innerText = "Error: " + result.error;
    }
    body.appendChild(scramble);

    const help = document.createElement("p");
    help.appendChild(document.createTextNode("See "));
    const a = document.createElement("a");
    a.href = "https://meep.cubing.net/wcanotation.html";
    a.innerText = "WCA Notation";
    help.appendChild(a);
    help.appendChild(document.createTextNode(" for help on face turns."));
    body.appendChild(help);
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
