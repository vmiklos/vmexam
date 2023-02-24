/*
 * Copyright 2023 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
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
