/*
 * Copyright 2020 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

import domready = require('domready');

function getLoUrl(name: string)
{
    const loPrefix =
        'https://bugs.documentfoundation.org/buglist.cgi?keywords=regression%2C%20&keywords_type=allwords&longdesc=Adding%20Cc%3A%20to%20';
    const loSuffix =
        '&longdesc_type=substring&query_format=advanced&resolution=---';
    return loPrefix + encodeURI(name) + loSuffix;
}

function loJump()
{
    const nameElement = <HTMLInputElement>document.getElementById('name');
    const name = nameElement.value;
    document.location.href = getLoUrl(name);
}

function loShow()
{
    const nameElement = <HTMLInputElement>document.getElementById('name');
    const name = nameElement.value;
    nameElement.value = getLoUrl(name);
}

domready(function() {
    // Create our page.
    const body = document.getElementsByTagName('body')[0];
    const desc = document.createElement('p');
    desc.appendChild(document.createTextNode(
        'Takes a LibreOffice contributor name from public git and attempts to look up matching bisected bugs.'));
    body.appendChild(desc);

    const input = document.createElement('p');
    const nameInput = document.createElement('input');
    nameInput.id = 'name';
    nameInput.type = 'text';
    nameInput.placeholder = 'Name';
    nameInput.style.width = '50%';
    input.appendChild(nameInput);

    input.appendChild(document.createElement('br'));
    const linkButton = document.createElement('input');
    linkButton.type = 'button';
    linkButton.value = 'Link';
    linkButton.onclick = loShow;
    input.appendChild(linkButton);
    input.appendChild(document.createTextNode(' '));
    const jumpButton = document.createElement('input');
    jumpButton.type = 'button';
    jumpButton.value = 'Jump';
    jumpButton.onclick = loJump;
    input.appendChild(jumpButton);

    body.appendChild(input);

    const note = document.createElement('p');
    note.appendChild(document.createTextNode(
        'NOTE: the purpose of this page is to allow contributors to find badness before others do, not to put blame on them.'));
    body.appendChild(note);
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
