/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

export {};

interface Comment
{
    text: string;
    url: string;
}

declare global
{
    interface Window
    {
        commentList: Array<Comment>|undefined;
    }
}

// vim: shiftwidth=4 softtabstop=4 expandtab:
