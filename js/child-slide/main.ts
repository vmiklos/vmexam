/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

interface CollectionData {
    [videoUrl: string]: { [kidName: string]: number };
}

function findMinMax(data: CollectionData, kid: string): [number, number] {
    let min = Infinity;
    let max = -Infinity;
    for (const ages of Object.values(data)) {
        if (kid in ages) {
            const age = ages[kid];
            if (age < min) {
                min = age;
            }
            if (age > max) {
                max = age;
            }
        }
    }
    return [min, max];
}

function findClosestVideo(data: CollectionData, kid: string, targetAge: number): string | null {
    let bestUrl: string | null = null;
    let bestDiff = Infinity;
    for (const [url, ages] of Object.entries(data)) {
        if (kid in ages) {
            const diff = Math.abs(ages[kid] - targetAge);
            if (diff < bestDiff) {
                bestDiff = diff;
                bestUrl = url;
            }
        }
    }
    return bestUrl;
}

document.addEventListener("DOMContentLoaded", async function () {
    const urlParams = new URLSearchParams(window.location.search);
    const token = urlParams.get("c");
    if (token == null) {
        document.body.appendChild(document.createTextNode("Missing ?c= parameter."));
        return;
    }

    const response = await window.fetch(token + ".json");
    const data: CollectionData = await response.json();

    const kids = [...new Set(Object.values(data).flatMap((ages) => Object.keys(ages)))];
    kids.sort();

    const container = document.createElement("div");
    container.style.fontFamily = "sans-serif";
    container.style.maxWidth = "400px";
    container.style.margin = "2rem auto";

    // Kid dropdown
    const kidLabel = document.createElement("label");
    kidLabel.textContent = "Kid name:";
    container.appendChild(kidLabel);

    const kidSelect = document.createElement("select");
    kidSelect.appendChild(document.createElement("option"));
    for (const kid of kids) {
        const option = document.createElement("option");
        option.value = kid;
        option.textContent = kid;
        kidSelect.appendChild(option);
    }
    container.appendChild(document.createElement("br"));
    container.appendChild(kidSelect);

    // Age slider
    const ageLabel = document.createElement("label");
    ageLabel.textContent = "Age: ";
    container.appendChild(document.createElement("br"));
    container.appendChild(ageLabel);

    const slider = document.createElement("input");
    slider.type = "range";
    slider.disabled = true;
    slider.style.width = "100%";
    container.appendChild(slider);

    const ageValue = document.createElement("span");
    container.appendChild(ageValue);

    kidSelect.addEventListener("change", function () {
        const kid = this.value;
        if (kid === "") {
            slider.disabled = true;
            return;
        }
        const [min, max] = findMinMax(data, kid);
        slider.min = String(min);
        slider.max = String(max);
        slider.value = String(min);
        slider.disabled = false;
        ageValue.textContent = String(min);
    });

    slider.addEventListener("input", function () {
        ageValue.textContent = this.value;
    });

    // Go button
    const goButton = document.createElement("button");
    goButton.textContent = "Go!";
    goButton.addEventListener("click", function () {
        const kid = kidSelect.value;
        if (kid === "") {
            return;
        }
        const url = findClosestVideo(data, kid, Number(slider.value));
        if (url != null) {
            window.location.href = url;
        }
    });
    container.appendChild(document.createElement("br"));
    container.appendChild(goButton);

    document.body.appendChild(container);
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
