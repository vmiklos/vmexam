/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

var domready = require('domready');
var shuffle = require('shuffle-array');

String.prototype.replaceAll = function(search, replacement) {
    var target = this;
    return target.replace(new RegExp(search, 'g'), replacement);
};

domready(function() {
    var text = '';
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

    var actors = [
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
    // TODO duplication
    var currentActors = [ 0, 1, 2, 3 ];

    shuffle(currentActors);

    for (var actor = 0; actor < actors.length; actor += 1)
    {
        for (var variant = 0; variant < actors[0].length; variant += 1)
        {
            var fro = '{Actor' + actor.toString() + variant.toString() + '}';
            var to = actors[currentActors[actor]][variant];
            text = text.replaceAll(fro, to);
        }
    }

    // Create our page.
    var body = document.getElementsByTagName('body')[0];
    var paragraph = document.createElement('p');
    paragraph.appendChild(document.createTextNode(text));
    body.appendChild(paragraph);
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
