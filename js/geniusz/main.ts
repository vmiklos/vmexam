/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

import arrayShuffle from 'array-shuffle';

function refreshTopics()
{
    let topicList = [
        "élővilág", "zene", "földrajz", "sport", "nyelv és irodalom",
        "film és tv", "történelem és közélet", "életmód",
        "tudomány és technika", "bulvár"
    ];
    topicList = arrayShuffle(topicList);
    const topics = <HTMLElement>document.querySelector('#topics');
    let buf =
        "párbaj témakörei: első játszma: " + topicList.slice(0, 3).join(", ") +
        "; ";
    buf += "második játszma: " + topicList.slice(3, 6).join(", ") + "; ";
    buf += "harmadik játszma: " + topicList.slice(6, 9).join(", ") + "; ";
    buf += "kimarad: " + topicList[9];
    topics.innerText = buf;
}

// Both min and max are inclusive.
function getRandomInt(min: number, max: number): number
{
    return Math.floor(Math.random() * (max - min + 1)) + min;
}

function refreshSeries()
{
    const first = getRandomInt(1, 27);
    const second = getRandomInt(1, 10);
    const series = <HTMLElement>document.querySelector('#series');
    series.innerText =
        "kérdéssor: " + String(first) + ", kérdés: " + String(second);
}

function refreshGeneric()
{
    const first = getRandomInt(1, 130);
    const second = getRandomInt(1, 10);
    const generic = <HTMLElement>document.querySelector('#generic');
    generic.innerText =
        "kérdéssor: " + String(first) + ", kérdés: " + String(second);
}

document.addEventListener("DOMContentLoaded", function() {
    // Topics.
    const body = document.getElementsByTagName('body')[0];
    const topics = document.createElement('p');
    topics.id = 'topics';
    body.appendChild(topics);
    const topicsButton = document.createElement('input');
    topicsButton.type = 'button';
    topicsButton.value = 'új téma sorrend';
    topicsButton.onclick = refreshTopics;
    body.appendChild(topicsButton);
    refreshTopics();

    // Random topic question: 1..27 for the series, 1..10 for the question.
    const series = document.createElement('p');
    series.id = 'series';
    body.appendChild(series);
    const seriesButton = document.createElement('input');
    seriesButton.type = 'button';
    seriesButton.value = 'új téma kérdés';
    seriesButton.onclick = refreshSeries;
    body.appendChild(seriesButton);
    refreshSeries();

    // Random generic question: 1..130 for the series, 1..10 for the question.
    const generic = document.createElement('p');
    generic.id = 'generic';
    body.appendChild(generic);
    const genericButton = document.createElement('input');
    genericButton.type = 'button';
    genericButton.value = 'új vegyes kérdés';
    genericButton.onclick = refreshGeneric;
    body.appendChild(genericButton);
    refreshGeneric();
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
