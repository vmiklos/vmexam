/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

function shuffle(array: string[])
{
    let currentIndex = array.length, randomIndex;

    // While there remain elements to shuffle.
    while (currentIndex > 0)
    {

        // Pick a remaining element.
        randomIndex = Math.floor(Math.random() * currentIndex);
        currentIndex--;

        // And swap it with the current element.
        [array[currentIndex], array[randomIndex]] =
            [ array[randomIndex], array[currentIndex] ];
    }
}

async function refreshTopics()
{
    const topicList = [
        "élővilág", "zene", "földrajz", "sport", "nyelv és irodalom",
        "film és tv", "történelem és közélet", "életmód",
        "tudomány és technika", "bulvár"
    ];
    shuffle(topicList);
    const topics = <HTMLElement>document.querySelector('#topics');
    topics.innerText = topicList.join(", ");
}

// Both min and max are inclusive.
function getRandomInt(min: number, max: number): number
{
    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(Math.random() * (max - min + 1)) + min;
}

async function refreshSeries()
{
    const first = getRandomInt(1, 27);
    const second = getRandomInt(1, 10);
    const series = <HTMLElement>document.querySelector('#series');
    series.innerText = String(first) + ", " + String(second);
}

async function refreshGeneric()
{
    const first = getRandomInt(1, 130);
    const second = getRandomInt(1, 10);
    const generic = <HTMLElement>document.querySelector('#generic');
    generic.innerText = String(first) + ", " + String(second);
}

document.addEventListener("DOMContentLoaded", async function() {
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
