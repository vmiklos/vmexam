/*
 * Copyright 2024 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

function jumpButtonOnClick()
{
    const urlElement = document.getElementById('url') as HTMLInputElement;
    const template = urlElement.value;
    const suffixElement = document.getElementById('suffix') as HTMLInputElement;
    const suffix = suffixElement.value;
    const url = template.replace('{0}', encodeURI(suffix));
    document.location.href = url;
}

function linkButtonOnClick()
{
    const urlElement = document.getElementById('url') as HTMLInputElement;
    const template = urlElement.value;
    const suffixElement = document.getElementById('suffix') as HTMLInputElement;
    const suffix = suffixElement.value;
    const url = template.replace('{0}', encodeURI(suffix));
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
    url: string;
    placeholder: string;
    note: string;
}

const options: Option[] = [
    {
        // test data: 1f471a24efef039fdfff161f910142fdc6bb58dd
        id : 'cool-commit',
        url : 'https://gerrit.collaboraoffice.com/plugins/gitiles/online/+/{0}^!/',
        placeholder : 'Git commit hash',
        note :
            'This can be useful when viewing commit messages on mobile where running git-show from the cmdline is not easy.',
    },
    {
        // test data: Idc7ce9c2c659e619f748c6cc62d7e29032cd9a86
        id : 'cool-change',
        url :
            'https://gerrit.collaboraoffice.com/q/{0}',
        placeholder : 'Gerrit change ID',
        note :
            'This can be useful when viewing commit messages on mobile where running git-log from the cmdline is not easy.',
    },
    {
        id : 'lo-core-commit',
        url : 'https://git.libreoffice.org/core/commit/{0}',
        placeholder : 'Git commit hash',
        note :
            'This can be useful when viewing commit messages on mobile where running git-show from the cmdline is not easy.',
    },
    {
        // test data: I5e494a0714e398221bee00744d7e25c419a41df7
        id : 'lo-core-change',
        url : 'https://gerrit.libreoffice.org/q/{0}',
        placeholder : 'Gerrit change ID',
        note :
            'This can be useful when viewing commit messages on mobile where running git-log from the cmdline is not easy.',
    },
    {
        id : 'lo-regression',
        url :
            'https://bugs.documentfoundation.org/buglist.cgi?f1=cf_regressionby&o1=equals&query_format=advanced&resolution=---&v1={0}',
        placeholder : 'Git author name',
        note :
            'The purpose of this page is to allow contributors to find badness before others do, not to put blame on them.',
    },
    {
        // test data: cfb1d155-499d-3205-8283-ce84c39dbb14@redhat.com
        id : 'mail-archive',
        url : 'https://www.mail-archive.com/search?l=mid&q={0}',
        placeholder : 'Message-Id',
        note :
            'Turns an email Message-Id header into a URL for many public mailing lists.',
    },
];

function selectOnChange()
{
    const selectElement = document.querySelector('select') as HTMLSelectElement;
    const selectedIndex = selectElement.selectedIndex;
    const option = options[selectedIndex];

    const urlElement = document.getElementById('url') as HTMLInputElement;
    urlElement.value = option.url;
    const suffixElement = document.getElementById('suffix') as HTMLInputElement;
    suffixElement.placeholder = option.placeholder;
    const noteElement = document.getElementById('note') as HTMLInputElement;
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

    const urlInput = document.createElement('input');
    urlInput.id = 'url';
    urlInput.type = 'text';
    urlInput.value = options[0].url;
    urlInput.style.width = '50%';
    input.appendChild(urlInput);

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
