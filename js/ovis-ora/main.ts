/*
 * Copyright 2021 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

import confetti = require('canvas-confetti');
import domready = require('domready');

async function drawClock(ratio: number)
{
    const canvas = <HTMLCanvasElement>document.getElementById('mycanvas');
    const ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, 1000, 1000);
    ctx.fillStyle = 'rgb(0, 255, 0)';
    ctx.fillRect(0, 0, 1000 * ratio, 1000);
    ctx.fillStyle = 'rgb(255, 0, 0)';
    ctx.fillRect(1000 * ratio, 0, 1000 - 1000 * ratio, 1000);
}

async function submitClick()
{
    const durationElement =
        <HTMLInputElement>document.querySelector('#duration');
    durationElement.style.display = 'none';
    let duration = Number(durationElement.value);
    const submitElement = <HTMLButtonElement>document.querySelector('#submit');
    submitElement.style.display = 'none;';
    const canvas = <HTMLCanvasElement>document.getElementById('mycanvas');
    canvas.style.display = 'inline';
    duration *= 60;
    for (let i = 1; i <= duration; i++)
    {
        drawClock(i / duration);
        await new Promise(r => setTimeout(r, 1000));
    }

    // Full green: end text.
    const body = document.getElementsByTagName('body')[0];
    const end = document.createElement('p');
    end.style.zIndex = "1";
    end.style.position = "fixed";
    end.style.font = '100px sans-serif';
    end.style.top = '50%';
    end.style.left = '50%';
    end.style.transform = 'translate(-50%, -50%)';
    end.appendChild(document.createTextNode('Vége'));
    body.appendChild(end);
    confetti({
        particleCount : 150,
        ticks : 600,
    });
}

domready(function() {
    // Create our page.
    const body = document.getElementsByTagName('body')[0];
    const canvas = document.createElement('canvas');
    canvas.id = 'mycanvas';
    canvas.width = 1000;
    canvas.height = 1000;
    canvas.style.position = 'fixed';
    canvas.style.left = '0';
    canvas.style.top = '0'
    canvas.style.width = '100%';
    canvas.style.height = '100%';
    canvas.style.display = 'none';
    body.appendChild(canvas);

    const duration = document.createElement('input');
    duration.id = 'duration';
    duration.type = 'text';
    duration.placeholder = 'Perc';
    duration.style.width = '50%';
    body.appendChild(duration);

    const submit = document.createElement('input');
    submit.id = 'submit';
    submit.type = 'button';
    submit.value = 'OK';
    submit.onclick = submitClick;
    body.appendChild(submit);
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
