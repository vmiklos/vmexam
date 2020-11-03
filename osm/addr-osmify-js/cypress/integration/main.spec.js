/*
 * Copyright 2020 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

describe('TestMain', () => {
    it('testHappy', () => {
        cy.route2(
            'GET',
            'https://nominatim.openstreetmap.org/search.php?q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest&format=json',
            {fixture : 'nominatim-happy.json'});
        cy.route2('POST', 'http://overpass-api.de/api/interpreter',
                  {fixture : 'overpass-happy.json'});

        cy.visit('http://0.0.0.0:8000/');

        cy.get('#nominatim-input').type('Mészáros utca 58/a, Budapest');

        cy.get('input[type="button"]').click();

        cy.get('#output').should(
            'have.value',
            'geo:47.490592,19.030662 (1016 Budapest, Mészáros utca 58/a)');
    });
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
