/*
 * Copyright 2020 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

import domready = require('domready');

function queryTurbo(protocol: string, element: {[index: string]: any})
{
    const output = <HTMLInputElement>document.getElementById('output');
    output.value = 'Using overpass-api...';

    const lat = element['lat'];
    const lon = element['lon'];
    const objectType = element['osm_type'];
    const objectId = element['osm_id'];
    const query = '[out:json];\n(\n    ' + objectType + '(' + objectId +
                  ');\n);\nout body;';

    const url = protocol + '//overpass-api.de/api/interpreter';

    const request = new Request(url, {method : 'POST', body : query});
    window.fetch(request)
        .then(response => response.json())
        .then(body => {
            let output = <HTMLInputElement>document.getElementById('output');

            element = body['elements'][0];
            if (element == null)
            {
                output.value = 'No results from overpass';
                return;
            }

            const city = element['tags']['addr:city'];
            const housenumber = element['tags']['addr:housenumber'];
            const postcode = element['tags']['addr:postcode'];
            const street = element['tags']['addr:street'];
            const addr =
                postcode + ' ' + city + ', ' + street + ' ' + housenumber;

            // Show the result.
            const result = 'geo:' + lat + ',' + lon + ' (' + addr + ')';
            output = <HTMLInputElement>document.getElementById('output');
            output.value = result;
        })
        .catch(reason => {
            const output = <HTMLInputElement>document.getElementById('output');
            output.value = 'Overpass error: ' + reason;
            return;
        });
}

interface Next {
    (protocol: string, element: {[index: string]: any}): void
}

function getJsonArray(request: Request):
    Promise<any[]>{return <Promise<any>>fetch(request).then(res => res.json())}

function queryNominatim(protocol: string, query: string, next: Next) {
    const output = <HTMLInputElement>document.getElementById('output');
    output.value = 'Using nominatim...';

    let url = protocol + '//nominatim.openstreetmap.org/search.php?';
    const urlParams = new URLSearchParams();
    urlParams.append('q', query);
    urlParams.append('format', 'json');
    url += urlParams.toString();
    const request = new Request(url, {method : 'GET'});
    getJsonArray(request)
        .then(elements => {
            const output = <HTMLInputElement>document.getElementById('output');

            if (elements.length == 0)
            {
                output.value = 'No results from nominatim';
                return;
            }

            if (elements.length > 1)
            {
                // There are multiple elements, prefer buildings if
                // possible.
                const buildings =
                    elements.filter(function(element: {[index: string]: any}) {
                        return element['class'] == 'building';
                    });
                if (buildings.length > 0)
                    elements = buildings;
            }

            const element = elements[0];

            next(protocol, element);
        })
        .catch(reason => {
            const output = <HTMLInputElement>document.getElementById('output');
            output.value = 'Nominatim error: ' + reason;
        });
}

function
osmify() {
    const protocol = location.protocol != 'http:' ? 'https:' : 'http:';
    const nominatimInput =
        <HTMLInputElement>document.getElementById('nominatim-input');
    const query = nominatimInput.value;

    // Use nominatim to get the coordinates and the osm type/id.
    // Next, use overpass to get the properties of the object.
    queryNominatim(protocol, query, queryTurbo);
}

domready(function() {
    // Create our page.
    const body = document.getElementsByTagName('body')[0];

    const desc = document.createElement('p');
    desc.appendChild(document.createTextNode(
        'Takes an nominatim query and turns it into a string that is readable (so that you can save it to your contacts) and is also machine-friendly, e.g. OsmAnd can parse it as well.'));
    body.appendChild(desc);

    const input = document.createElement('p');
    const nominatimInput = document.createElement('input');
    nominatimInput.id = 'nominatim-input';
    nominatimInput.type = 'text';
    nominatimInput.placeholder = 'Query';
    nominatimInput.size = 64;
    input.appendChild(nominatimInput);
    input.appendChild(document.createTextNode(' '));
    const button = document.createElement('input');
    button.type = 'button';
    button.value = 'osmify';
    button.onclick = osmify;
    input.appendChild(button);
    body.appendChild(input);

    const result = document.createElement('p');
    result.appendChild(document.createTextNode('Result: '));
    const output = document.createElement('input');
    output.id = 'output';
    output.type = 'text';
    output.size = 64;
    result.appendChild(output);
    body.appendChild(result);

    const example = document.createElement('p');
    example.appendChild(
        document.createTextNode('Example query: Mészáros utca 58/a, Budapest'));
    body.appendChild(example);

    // Allow pre-fill via GET parameters.
    const query = new URLSearchParams(window.location.search).get('query');
    if (query)
    {
        nominatimInput.value = decodeURIComponent(query);
    }
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
