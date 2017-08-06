/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

var domready = require('domready');
var querystring = require('querystring-browser');
var request = require('browser-request');

function queryTurbo(protocol, query)
{
    var url = protocol + '//overpass-api.de/api/interpreter';

    request({'method' : 'POST', 'url' : url, 'body' : query, 'json' : true},
            function(er, response, body) {
                if (er)
                {
                    var output = document.getElementById('output');
                    output.value = 'Overpass error: ' + er;
                    return;
                }

                var element = body['elements'][0];
                var city = element['tags']['addr:city'];
                var housenumber = element['tags']['addr:housenumber'];
                var postcode = element['tags']['addr:postcode'];
                var street = element['tags']['addr:street'];
                var addr =
                    postcode + ' ' + city + ', ' + street + ' ' + housenumber;

                // Have the address, now talk to nominatim to get the
                // coordinates as well.
                queryNominatim(protocol, addr, city, street, housenumber);
            });
}

function queryNominatim(protocol, addr, city, street, housenumber)
{
    var url = protocol + '//nominatim.openstreetmap.org/search.php?';
    url += querystring.stringify(
        {'q' : housenumber + ' ' + street + ', ' + city, 'format' : 'json'});
    request({'method' : 'GET', 'url' : url, 'json' : true},
            function(er, response, body) {
                if (er)
                {
                    var output = document.getElementById('output');
                    output.value = 'Nominatim error: ' + er;
                    return;
                }

                var element = body[0];
                var lat = element['lat'];
                var lon = element['lon'];

                // Show the result.
                var result = lat + ',' + lon + ' (' + addr + ')';
                output = document.getElementById('output');
                output.value = result;
            });
}

function osmify()
{
    var output = document.getElementById('output');
    output.value = 'Please wait...';

    var url = document.getElementById('url-input').value;
    var tokens = url.split('/');
    // E.g. node or way.
    var objectType = tokens[tokens.length - 2];
    // Numeric ID.
    var objectId = tokens[tokens.length - 1];

    // Turn the ID into an address.
    var protocol = location.protocol != 'https:' ? 'http:' : 'https:';
    var query = '[out:json];\n(\n    ' + objectType + '(' + objectId +
                ');\n);\nout body;';
    queryTurbo(protocol, query);
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
        'Takes an OSM object ID and turns it into a string that is readable (so that you can save it to your contacts) and is also machine-friendly, e.g. OsmAnd can parse it as well.'));
    body.appendChild(desc);

    var input = document.createElement('p');
    var urlInput = document.createElement('input');
    urlInput.id = 'url-input';
    urlInput.type = 'text';
    urlInput.placeholder = 'URL';
    urlInput.size = 64;
    input.appendChild(urlInput);
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
    example.appendChild(document.createTextNode(
        'Example URL: http://www.openstreetmap.org/node/2700453924'));
    body.appendChild(example);

    // Allow pre-fill via GET parameters.
    var url = getParameterByName('url');
    if (url)
    {
        urlInput.value = url;
    }

});

// vim: shiftwidth=4 softtabstop=4 expandtab:
