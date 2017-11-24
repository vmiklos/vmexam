/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

var domready = require('domready');

function mailArchiveClick()
{
    var msgid = document.getElementById('msgid').value;
    document.location.href =
        'https://www.mail-archive.com/search?l=mid&q=' + msgid;
}

function debianClick()
{
    var msgid = document.getElementById('msgid').value;
    document.location.href = 'https://lists.debian.org/msgid-search/' + msgid;
}

domready(function() {
    // Create our page.
    var body = document.getElementsByTagName('body')[0];
    var desc = document.createElement('p');
    desc.appendChild(document.createTextNode(
        'Takes a Message-ID from a public mailing list and attempts to look up a public archive entry for it from multiple sources.'));
    body.appendChild(desc);

    var input = document.createElement('p');
    var msgidInput = document.createElement('input');
    msgidInput.id = 'msgid';
    msgidInput.type = 'text';
    msgidInput.placeholder = 'Message-ID';
    msgidInput.style.width = '50%';
    input.appendChild(msgidInput);
    input.appendChild(document.createElement('br'));
    var mailArchive = document.createElement('input');
    mailArchive.type = 'button';
    mailArchive.value = 'Mail Archive';
    mailArchive.onclick = mailArchiveClick;
    input.appendChild(mailArchive);
    input.appendChild(document.createTextNode(' '));
    var debian = document.createElement('input');
    debian.type = 'button';
    debian.value = 'Debian';
    debian.onclick = debianClick;
    input.appendChild(debian);
    body.appendChild(input);

    var maExample = document.createElement('p');
    maExample.appendChild(document.createTextNode(
        'Example Mail Archive MessageID: cfb1d155-499d-3205-8283-ce84c39dbb14@redhat.com'));
    body.appendChild(maExample);

    var dExample = document.createElement('p');
    dExample.appendChild(document.createTextNode(
        'Example Debian MessageID: 20171121214924.kwjklln5t6t7dedh@rene-engelhard.de'));
    body.appendChild(dExample);
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
