/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

var calendar = require('node-calendar');
var domready = require('domready');
var seedRandom = require('seed-random');
// cldr doesn't seem to work with browserify, so do this manually for now.
calendar.month_name = [
    '', 'január', 'február', 'március', 'április', 'május', 'június', 'július',
    'augusztus', 'szeptember', 'október', 'november', 'december'
];
calendar.day_name =
    [ 'hétfő', 'kedd', 'szerda', 'csütörtök', 'péntek', 'szombat', 'vasárnap' ];

var tasks = [
    'Segítségnyújtés gyermekeinknek az érzéseik elfogadásában (35.\xA0o.)',
    'Együttműködésre bírni gyermekeinket (79.\xA0o.)',
    'Büntetés helyett (118.\xA0o.)', 'Az önállóság támogatása (151.\xA0o.)',
    'Dicséret és önértékelés (182.\xA0o.)',
    'Hogy ne kelljen gyermekeinknek szerepet játszaniuk (211.\xA0o.)'
];

/// Look up name as a key in the query string.
function getParameterByName(name) {
    name = name.replace(/[\[]/, '\\[').replace(/[\]]/, '\\]');
    var regex = new RegExp('[\\?&]' + name + '=([^&#]*)'),
        results = regex.exec(location.search);
    return results === null ? '' : results[1].replace(/\+/g, ' ');
}

/// Formats the calendar.
function formatcal() {
    var table = document.createElement('table');
    table.className = 'month';
    document.getElementsByTagName('body')[0].appendChild(table);

    // The year / month row.
    var tr = table.insertRow(table.rows.length);
    var th = document.createElement('th');
    th.className = 'month';
    tr.appendChild(th);
    th.colSpan = 7;
    var date = new Date();
    var year = date.getFullYear();
    var month = date.getMonth() + 1;
    th.appendChild(
        document.createTextNode(year + ' ' + calendar.month_name[month]));

    // The days of the week row.
    tr = table.insertRow(table.rows.length);
    calendar.day_name.forEach(function(day) {
        var th = document.createElement('th');
        tr.appendChild(th);
        th.appendChild(document.createTextNode(day));
    });

    // The actual days.
    var cal = new calendar.Calendar();
    var matrix = cal.monthdayscalendar(year, month);
    matrix.forEach(function(week) {
        var tr = table.insertRow(table.rows.length);
        week.forEach(function(day) {
            var td = document.createElement('td');
            td.className = 'day';
            tr.appendChild(td);
            if (day == 0) {
                td.appendChild(document.createTextNode(' '));
            } else {
                td.appendChild(document.createTextNode(day));
                td.appendChild(document.createElement('br'));
                var task = tasks[Math.floor(Math.random() * tasks.length)];
                td.appendChild(document.createTextNode(task));
            }
        });
    });
}

domready(function() {
    var seed = getParameterByName('seed');
    if (seed) {
        seedRandom(seed, {global : true});
    }
    formatcal();
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
