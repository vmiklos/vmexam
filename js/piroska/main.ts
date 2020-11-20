/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

import 'ts-replace-all';
import domready = require('domready');

// Randomly reorder elements of `array`.
function shuffleArray(array: number[])
{
    for (let i = array.length - 1; i > 0; i--)
    {
        const j = Math.floor(Math.random() * (i + 1));
        [array[i], array[j]] = [ array[j], array[i] ];
    }
}

domready(function() {
    let text = `
{Actor04} bort és kalácsot visz a beteg {Actor11}. Útközben összetalálkozik {Actor22}, és az
rábeszéli, hogy virágot is vigyen. Míg {Actor00} virágot szed, {Actor20} elnyargal {Actor13},
megeszi és magára öltve hálóköntösét és hálósapkáját, befekszik az ágyába. Nemsokára megérkezik
{Actor00}, és megkérdezi, hogy miért olyan nagy a füle, szeme, szája. {Actor24} válaszai: a) azért,
hogy jobban halljalak, b) azért, hogy jobban lássalak, c) azért, hogy könnyebben bekaphassalak/vagy
felszólítja, hogy feküdjön mellé. {Actor04} nekikészülődik, de irtózik és kérdéseket tesz fel: miért
olyan nagy/vagy szőrös a lába, a keze, a szája stb., {Actor20} bekapja. Az arrajáró {Actor35}
meghallja {Actor20} erős horkolását, bemegy, felvágja a hasát, és {Actor00} és {Actor10} elevenen
kisétál belőle. A hasat megtöltik kaviccsal. {Actor24} elpusztul.
`;

    const actors = [
        [
            'Piroska', 'Piroskának', 'Piroskával', 'Piroskához', 'Piroska',
            'Piroska'
        ],
        [
            'a nagymama', 'nagymamának', 'a nagymamával', 'a nagymamához',
            'A nagymama', 'nagymama'
        ],
        [
            'a farkas', 'farkasnak', 'a farkassal', 'a farkashoz', 'A farkas',
            'farkas'
        ],
        [
            'a vadász', 'vadásznak', 'a vadásszal', 'a vadászhoz', 'A vadász',
            'vadász'
        ]
    ];

    // Maps original actors to actors in the current run.
    const currentActors: Array<number> = [];
    for (let i = 0; i < actors.length; i += 1)
    {
        currentActors.push(i);
    }

    while (true)
    {
        shuffleArray(currentActors);
        // Make sure that no elements stay at their original position.
        for (let i = 0; i < actors.length; i += 1)
        {
            if (currentActors[i] == i)
            {
                continue;
            }
        }
        break;
    }

    for (let actor = 0; actor < actors.length; actor += 1)
    {
        for (let variant = 0; variant < actors[0].length; variant += 1)
        {
            const fro = '{Actor' + actor.toString() + variant.toString() + '}';
            const to = actors[currentActors[actor]][variant];
            text = text.replaceAll(fro, to);
        }
    }

    // Create our page.
    const body = document.getElementsByTagName('body')[0];
    const paragraph = document.createElement('p');
    paragraph.appendChild(document.createTextNode(text));
    body.appendChild(paragraph);
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
