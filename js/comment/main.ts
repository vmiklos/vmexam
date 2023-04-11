/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

async function refreshClick()
{
    const jsonPath = "comments.json";

    // Fetch comment list if needed.
    if (window.commentList === undefined)
    {
        const request = new Request(jsonPath);
        const response = await window.fetch(request);
        window.commentList = await response.json();
    }

    // Pick a random comment.
    const index = Math.floor(Math.random() * window.commentList.length);
    const comment = window.commentList[index];
    const commentElement = <HTMLElement>document.querySelector('#comment');
    commentElement.innerText = comment.text;
    const permalinkElement =
        <HTMLAnchorElement>document.querySelector('#permalink');
    permalinkElement.href = comment.url;
}

document.addEventListener("DOMContentLoaded", async function() {
    // Create our page.
    const body = document.getElementsByTagName('body')[0];
    const comment = document.createElement('p');
    comment.id = 'comment';
    body.appendChild(comment);

    const footer = document.createElement('p');
    body.appendChild(footer);
    const permalink = document.createElement('a');
    permalink.id = 'permalink';
    permalink.innerText = 'Permalink';
    footer.appendChild(permalink);
    const space = document.createElement('span');
    space.innerText = ' ';
    footer.appendChild(space);
    const refresh = document.createElement('input');
    refresh.type = 'button';
    refresh.value = 'Refresh';
    refresh.onclick = refreshClick;
    footer.appendChild(refresh);

    // Show the initial comment.
    refreshClick();
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
