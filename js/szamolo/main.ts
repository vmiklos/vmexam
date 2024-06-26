/*
 * Copyright 2022 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

import confetti from 'canvas-confetti';

function isMedior()
{
    const urlParams = new URLSearchParams(window.location.search);
    const value = urlParams.get('medior');
    return value != null;
}

/**
 * Senior mode means "a" is 1..10, "b" is 1..10 and "c" is 0.
 */
function isSenior()
{
    const urlParams = new URLSearchParams(window.location.search);
    const value = urlParams.get('senior');
    const date = urlParams.get('20230429');
    return value != null || date != null;
}

function createSpan(label: string)
{
    const span = document.createElement("span");
    span.innerHTML = label;
    return span;
}

function checkAnswer()
{
    let a: number;
    if (isMedior())
    {
        a = 1;
    }
    else
    {
        const aSpan = document.getElementById("a");
        a = Number(aSpan.innerText);
    }
    const bSpan = document.getElementById("b");
    const b = Number(bSpan.innerText);
    const cSpan = document.getElementById("c");
    let c = 0;
    if (!isSenior())
    {
        c = Number(cSpan.innerText);
    }
    const retSpan = document.getElementById("ret");
    const ret = Number(retSpan.innerText);
    const scoreSpan = document.getElementById("score");
    let score = Number(scoreSpan.innerText);
    const origScore = score;
    if (a * b + c == ret)
    {
        score += 1;
    }
    else
    {
        score -= 1;
    }
    scoreSpan.innerText = score.toString();
    if (score > 0 && score > origScore && score % 5 == 0)
    {
        confetti({
            particleCount : 150,
            ticks : 600,
        });
    }

    // Generate next challenge.
    challenge();
}

function changeAnswer(delta: number)
{
    const retSpan = document.getElementById("ret");
    let ret = Number(retSpan.innerText);
    ret += delta;
    retSpan.innerText = ret.toString();
}

function incrementAnswer() { changeAnswer(1); }

function decrementAnswer() { changeAnswer(-1); }

function createLHS(p: HTMLParagraphElement)
{
    if (!isMedior())
    {
        const a = document.createElement("span");
        a.id = "a";
        p.appendChild(a);
        const multiply = document.createElement("span");
        multiply.innerText = " * ";
        p.appendChild(multiply);
    }
    const b = document.createElement("span");
    b.id = "b";
    p.appendChild(b);
    if (!isSenior())
    {
        const add = document.createElement("span");
        add.innerText = " + ";
        p.appendChild(add);
        const c = document.createElement("span");
        c.id = "c";
        p.appendChild(c);
    }
}

function createRHS(p: HTMLParagraphElement)
{
    const down = document.createElement("input");
    down.type = "button";
    down.style.verticalAlign = "middle";
    down.style.font = "50px sans-serif";
    down.id = "down";
    down.value = "▼";
    down.onclick = decrementAnswer;
    p.appendChild(down);

    p.appendChild(createSpan(" "));

    const ret = document.createElement("span");
    ret.id = "ret";
    p.appendChild(ret);

    p.appendChild(createSpan(" "));

    const up = document.createElement("input");
    up.type = "button";
    up.style.verticalAlign = "middle";
    up.style.font = "50px sans-serif";
    up.id = "up";
    up.value = "▲";
    up.onclick = incrementAnswer;
    p.appendChild(up);

    p.appendChild(createSpan(" "));

    const check = document.createElement("input");
    check.type = "button";
    check.style.verticalAlign = "middle";
    check.style.font = "50px sans-serif";
    check.id = "check";
    check.value = "✓";
    check.onclick = checkAnswer;
    p.appendChild(check);
}

function createPage()
{
    const body = document.getElementsByTagName("body")[0];
    const p = document.createElement("p");
    p.style.position = "fixed";
    p.style.font = "50px sans-serif";
    p.style.top = "25%";
    p.style.width = "100%";
    p.style.textAlign = "center";
    // Hundred points symbol.
    p.appendChild(createSpan("&#128175; "));
    const score = document.createElement("span");
    score.id = "score";
    score.innerText = " 0 ";
    p.appendChild(score);
    p.appendChild(document.createElement("br"));

    createLHS(p);
    const equals = document.createElement("span");
    equals.innerText = " = ";
    p.appendChild(equals);
    createRHS(p);
    body.appendChild(p);
}

// A random int between min and max, inclusive on both ends.
function randomIntFromInterval(min: number, max: number): number
{
    return Math.floor(Math.random() * (max - min + 1) + min);
}

function challenge()
{
    const limit = isMedior() ? 20 : 999;
    let a: number;
    let bMin = 1;
    let bMax: number;
    let cMax: number;
    let c: number;
    if (isMedior())
    {
        a = 1;
        bMax = limit / 2;
        cMax = limit / 2;
    }
    else
    {
        // E.g. 3 * 250 + 249 = limit
        const aSpan = document.getElementById("a");
        if (isSenior())
        {
            a = randomIntFromInterval(2, 9);
            aSpan.innerText = a.toString();
            bMin = 2;
            bMax = 9;
        }
        else
        {
            a = randomIntFromInterval(1, 3);
            aSpan.innerText = a.toString();
            bMax = limit / 4;
            cMax = limit / 4 - 1;
        }
    }
    const bSpan = document.getElementById("b");
    const b = randomIntFromInterval(bMin, bMax);
    bSpan.innerText = b.toString();
    if (isSenior())
    {
        c = 0;
    }
    else
    {
        const cSpan = document.getElementById("c");
        c = randomIntFromInterval(1, cMax);
        cSpan.innerText = c.toString();
    }
    const retSpan = document.getElementById("ret");
    let ret = a * b + c;
    ret += randomIntFromInterval(-4, 4);
    if (ret < 0)
    {
        ret = 0;
    }
    retSpan.innerText = ret.toString();
}

document.addEventListener("DOMContentLoaded", function() {
    createPage();
    challenge();
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
