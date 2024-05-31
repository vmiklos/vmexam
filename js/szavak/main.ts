/*
 * Copyright 2021 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

import arrayShuffle from 'array-shuffle';
import confetti from 'canvas-confetti';

declare global
{
    interface Window
    {
        wordList: Array<string>|undefined;
        lastConfetti: number;
        lastCount: number;
    }
}

async function refreshClick()
{
    const now = (+new Date()) / 1000;
    let jsonPath = "szavak.json";

    const urlParams = new URLSearchParams(window.location.search);
    let value = urlParams.get('path');
    if (value != null)
    {
        jsonPath = value;
    }
    value = urlParams.get('filter');
    const filter = value == null || Number(value) != 0;
    value = urlParams.get('counter');
    const counter = Number(value) != 0;
    let confettiTimeout = 600; // 10 minutes
    value = urlParams.get('confetti');
    if (value != null)
    {
        confettiTimeout = Number(value);
    }

    // Fetch word list if needed.
    if (window.wordList === undefined)
    {
        const request = new Request(jsonPath);
        const response = await window.fetch(request);
        window.wordList = await response.json();

        window.lastConfetti = now;
        window.lastCount = 0;
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
        'a', 'á', 'b', 'c', 'd', 'e', 'é', 'f', 'g', 'h', 'i',
        'í', 'j', 'k', 'l', 'm', 'n', 'o', 'ó', 'ö', 'ő', 'p',
        'r', 's', 't', 'u', 'ú', 'ü', 'ű', 'v', 'z', '='
    ];
    const y_prefixes: string[] = []; // could be e.g. 'g'.
    for (let i = 0; i < window.wordList.length; i++)
    {
        const word = window.wordList[i];
        for (let j = 0; j < word.length; j++)
        {
            const letter = word[j];
            // 'y' is invalid in general, but "gy" is fine.
            if (!valid_letters.includes(letter) &&
                !(letter == 'y' && j > 0 && y_prefixes.includes(word[j - 1])))
            {
                valid = false;
                break;
            }
        }
        if ((valid && word.length >= 2) || !filter)
        {
            let prefix = '';
            if (counter)
            {
                window.lastCount += 1;
                prefix = window.lastCount + '. ';
                const listElement =
                    <HTMLElement>document.querySelector('#list');
                listElement.innerText += "\n" + word;
            }
            wordElement.innerHTML = prefix + word.replace(/=/g, '-');
            wordElement.style.color = color;
            break;
        }
        valid = true;
    }

    if (now - window.lastConfetti > confettiTimeout)
    {
        confetti({
            particleCount : 150,
            ticks : 600,
        });
        window.lastConfetti = now;
    }
}

document.addEventListener("DOMContentLoaded", function() {
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

    const list = document.createElement('p');
    list.id = 'list';
    list.style.position = 'fixed';
    body.appendChild(list);

    // Show the initial word.
    refreshClick();
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
