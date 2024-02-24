/*
 * Copyright 2024 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

function jumpButtonOnClick()
{
    const prefixElement = <HTMLInputElement>document.getElementById('prefix');
    const prefix = prefixElement.value;
    const suffixElement = <HTMLInputElement>document.getElementById('suffix');
    const suffix = suffixElement.value;
    const url = prefix + encodeURI(suffix);
    document.location.href = url;
}

document.addEventListener("DOMContentLoaded", function() {
    // Create our page.
    const body = document.getElementsByTagName('body')[0];

    const input = document.createElement('p');
    const prefixInput = document.createElement('input');
    prefixInput.id = 'prefix';
    prefixInput.type = 'text';
    prefixInput.value = 'https://git.libreoffice.org/core/commit/';
    prefixInput.style.width = '50%';
    input.appendChild(prefixInput);

    input.appendChild(document.createElement('br'));
    const suffixInput = document.createElement('input');
    suffixInput.id = 'suffix';
    suffixInput.type = 'text';
    suffixInput.placeholder = 'Git commit hash';
    suffixInput.style.width = '50%';
    input.appendChild(suffixInput);

    input.appendChild(document.createElement('br'));
    const jumpButton = document.createElement('input');
    jumpButton.type = 'button';
    jumpButton.value = 'Jump';
    jumpButton.onclick = jumpButtonOnClick;
    input.appendChild(jumpButton);

    body.appendChild(input);
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
