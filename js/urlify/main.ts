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

function linkButtonOnClick()
{
    const prefixElement = <HTMLInputElement>document.getElementById('prefix');
    const prefix = prefixElement.value;
    const suffixElement = <HTMLInputElement>document.getElementById('suffix');
    const suffix = suffixElement.value;
    const url = prefix + encodeURI(suffix);
    suffixElement.value = url;
}

function createOption(id: string): HTMLElement
{
    const element = document.createElement('option');
    element.value = id;
    element.innerText = id;
    return element;
}

interface Option
{
    id: string;
    prefix: string;
    placeholder: string;
    note: string;
}
;

const options: Option[] = [
    {
        id : 'lo-core-commit',
        prefix : 'https://git.libreoffice.org/core/commit/',
        placeholder : 'Git commit hash',
        note :
            'This can be useful when viewing commit messages on mobile where running git-show from the cmdline is not easy.',
    },
    {
        id : 'lo-regression',
        prefix :
            'https://bugs.documentfoundation.org/buglist.cgi?f1=cf_regressionby&o1=equals&query_format=advanced&resolution=---&v1=',
        placeholder : 'Git author name',
        note :
            'The purpose of this page is to allow contributors to find badness before others do, not to put blame on them.',
    },
];

function selectOnChange()
{
    const selectElement = <HTMLSelectElement>document.querySelector('select');
    const selectedIndex = selectElement.selectedIndex;
    const option = options[selectedIndex];

    const prefixElement = <HTMLInputElement>document.getElementById('prefix');
    prefixElement.value = option.prefix;
    const suffixElement = <HTMLInputElement>document.getElementById('suffix');
    suffixElement.placeholder = option.placeholder;
    const noteElement = <HTMLInputElement>document.getElementById('note');
    noteElement.innerText = option.note;
}

document.addEventListener("DOMContentLoaded", function() {
    // Create our page.
    const body = document.getElementsByTagName('body')[0];

    const input = document.createElement('p');

    const selectElement = document.createElement('select');
    for (const option of options)
    {
        selectElement.appendChild(createOption(option.id));
    }
    selectElement.addEventListener("change", selectOnChange);
    input.appendChild(selectElement);
    input.appendChild(document.createElement('br'));

    const prefixInput = document.createElement('input');
    prefixInput.id = 'prefix';
    prefixInput.type = 'text';
    prefixInput.value = options[0].prefix;
    prefixInput.style.width = '50%';
    input.appendChild(prefixInput);

    input.appendChild(document.createElement('br'));
    const suffixInput = document.createElement('input');
    suffixInput.id = 'suffix';
    suffixInput.type = 'text';
    suffixInput.placeholder = options[0].placeholder;
    suffixInput.style.width = '50%';
    input.appendChild(suffixInput);

    input.appendChild(document.createElement('br'));
    const jumpButton = document.createElement('input');
    jumpButton.type = 'button';
    jumpButton.value = 'Jump';
    jumpButton.onclick = jumpButtonOnClick;
    input.appendChild(jumpButton);
    const linkButton = document.createElement('input');
    linkButton.type = 'button';
    linkButton.value = 'Link';
    linkButton.onclick = linkButtonOnClick;
    input.appendChild(linkButton);
    body.appendChild(input);

    const note = document.createElement('p');
    note.id = 'note';
    note.appendChild(document.createTextNode(options[0].note));
    body.appendChild(note);
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
