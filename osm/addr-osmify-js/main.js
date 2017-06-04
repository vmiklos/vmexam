/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

var domready = require('domready');

function osmify() {}

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
    var url = getParameterByName('url');
    if (url)
    {
        var urlInput = document.getElementById('url-input');
        urlInput.value = url;
    }
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
