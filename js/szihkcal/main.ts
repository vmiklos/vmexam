/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

import calendar = require('node-calendar');
import domready = require('domready');
import seedRandom = require('seed-random');
// cldr doesn't seem to work with browserify, so do this manually for now.
calendar.month_name = [
    '', 'január', 'február', 'március', 'április', 'május', 'június', 'július',
    'augusztus', 'szeptember', 'október', 'november', 'december'
];
calendar.day_name =
    [ 'hétfő', 'kedd', 'szerda', 'csütörtök', 'péntek', 'szombat', 'vasárnap' ];

const tasks = [
    'Segítségnyújtés gyermekeinknek az érzéseik elfogadásában (35.\xA0o.)',
    'Együttműködésre bírni gyermekeinket (79.\xA0o.)',
    'Büntetés helyett (118.\xA0o.)', 'Az önállóság támogatása (151.\xA0o.)',
    'Dicséret és önértékelés (182.\xA0o.)',
    'Hogy ne kelljen gyermekeinknek szerepet játszaniuk (211.\xA0o.)'
];

/// Look up name as a key in the query string.
function getParameterByName(name: string)
{
    name = name.replace(/[[]/, '\\[').replace(/[\]]/, '\\]');
    const regex = new RegExp('[\\?&]' + name + '=([^&#]*)'),
          results = regex.exec(location.search);
    return results === null ? '' : results[1].replace(/\+/g, ' ');
}

/// Formats the calendar.
function formatcal()
{
    const table = document.createElement('table');
    table.className = 'month';
    document.getElementsByTagName('body')[0].appendChild(table);

    // The year / month row.
    let tr = table.insertRow(table.rows.length);
    const th = document.createElement('th');
    th.className = 'month';
    tr.appendChild(th);
    th.colSpan = 7;
    const date = new Date();
    const year = date.getFullYear();
    const month = date.getMonth() + 1;
    th.appendChild(
        document.createTextNode(year + ' ' + calendar.month_name[month]));

    // The days of the week row.
    tr = table.insertRow(table.rows.length);
    calendar.day_name.forEach(function(day: string) {
        const th = document.createElement('th');
        tr.appendChild(th);
        th.appendChild(document.createTextNode(day));
    });

    // The actual days.
    const cal = new calendar.Calendar();
    const matrix = cal.monthdayscalendar(year, month);
    matrix.forEach(function(week: number[]) {
        const tr = table.insertRow(table.rows.length);
        week.forEach(function(day: number) {
            const td = document.createElement('td');
            td.className = 'day';
            tr.appendChild(td);
            if (day == 0)
            {
                td.appendChild(document.createTextNode(' '));
            }
            else
            {
                td.appendChild(document.createTextNode(day.toString()));
                td.appendChild(document.createElement('br'));
                const task = tasks[Math.floor(Math.random() * tasks.length)];
                td.appendChild(document.createTextNode(task));
            }
        });
    });
}

domready(function() {
    const seed = getParameterByName('seed');
    if (seed)
    {
        seedRandom(seed, {global : true});
    }
    formatcal();
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
