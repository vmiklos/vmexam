/*
 * Copyright 2019 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

function mailArchiveJump()
{
    const msgid = (<HTMLInputElement>document.getElementById('msgid')).value;
    document.location.href =
        'https://www.mail-archive.com/search?l=mid&q=' + msgid;
}

function debianJump()
{
    const msgid = (<HTMLInputElement>document.getElementById('msgid')).value;
    document.location.href = 'https://lists.debian.org/msgid-search/' + msgid;
}

function mailArchiveShow()
{
    const msgidElement = <HTMLInputElement>document.getElementById('msgid');
    const msgid = msgidElement.value;
    msgidElement.value = 'https://www.mail-archive.com/search?l=mid&q=' + msgid;
}

function debianShow()
{
    const msgidElement = <HTMLInputElement>document.getElementById('msgid');
    const msgid = msgidElement.value;
    msgidElement.value = 'https://lists.debian.org/msgid-search/' + msgid;
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
document.addEventListener("DOMContentLoaded", async function(event) {
    // Create our page.
    const body = document.getElementsByTagName('body')[0];
    const desc = document.createElement('p');
    desc.appendChild(document.createTextNode(
        'Takes a Message-ID from a public mailing list and attempts to look up a public archive entry for it from multiple sources.'));
    body.appendChild(desc);

    const input = document.createElement('p');
    const msgidInput = document.createElement('input');
    msgidInput.id = 'msgid';
    msgidInput.type = 'text';
    msgidInput.placeholder = 'Message-ID';
    msgidInput.style.width = '50%';
    input.appendChild(msgidInput);

    input.appendChild(document.createElement('br'));
    const mailArchiveLink = document.createElement('input');
    mailArchiveLink.type = 'button';
    mailArchiveLink.value = 'Mail Archive Link';
    mailArchiveLink.onclick = mailArchiveShow;
    input.appendChild(mailArchiveLink);
    input.appendChild(document.createTextNode(' '));
    const debianLink = document.createElement('input');
    debianLink.type = 'button';
    debianLink.value = 'Debian Link';
    debianLink.onclick = debianShow;
    input.appendChild(debianLink);

    input.appendChild(document.createElement('br'));
    const mailArchive = document.createElement('input');
    mailArchive.type = 'button';
    mailArchive.value = 'Mail Archive Jump';
    mailArchive.onclick = mailArchiveJump;
    input.appendChild(mailArchive);
    input.appendChild(document.createTextNode(' '));
    const debian = document.createElement('input');
    debian.type = 'button';
    debian.value = 'Debian Jump';
    debian.onclick = debianJump;
    input.appendChild(debian);

    body.appendChild(input);

    const maExample = document.createElement('p');
    maExample.appendChild(document.createTextNode(
        'Example Mail Archive MessageID: cfb1d155-499d-3205-8283-ce84c39dbb14@redhat.com'));
    body.appendChild(maExample);

    const dExample = document.createElement('p');
    dExample.appendChild(document.createTextNode(
        'Example Debian MessageID: 20171121214924.kwjklln5t6t7dedh@rene-engelhard.de'));
    body.appendChild(dExample);
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
