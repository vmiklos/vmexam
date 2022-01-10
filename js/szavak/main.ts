/*
 * Copyright 2021 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

import arrayShuffle from 'array-shuffle';

declare global
{
    interface Window {
        wordList: Array<string>|undefined;
    }
}

async function refreshClick()
{
    let jsonPath = "2022-01-04.json";

    const urlParams = new URLSearchParams(window.location.search);
    const path = urlParams.get('path');
    if (path != null)
    {
        jsonPath = path;
    }
    const filter = urlParams.get('filter') != null;

    // Fetch word list if needed.
    if (window.wordList === undefined)
    {
        const request = new Request(jsonPath);
        const response = await window.fetch(request);
        window.wordList = await response.json();
    }

    // Decide what color to use.
    const colors =
        [ '#4472C4', '#ED7D31', '#A5A5A5', '#FFC000', '#5B9BD5', '#70AD47' ];
    const color = colors[Math.floor(Math.random() * colors.length)];

    // Pick a random word that contains only valid letters.
    window.wordList = arrayShuffle(window.wordList);
    const wordElement = <HTMLElement>document.querySelector('#word');

    let valid = true;
    const valid_letters = [
        'a', 'á', 'e', 'é', 'i', 'í', 'l', 'm', 'o', 'ó', 'r', 't', 'u', 'ú',
        'ü', 'ű', '='
    ];
    for (let i = 0; i < window.wordList.length; i++)
    {
        const word = window.wordList[i];
        for (let j = 0; j < word.length; j++)
        {
            const letter = word[j];
            if (!valid_letters.includes(letter))
            {
                valid = false;
                break;
            }
        }
        if ((valid && word.length >= 2) || !filter)
        {
            wordElement.innerHTML = word.replace(/=/g, '-');
            wordElement.style.color = color;
            break;
        }
        valid = true;
    }
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
document.addEventListener("DOMContentLoaded", async function(event) {
    // Create our page.
    const body = document.getElementsByTagName('body')[0];
    const word = document.createElement('p');
    word.id = 'word';
    word.style.position = 'fixed';
    word.style.font = '100px sans-serif';
    word.style.top = '25%';
    word.style.left = '50%';
    word.style.transform = 'translate(-50%, -50%)';
    body.appendChild(word);

    const refresh = document.createElement('img');
    refresh.style.position = 'fixed';
    refresh.style.top = '75%';
    refresh.style.left = '50%';
    refresh.style.width = '100px';
    refresh.style.height = '100px';
    refresh.style.transform = 'translate(-50%, -50%)';
    refresh.style.cursor = 'pointer';
    refresh.src = 'refresh.svg';
    refresh.onclick = refreshClick;
    body.appendChild(refresh);

    // Show the initial word.
    refreshClick();
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
