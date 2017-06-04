/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

var domready = require('domready');
var querystring = require('querystring-browser');
var request = require('browser-request');

function queryTurbo(query)
{
    var url = 'http://overpass-api.de/api/interpreter';

    request({'method' : 'POST', 'url' : url, 'body' : query, 'json' : true},
            function(er, response, body) {
                if (er)
                    throw er;

                var element = body['elements'][0];
                var city = element['tags']['addr:city'];
                var housenumber = element['tags']['addr:housenumber'];
                var postcode = element['tags']['addr:postcode'];
                var street = element['tags']['addr:street'];
                var addr =
                    postcode + ' ' + city + ', ' + street + ' ' + housenumber;

                // Have the address, now talk to nominatim to get the
                // coordinates as well.
                queryNominatim(addr, city, street, housenumber);
            });
}

function queryNominatim(addr, city, street, housenumber)
{
    var url = 'http://nominatim.openstreetmap.org/search.php?';
    url += querystring.stringify(
        {'q' : housenumber + ' ' + street + ', ' + city, 'format' : 'json'});
    request({'method' : 'GET', 'url' : url, 'json' : true},
            function(er, response, body) {
                if (er)
                    throw er;

                var element = body[0];
                var lat = element['lat'];
                var lon = element['lon'];

                // Show the result.
                var result = lat + ',' + lon + ' (' + addr + ')';
                var output = document.getElementById('output');
                output.value = result;
            });
}

function osmify()
{
    var url = document.getElementById('url-input').value;
    var tokens = url.split('/');
    // E.g. node or way.
    var objectType = tokens[tokens.length - 2];
    // Numeric ID.
    var objectId = tokens[tokens.length - 1];

    // Turn the ID into an address.
    var query = '[out:json];\n(\n    ' + objectType + '(' + objectId +
                ');\n);\nout body;';
    queryTurbo(query);
}

// Allow calling this from the button event handler.
window.osmify = osmify;

/// Look up name as a key in the query string.
function getParameterByName(name)
{
    name = name.replace(/[\[]/, '\\[').replace(/[\]]/, '\\]');
    var regex = new RegExp('[\\?&]' + name + '=([^&#]*)'),
        results = regex.exec(location.search);
    return results === null ? '' : results[1].replace(/\+/g, ' ');
}

domready(function() {
    // Allow pre-fill via GET parameters.
    var url = getParameterByName('url');
    if (url)
    {
        var urlInput = document.getElementById('url-input');
        urlInput.value = url;
    }

    // Create our page.
    var body = document.getElementsByTagName('body')[0];
    // TODO remaining contents
    var example = document.createElement('p');
    example.appendChild(document.createTextNode(
        'Example URL: http://www.openstreetmap.org/node/2700453924'));
    body.appendChild(example);
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
