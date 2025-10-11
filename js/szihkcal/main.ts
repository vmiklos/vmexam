/*
 * Copyright 2019 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

import * as calendar from 'calendar';
import * as seedRandom from 'seed-random';
// do this manually for now.
const month_name = [
    "január",
    "február",
    "március",
    "április",
    "május",
    "június",
    "július",
    "augusztus",
    "szeptember",
    "október",
    "november",
    "december",
];
const day_name = ["hétfő", "kedd", "szerda", "csütörtök", "péntek", "szombat", "vasárnap"];

const tasks = [
    "Segítségnyújtés gyermekeinknek az érzéseik elfogadásában (35.\xA0o.)",
    "Együttműködésre bírni gyermekeinket (79.\xA0o.)",
    "Büntetés helyett (118.\xA0o.)",
    "Az önállóság támogatása (151.\xA0o.)",
    "Dicséret és önértékelés (182.\xA0o.)",
    "Hogy ne kelljen gyermekeinknek szerepet játszaniuk (211.\xA0o.)",
];

/// Look up name as a key in the query string.
function getParameterByName(name: string) {
    name = name.replace(/[[]/, "\\[").replace(/[\]]/, "\\]");
    const regex = new RegExp("[\\?&]" + name + "=([^&#]*)"),
        results = regex.exec(location.search);
    return results === null ? "" : results[1].replace(/\+/g, " ");
}

/// Formats the calendar.
function formatcal() {
    const table = document.createElement("table");
    table.className = "month";
    document.getElementsByTagName("body")[0].appendChild(table);

    // The year / month row.
    let tr = table.insertRow(table.rows.length);
    const th = document.createElement("th");
    th.className = "month";
    tr.appendChild(th);
    th.colSpan = 7;
    const date = new Date();
    const year = date.getFullYear();
    const month = date.getMonth();
    th.appendChild(document.createTextNode(year + " " + month_name[month]));

    // The days of the week row.
    tr = table.insertRow(table.rows.length);
    day_name.forEach(function (day: string) {
        const th = document.createElement("th");
        tr.appendChild(th);
        th.appendChild(document.createTextNode(day));
    });

    // The actual days.
    const cal = new calendar.Calendar(1); // weeks starting on Monday
    const matrix = cal.monthDays(year, month);
    matrix.forEach(function (week: number[]) {
        const tr = table.insertRow(table.rows.length);
        week.forEach(function (day: number) {
            const td = document.createElement("td");
            td.className = "day";
            tr.appendChild(td);
            if (day == 0) {
                td.appendChild(document.createTextNode(" "));
            } else {
                td.appendChild(document.createTextNode(day.toString()));
                td.appendChild(document.createElement("br"));
                const task = tasks[Math.floor(Math.random() * tasks.length)];
                td.appendChild(document.createTextNode(task));
            }
        });
    });
}

document.addEventListener("DOMContentLoaded", function () {
    const seed = getParameterByName("seed");
    if (seed) {
        seedRandom(seed, { global: true });
    }
    formatcal();
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
