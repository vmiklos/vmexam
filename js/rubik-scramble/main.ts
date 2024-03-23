/*
 * Copyright 2024 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

interface RubikResult
{
    ok: string;
    error: string;
}

document.addEventListener("DOMContentLoaded", async function() {
    const body = document.querySelector('body');
    const request =
        new Request('https://share.vmiklos.hu/apps/rubik-scramble/');
    const response = await window.fetch(request);
    const result = await<Promise<RubikResult>>response.json();
    const pre = document.createElement('pre');
    if (result.error === '')
    {
        pre.innerText = result.ok;
    }
    else
    {
        pre.innerText = 'Error: ' + result.error;
    }
    body.appendChild(pre);

    const help = document.createElement('p');
    help.appendChild(document.createTextNode('See '));
    const a = document.createElement('a');
    a.href = 'https://meep.cubing.net/wcanotation.html';
    a.innerText = 'WCA Notation';
    help.appendChild(a);
    help.appendChild(document.createTextNode(' for help on face turns.'));
    body.appendChild(help);
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
