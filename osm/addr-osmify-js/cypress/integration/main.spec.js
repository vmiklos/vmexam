/*
 * Copyright 2020 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

describe('TestMain', () => {
    it('testHappy', () => {
        cy.visit('http://0.0.0.0:8000/');

        cy.get('#nominatim-input').type('Mészáros utca 58/a, Budapest');

        cy.get('input[type="button"]').click();

        cy.get('#output').should(
            'have.value',
            'geo:47.49054945,19.030744891956317 (1016 Budapest, Mészáros utca 58/A)');
    });
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
