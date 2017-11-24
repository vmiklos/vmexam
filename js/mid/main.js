/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

var domready = require('domready');

domready(function() {
    // Create our page.
    var body = document.getElementsByTagName('body')[0];
    var desc = document.createElement('p');
    desc.appendChild(document.createTextNode(
        "Takes a Message-ID from a public mailing list and attempts to look up a public archive entry for it."));
    body.appendChild(desc);
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
