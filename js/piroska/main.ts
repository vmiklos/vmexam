/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

import 'ts-replace-all';
import domready = require('domready');

/// Picks a random element form the array.
function choose(array: Array<number>): number
{
    return array[Math.floor(Math.random() * array.length)];
}

domready(function() {
    let text = '';
    text +=
        '{Actor04} bort és kalácsot visz a beteg {Actor11}. Útközben összetalálkozik ';
    text +=
        '{Actor22}, és az rábeszéli, hogy virágot is vigyen. Míg {Actor00} virágot szed, ';
    text +=
        '{Actor20} elnyargal {Actor13}, megeszi és magára öltve hálóköntösét és ';
    text +=
        'hálósapkáját, befekszik az ágyába. Nemsokára megérkezik {Actor00}, és ';
    text +=
        'megkérdezi, hogy miért olyan nagy a füle, szeme, szája. {Actor24} válaszai: a) ';
    text +=
        'azért, hogy jobban halljalak, b) azért, hogy jobban lássalak, c) azért, hogy ';
    text +=
        'könnyebben bekaphassalak/vagy felszólítja, hogy feküdjön mellé. {Actor04} ';
    text +=
        'nekikészülődik, de irtózik és kérdéseket tesz fel: miért olyan nagy/vagy szőrös ';
    text +=
        'a lába, a keze, a szája stb., {Actor20} bekapja. Az arrajáró {Actor35} ';
    text +=
        'meghallja {Actor20} erős horkolását, bemegy, felvágja a hasát, és {Actor00} és ';
    text +=
        '{Actor10} elevenen kisétál belőle. A hasat megtöltik kaviccsal. {Actor24} ';
    text += 'elpusztul.';

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

    for (let actor = 0; actor < actors.length; actor += 1)
    {
        const choices = [];
        for (let i = 0; i < actors.length; i += 1)
        {
            if (i != actor && !currentActors.includes(i))
            {
                choices.push(i);
            }
        }
        // TODO: if set 0 to 1, 1 to 2, 2 to 0, then 3 will blow up: it can only
        // be 3, but it also wants be something other than 3.
        const choice = choose(choices);
        currentActors.push(choice);
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
