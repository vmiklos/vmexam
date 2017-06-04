/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

var domready = require('domready');

function osmify() { var url = document.getElementById('url-input').value; }

// Allow calling this from the button event handler.
window.osmify = osmify;

domready(function() {});

// vim: shiftwidth=4 softtabstop=4 expandtab:
