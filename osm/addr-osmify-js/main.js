/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

var domready = require('domready');
var querystring = require('querystring-browser');
var request = require('browser-request');

function queryTurbo(protocol, element)
{
    var output = document.getElementById('output');
    output.value = 'Using overpass-api...';

    var lat = element['lat'];
    var lon = element['lon'];
    var objectType = element['osm_type'];
    var objectId = element['osm_id'];
    var query = '[out:json];\n(\n    ' + objectType + '(' + objectId +
                ');\n);\nout body;';

    var url = protocol + '//overpass-api.de/api/interpreter';

    request({'method' : 'POST', 'url' : url, 'body' : query, 'json' : true},
            function(er, response, body) {
                var output = document.getElementById('output');
                if (er)
                {
                    output.value = 'Overpass error: ' + er;
                    return;
                }

                element = body['elements'][0];
                if (element == null)
                {
                    output.value = 'No results from overpass';
                    return;
                }

                var city = element['tags']['addr:city'];
                var housenumber = element['tags']['addr:housenumber'];
                var postcode = element['tags']['addr:postcode'];
                var street = element['tags']['addr:street'];
                var addr =
                    postcode + ' ' + city + ', ' + street + ' ' + housenumber;

                // Show the result.
                var result = 'geo:' + lat + ',' + lon + ' (' + addr + ')';
                output = document.getElementById('output');
                output.value = result;
            });
}

function queryNominatim(protocol, query, next)
{
    var output = document.getElementById('output');
    output.value = 'Using nominatim...';

    var url = protocol + '//nominatim.openstreetmap.org/search.php?';
    url += querystring.stringify({'q' : query, 'format' : 'json'});
    request({'method' : 'GET', 'url' : url, 'json' : true},
            function(er, response, elements) {
                var output = document.getElementById('output');
                if (er)
                {
                    output.value = 'Nominatim error: ' + er;
                    return;
                }

                if (elements.length == 0)
                {
                    output.value = 'No results from nominatim';
                    return;
                }

                if (elements.length > 1)
                {
                    // There are multiple elements, prefer buildings if
                    // possible.
                    var buildings = elements.filter(function(element) {
                        return element['class'] == 'building';
                    });
                    if (buildings.length > 0)
                        elements = buildings;
                }

                var element = elements[0];

                next(protocol, element);
            });
}

function osmify()
{
    var protocol = location.protocol != 'http:' ? 'https:' : 'http:';
    var query = document.getElementById('nominatim-input').value;

    // Use nominatim to get the coordinates and the osm type/id.
    // Next, use overpass to get the properties of the object.
    queryNominatim(protocol, query, queryTurbo);
}

/// Look up name as a key in the query string.
function getParameterByName(name)
{
    name = name.replace(/[\[]/, '\\[').replace(/[\]]/, '\\]');
    var regex = new RegExp('[\\?&]' + name + '=([^&#]*)'),
        results = regex.exec(location.search);
    return results === null ? '' : results[1].replace(/\+/g, ' ');
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
    var query = getParameterByName('query');
    if (query)
    {
        nominatimInput.value = decodeURIComponent(query);
    }

});

// vim: shiftwidth=4 softtabstop=4 expandtab:
