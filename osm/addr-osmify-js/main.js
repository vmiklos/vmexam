/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

var domready = require('domready');

async function queryTurbo(protocol, element)
{
    var output = document.getElementById('output');
    output.value = 'Using overpass-api...';

    var lat = element.lat;
    var lon = element.lon;
    var objectType = element.osm_type;
    var objectId = element.osm_id;
    var query = '[out:json];\n(\n    ' + objectType + '(' + objectId +
                ');\n);\nout body;';

    var url = protocol + '//overpass-api.de/api/interpreter';

    var request = new Request(url, {method : 'POST', body : query});
    try
    {
        var response = await window.fetch(request);
        var body = await response.json();

        var element = body.elements[0];
        if (element == null)
        {
            output.value = 'No results from overpass';
            return;
        }

        var city = element.tags['addr:city'];
        var housenumber = element.tags['addr:housenumber'];
        var postcode = element.tags['addr:postcode'];
        var street = element.tags['addr:street'];
        var addr = postcode + ' ' + city + ', ' + street + ' ' + housenumber;

        // Show the result.
        var result = 'geo:' + lat + ',' + lon + ' (' + addr + ')';
        output.value = result;
    }
    catch (reason)
    {
        var output = document.getElementById('output');
        output.value = 'Overpass error: ' + reason;
        return;
    }
}

async function queryNominatim(protocol, query, next)
{
    var output = document.getElementById('output');
    output.value = 'Using nominatim...';

    let url = protocol + '//nominatim.openstreetmap.org/search.php?';
    var urlParams = new URLSearchParams();
    urlParams.append('q', query);
    urlParams.append('format', 'json');
    url += urlParams.toString();
    var request = new Request(url, {method : 'GET'});
    try
    {
        var response = await window.fetch(request);
        let elements = await response.json();
        var output = document.getElementById('output');

        if (elements.length == 0)
        {
            output.value = 'No results from nominatim';
            return;
        }

        if (elements.length > 1)
        {
            // There are multiple elements, prefer buildings if
            // possible.
            var buildings = elements.filter(function(
                element) { return element['class'] == 'building'; });
            if (buildings.length > 0)
                elements = buildings;
        }

        return elements[0];
    }
    catch (reason)
    {
        var output = document.getElementById('output');
        output.value = 'Nominatim error: ' + reason;
    };
}

async function osmify()
{
    var protocol = location.protocol != 'http:' ? 'https:' : 'http:';
    var nominatimInput = document.getElementById('nominatim-input');
    var query = nominatimInput.value;

    // Use nominatim to get the coordinates and the osm type/id.
    var nominatimResult = await queryNominatim(protocol, query);

    // Use overpass to get the properties of the object.
    queryTurbo(protocol, nominatimResult);
}

domready(function() {
    // Create our page.
    var body = document.getElementsByTagName('body')[0];

    var desc = document.createElement('p');
    desc.appendChild(document.createTextNode(
        'Takes an nominatim query and turns it into a string that is readable (so that you can save it to your contacts) and is also machine-friendly, e.g. OsmAnd can parse it as well.'));
    body.appendChild(desc);

    var input = document.createElement('p');
    var nominatimInput = document.createElement('input');
    nominatimInput.id = 'nominatim-input';
    nominatimInput.type = 'text';
    nominatimInput.placeholder = 'Query';
    nominatimInput.size = 64;
    input.appendChild(nominatimInput);
    input.appendChild(document.createTextNode(' '));
    var button = document.createElement('input');
    button.type = 'button';
    button.value = 'osmify';
    button.onclick = osmify;
    input.appendChild(button);
    body.appendChild(input);

    var result = document.createElement('p');
    result.appendChild(document.createTextNode('Result: '));
    var output = document.createElement('input');
    output.id = 'output';
    output.type = 'text';
    output.size = 64;
    result.appendChild(output);
    body.appendChild(result);

    var example = document.createElement('p');
    example.appendChild(
        document.createTextNode('Example query: Mészáros utca 58/a, Budapest'));
    body.appendChild(example);

    // Allow pre-fill via GET parameters.
    var query = new URLSearchParams(window.location.search).get('query');
    if (query)
    {
        nominatimInput.value = decodeURIComponent(query);
    }
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
