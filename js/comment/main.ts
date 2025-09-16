/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

export {}

interface Comment
{
    text: string;
    url: string;
}

declare global
{
    interface Window
    {
        commentList: Array<Comment> | undefined;
    }
}

async function refreshClick()
{
    // Fetch comment list if needed.
    if (window.commentList === undefined)
    {
        const jsonPath = "comments.json";
        const request = new Request(jsonPath);
        const response = await window.fetch(request);
        window.commentList = await response.json();
    }
    if (!window.commentList)
    {
        return;
    }

    // Pick a random comment.
    const index = Math.floor(Math.random() * window.commentList.length);
    const comment = window.commentList[index];
    const commentElement = document.querySelector('#comment') as HTMLElement;
    commentElement.innerText = comment.text;
    const permalinkElement = document.querySelector("#permalink") as HTMLAnchorElement;
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
