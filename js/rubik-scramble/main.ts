/*
 * Copyright 2024 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

// Both min and max are inclusive.
function getRandomInt(min: number, max: number): number
{
    return Math.floor(Math.random() * (max - min + 1)) + min;
}

document.addEventListener("DOMContentLoaded", function() {
    const cells = [];
    const table = <HTMLElement>document.querySelector('#scramble');
    table.style.border = '1px solid';
    table.style.borderCollapse = 'collapse';
    for (let row = 0; row < 2; row++)
    {
        const tr = document.createElement('tr');
        table.appendChild(tr);
        for (let col = 0; col < 3; col++)
        {
            const td = document.createElement('td');
            td.style.border = '1px solid';
            tr.appendChild(td);
            cells.push(td);
        }
    }

    let text = "";
    let prev_side = "";
    for (let step = 0; step < 24; step++)
    {
        let side;
        for (;;)
        {
            // Randomly pick one side of the cube.
            const sideNumber = getRandomInt(1, 6);
            switch (sideNumber)
            {
            case 1:
                side = "F";
                break;
            case 2:
                side = "B";
                break;
            case 3:
                side = "R";
                break;
            case 4:
                side = "L";
                break;
            case 5:
                side = "U";
                break;
            case 6:
                side = "D";
                break;
            }
            if (side != prev_side)
            {
                break;
            }
            // Side would be the same as the previous, try again.
        }
        prev_side = side;
        // Randomly pick a direction.
        const directionNumber = getRandomInt(1, 3);
        let direction;
        switch (directionNumber)
        {
        case 1:
            direction = " ";
            break;
        case 2:
            direction = "'";
            break;
        case 3:
            direction = "2";
            break;
        }
        text += String(side) + String(direction) + ' ';
        if (step % 4 == 3)
        {
            cells[Math.floor(step / 4)].innerText = text;
            text = '';
        }
    }
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
