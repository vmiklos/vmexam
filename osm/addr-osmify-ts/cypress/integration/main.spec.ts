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

    it('testOverpassNoresults', () => {
        cy.route2(
            'GET',
            'https://nominatim.openstreetmap.org/search.php?q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest&format=json',
            {fixture : 'nominatim-happy.json'});
        cy.route2('POST', 'http://overpass-api.de/api/interpreter',
                  {fixture : 'overpass-no-result.json'});

        cy.visit('http://0.0.0.0:8000/');

        cy.get('#nominatim-input').type('Mészáros utca 58/a, Budapest');

        cy.get('input[type="button"]').click();

        cy.get('#output').should('have.value', 'No results from overpass');
    });

    it('testOverpassError', () => {
        cy.route2(
            'GET',
            'https://nominatim.openstreetmap.org/search.php?q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest&format=json',
            {fixture : 'nominatim-happy.json'});
        cy.route2('POST', 'http://overpass-api.de/api/interpreter',
                  {statusCode : 200, body : 'not json'});

        cy.visit('http://0.0.0.0:8000/');

        cy.get('#nominatim-input').type('Mészáros utca 58/a, Budapest');

        cy.get('input[type="button"]').click();

        cy.get('#output').should((output) => {
            const outputElement = <HTMLInputElement>output.get(0);
            expect(outputElement.value).to.match(/^Overpass error:.*/);
        });
    });

    it('testNominatimNoresults', () => {
        cy.route2(
            'GET',
            'https://nominatim.openstreetmap.org/search.php?q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest&format=json',
            {statusCode : 200, body : '[]'});
        cy.route2('POST', 'http://overpass-api.de/api/interpreter',
                  {fixture : 'overpass-happy.json'});

        cy.visit('http://0.0.0.0:8000/');

        cy.get('#nominatim-input').type('Mészáros utca 58/a, Budapest');

        cy.get('input[type="button"]').click();

        cy.get('#output').should('have.value', 'No results from nominatim');
    });

    it('testNominatimError', () => {
        cy.route2(
            'GET',
            'https://nominatim.openstreetmap.org/search.php?q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest&format=json',
            {statusCode : 200, body : 'not json'});
        cy.route2('POST', 'http://overpass-api.de/api/interpreter',
                  {fixture : 'overpass-happy.json'});

        cy.visit('http://0.0.0.0:8000/');

        cy.get('#nominatim-input').type('Mészáros utca 58/a, Budapest');

        cy.get('input[type="button"]').click();

        cy.get('#output').should((output) => {
            const outputElement = <HTMLInputElement>output.get(0);
            expect(outputElement.value).to.match(/^Nominatim error:.*/);
        });
    });

    it('testPreferBuildings', () => {
        cy.route2(
            'GET',
            'https://nominatim.openstreetmap.org/search.php?q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest&format=json',
            {fixture : 'nominatim-prefer-buildings.json'});
        cy.route2('POST', 'http://overpass-api.de/api/interpreter',
                  {fixture : 'overpass-prefer-buildings.json'});

        cy.visit('http://0.0.0.0:8000/');

        cy.get('#nominatim-input').type('Mészáros utca 58/a, Budapest');

        cy.get('input[type="button"]').click();

        cy.get('#output').should(
            'have.value',
            'geo:47.47690895,19.0512550758533 (1111 Budapest, Karinthy Frigyes út 18)');
    });

    it('testSearchParam', () => {
        cy.route2(
            'GET',
            'https://nominatim.openstreetmap.org/search.php?q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest&format=json',
            {fixture : 'nominatim-happy.json'});
        cy.route2('POST', 'http://overpass-api.de/api/interpreter',
                  {fixture : 'overpass-happy.json'});

        cy.visit(
            'http://0.0.0.0:8000/?query=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest');

        // No typing into '#nominatim-input'.

        cy.get('input[type="button"]').click();

        cy.get('#output').should(
            'have.value',
            'geo:47.490592,19.030662 (1016 Budapest, Mészáros utca 58/a)');
    });
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
