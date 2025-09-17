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
    for (let i = 0; i < topicList.length; i++)
    {
        const topic = document.querySelector('#topic' + i) as HTMLElement;
        topic.innerText = topicList[i];
    }
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
    const series = document.querySelector('#seriesId') as HTMLElement;
    const seriesQuestion = document.querySelector("#seriesQuestionId") as HTMLElement;
    series.innerText = String(first);
    seriesQuestion.innerText = String(second);
}

function refreshGeneric()
{
    const first = getRandomInt(1, 130);
    const second = getRandomInt(1, 10);
    const generic = document.querySelector('#genericId') as HTMLElement;
    const genericQuestion = document.querySelector("#genericQuestionId") as HTMLElement;
    generic.innerText = String(first);
    genericQuestion.innerText = String(second);
}

document.addEventListener("DOMContentLoaded", function() {
    // Topics.
    const topicsButton = document.querySelector('#topicsButton') as HTMLElement;
    topicsButton.onclick = refreshTopics;
    refreshTopics();

    // Random topic question: 1..27 for the series, 1..10 for the question.
    const seriesButton = document.querySelector('#seriesButton') as HTMLElement;
    seriesButton.onclick = refreshSeries;
    refreshSeries();

    // Random generic question: 1..130 for the series, 1..10 for the question.
    const genericButton = document.querySelector('#genericButton') as HTMLElement;
    genericButton.onclick = refreshGeneric;
    refreshGeneric();
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
