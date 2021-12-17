/*
 * Copyright 2021 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

import domready = require('domready');

declare global
{
    interface Window {
        wordList: Array<string>|undefined;
    }
}

// Need to figure out how to use the array-shuffle package...
function shuffle(array: Array<string>)
{
    let currentIndex = array.length, randomIndex;

    // While there remain elements to shuffle...
    while (currentIndex != 0)
    {

        // Pick a remaining element...
        randomIndex = Math.floor(Math.random() * currentIndex);
        currentIndex--;

        // And swap it with the current element.
        [array[currentIndex], array[randomIndex]] =
            [ array[randomIndex], array[currentIndex] ];
    }

    return array;
}

async function refreshClick()
{
    // Fetch word list if needed.
    if (window.wordList === undefined)
    {
        let request = new Request("szavak.json");
        const response = await window.fetch(request);
        window.wordList = await response.json();
    }

    // Pick a random word that contains only valid letters.
    window.wordList = shuffle(window.wordList);
    const wordElement = document.querySelector('#word');

    let valid = true;
    let valid_letters =
        [ 'a', 'á', 'e', 'i', 'í', 'm', 'o', 'ó', 'r', 't', 'u', 'ú', '=' ];
    for (let i = 0; i < window.wordList.length; i++)
    {
        let word = window.wordList[i];
        for (let j = 0; j < word.length; j++)
        {
            let letter = word[j];
            if (!valid_letters.includes(letter))
            {
                valid = false;
                break;
            }
        }
        if (valid && word.length >= 2)
        {
            wordElement.innerHTML = word.replace(/=/g, '-');
            break;
        }
        valid = true;
    }
}

domready(function() {
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
