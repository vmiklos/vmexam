/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

import confetti from 'canvas-confetti';
import * as THREE from 'three';

import * as rubik from './rubik';

declare global
{
    interface Window
    {
        app: App;
    }
}
// RubikResult represents the result from the solver.
interface RubikResult
{
    solution: string;
    error: string;
}

class App
{
    rubik: rubik.Rubik;

    // Painting the faces alterns this. Format:
    // UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB
    // Once this has no X in it, we can send it to the solver.
    faces: string[];

    // The solution we got from the solver. If empty, then we paint faces.
    // Otherwise we show a solution.
    solution: string[];
    // If solution is not empty we show this state.
    // If solution has N moves, this goes from 0..N (inclusive).
    solutionIndex: number;

    // We want to paint the face to colorName / colorValue.
    colorName: string;
    colorValue: string;

    colorPickerCells: HTMLTableCellElement[];
    prevFaceButton: HTMLInputElement;
    nextFaceButton: HTMLInputElement;
    solveButton: HTMLInputElement;

    counterSpan: HTMLSpanElement;

    // The picker is used on this face: 0..5 (FRUBDL).
    pickingFace: number;

    static colorValues: {[index: string]: string} = {
        'U' : '#01499b', // Blue
        'F': '#7c0000',  // Red
        'R': '#e1c501',  // Yellow
        'B': '#f55225',  // Orange
        'L': '#b9d0d8',  // White
        'D': '#028d76',  // Green
        'X': '#7f7f7f',  // Gray
    };

    constructor()
    {
        this.solution = [];
        this.solutionIndex = 0;
        this.colorName = 'U';
        this.colorValue = App.colorValues['U'];
        this.colorPickerCells = [];
        this.pickingFace = 0;
    }

    createPickerCell(row: HTMLTableRowElement, cName: string, cValue: string)
    {
        const cell = document.createElement('td');
        cell.style.width = '64px';
        cell.style.height = '64px';
        cell.style.background = cValue;
        cell.style.borderWidth = 'medium';
        cell.style.borderStyle = 'solid';
        if (cName == 'U')
        {
            cell.style.borderColor = '#000000';
        }
        else
        {
            cell.style.borderColor = '#ffffff';
        }
        cell.onclick = function() {
            app.colorName = cName;
            app.colorValue = cValue;
            app.colorPickerCells.forEach(function(c) {
                if (c == cell)
                {
                    c.style.borderColor = '#000000';
                }
                else
                {
                    c.style.borderColor = '#ffffff';
                }
            });
        };
        const cellDiv = document.createElement('div');
        cellDiv.id = 'cell-div-' + cName;
        cellDiv.style.textAlign = 'center';

        const cellBackground = cell.style.background;
        // Parse rgb(r, g, b).
        const [r, g, b] = cellBackground.substring(4, cellBackground.length - 1)
                              .split(' ')
                              .map(x => parseInt(x));
        const isDark = ((b * 29 + g * 151 + r * 76) >> 8) <= 156;
        if (isDark)
        {
            cellDiv.style.color = '#ffffff';
        }
        else
        {
            cellDiv.style.color = '#000000';
        }

        cell.appendChild(cellDiv);
        row.appendChild(cell);
        app.colorPickerCells.push(cell);
    }

    // Handles clicks on the cube (scene).
    static cubeOnClick(event: MouseEvent)
    {
        if (app.solution.length)
        {
            // Showing solution, not painting.
            return;
        }

        const x = (event.clientX / app.rubik.SCREEN_WIDTH) * 2 - 1;
        const y = -(event.clientY / app.rubik.SCREEN_HEIGHT) * 2 + 1;
        const raycaster = new THREE.Raycaster();
        raycaster.setFromCamera(new THREE.Vector2(x, y), app.rubik.camera);
        const intersects = raycaster.intersectObjects(app.rubik.allCubes);
        const cubelets = intersects.filter(
            (intersect) => intersect.object.name.startsWith('cubelet'));
        if (!cubelets.length)
        {
            return;
        }

        const cubelet = cubelets[0].object as THREE.Mesh;

        // Update the facelet model.
        const cubeletIndex = Number(cubelet.name.substr('cubelet'.length));
        // F -> R -> U -> B -> D -> L
        // Hit testing gives us a cubelet index. Depending on what face we see,
        // different cubelet indexes (0..26) refer to different face indexes
        // (0..53). We need face indexes to construct a facelet string, which
        // will be the input for the solver. Disallow painting the central
        // cublet, which should already have the right color if you hold the
        // cube with the correct orientation.
        const cubeToFaceMap: {[index: number]: {[index: number]: number}} = {
            0 : {
                // F
                8 : 18,
                17 : 19,
                26 : 20,
                5 : 21,
                // 14 : 22,
                23 : 23,
                2 : 24,
                11 : 25,
                20 : 26,
            },
            1 : {
                // R
                26 : 9,
                25 : 10,
                24 : 11,
                23 : 12,
                // 22 : 13,
                21 : 14,
                20 : 15,
                19 : 16,
                18 : 17,
            },
            2 : {
                // U
                6 : 0,
                15 : 1,
                24 : 2,
                7 : 3,
                // 16 : 4,
                25 : 5,
                8 : 6,
                17 : 7,
                26 : 8,
            },
            3 : {
                // B
                24 : 45,
                15 : 46,
                6 : 47,
                21 : 48,
                // 12 : 49,
                3 : 50,
                18 : 51,
                9 : 52,
                0 : 53,
            },
            4 : {
                // D
                2 : 27,
                11 : 28,
                20 : 29,
                1 : 30,
                // 10 : 31,
                19 : 32,
                0 : 33,
                9 : 34,
                18 : 35,
            },
            5 : {
                // L
                6 : 36,
                7 : 37,
                8 : 38,
                3 : 39,
                // 4 : 40,
                5 : 41,
                0 : 42,
                1 : 43,
                2 : 44,
            },
        };
        const faceIndex = cubeToFaceMap[app.pickingFace][cubeletIndex];
        if (faceIndex === undefined)
        {
            return;
        }

        app.faces[faceIndex] = app.colorName;
        app.updateSolveButton();
        // Update the view.
        const cubeletMaterials = cubelet.material as THREE.Material[];
        // Box geometry order is    R, L, U, D, F, B.
        const pickingToMaterialMap: {[index: number]: number} = {
            /*F=*/ 0 : 4,
            /*R=*/ 1 : 0,
            /*U=*/ 2 : 2,
            /*B=*/ 3 : 5,
            /*D=*/ 4 : 3,
            /*L=*/ 5 : 1,
        };
        const materialIndex = pickingToMaterialMap[app.pickingFace];
        cubeletMaterials[materialIndex] = new THREE.MeshLambertMaterial({
            emissive : app.colorValue,
            emissiveMap : app.rubik.cubeletTexture
        });
    }

    static async solveOnClick()
    {
        const url = 'https://share.vmiklos.hu/apps/rubik/?facelet=' +
                    app.faces.join('');
        const request = new Request(url, {method : 'GET'});
        try
        {
            const response = await window.fetch(request);
            const result = await<Promise<RubikResult>>response.json();
            if (result.error.length)
            {
                const p = document.getElementById('p-error');
                p.innerText = 'error from solver: ' + result.error;
                return;
            }

            app.solution = result.solution.split(' ');
            app.solveButton.disabled = true;
            // Back to the starting point.
            app.rubik.cubeTurn('U');
            app.rubik.camera.position.y = 10;
            app.rubik.camera.rotation.x = -Math.PI / 6;
            app.prevFaceButton.disabled = true;
            app.nextFaceButton.disabled = false;
            app.updateCounterSpan();
        }
        catch (reason)
        {
            const p = document.getElementById('p-error');
            p.innerText = 'failed to fetch from solver: ' + reason;
        }
    }

    static prevFaceOnClick()
    {
        // L -> D -> B -> U -> R -> F
        const faceIndexToNotationMap: {[index: number]: string} = {
            5 : `L'`,
            4 : `U'`,
            3 : `U'`,
            2 : `L'`,
            1 : `U'`,
        };
        const notation = faceIndexToNotationMap[app.pickingFace];
        app.pickingFace--;
        if (app.pickingFace == 4)
        {
            app.nextFaceButton.disabled = false;
        }
        if (app.pickingFace == 0)
        {
            app.prevFaceButton.disabled = true;
        }
        app.updateCounterSpan();

        app.rubik.cubeTurn(notation);
    }

    updateCounterSpan()
    {
        if (this.solution.length)
        {
            this.counterSpan.innerText = String(this.solutionIndex + 1) +
                                         ' / ' + (this.solution.length + 1) +
                                         ' ';
            return;
        }

        this.counterSpan.innerText = String(this.pickingFace + 1) + ' / 6 ';
    }

    updateSolveButton()
    {
        let validCounts = true;
        // UFRBLD is the order of the color picker cells.
        ['U', 'F', 'R', 'B', 'L', 'D'].forEach(face => {
            const count = this.faces.filter(i => i === face).length;
            if (count != 9)
            {
                validCounts = false;
            }
            const div = document.getElementById('cell-div-' + face);
            div.innerText = String(count);
        });

        if (this.pickingFace == 5 && validCounts)
        {
            this.solveButton.disabled = false;
        }
        else
        {
            this.solveButton.disabled = true;
        }
    }

    static nextFaceOnClick()
    {
        if (app.solution.length)
        {
            const notation = app.solution[app.solutionIndex];
            app.solutionIndex++;
            if (app.solutionIndex == app.solution.length)
            {
                app.nextFaceButton.disabled = true;
                confetti({
                    particleCount : 150,
                    ticks : 600,
                });
            }
            app.updateCounterSpan();
            app.rubik.faceTurn(notation);
            return;
        }

        // F -> R -> U -> B -> D -> L
        const faceIndexToNotationMap: {[index: number]: string} = {
            0 : 'U',
            1 : 'L',
            2 : 'U',
            3 : 'U',
            4 : 'L',
        };
        const notation = faceIndexToNotationMap[app.pickingFace];
        app.pickingFace++;
        if (app.pickingFace == 1)
        {
            app.prevFaceButton.disabled = false;
        }
        if (app.pickingFace == 5)
        {
            app.nextFaceButton.disabled = true;
        }
        app.updateCounterSpan();
        app.updateSolveButton();

        app.rubik.cubeTurn(notation);
    }

    static render()
    {
        if (app.rubik.isMoving)
        {
            app.rubik.doMove();
        }

        app.rubik.renderer.render(app.rubik.scene, app.rubik.camera);
        requestAnimationFrame(App.render);
    }

    createPage()
    {
        document.body.style.backgroundColor = '#ffffff';
        // Create our page: the cube.
        app.rubik = new rubik.Rubik();
        let faces = 'XXXXUXXXXXXXXRXXXXXXXXFXXXXXXXXDXXXXXXXXLXXXXXXXXBXXXX';
        const urlParams = new URLSearchParams(window.location.search);
        const facesParam = urlParams.get('faces');
        if (facesParam != null)
        {
            faces = facesParam;
        }
        app.faces = [...faces ];
        app.rubik.init(faces, App.colorValues);
        app.rubik.scene.background = new THREE.Color('#ffffff');
        app.rubik.renderer.domElement.addEventListener('click',
                                                       App.cubeOnClick);

        // Color picker table: White, green, red; then blue, orange, yellow; 10%
        // height in total.
        const colorsTable = document.createElement('table');
        colorsTable.style.marginLeft = 'auto';
        colorsTable.style.marginRight = 'auto';
        document.body.appendChild(colorsTable);
        const colorsRow1 = document.createElement('tr');
        colorsTable.appendChild(colorsRow1);
        app.createPickerCell(colorsRow1, 'U', App.colorValues['U']);
        app.createPickerCell(colorsRow1, 'F', App.colorValues['F']);
        app.createPickerCell(colorsRow1, 'R', App.colorValues['R']);
        const colorsRow2 = document.createElement('tr');
        colorsTable.appendChild(colorsRow2);
        app.createPickerCell(colorsRow2, 'B', App.colorValues['B']);
        app.createPickerCell(colorsRow2, 'L', App.colorValues['L']);
        app.createPickerCell(colorsRow2, 'D', App.colorValues['D']);

        const buttons = document.createElement('p');
        buttons.style.textAlign = 'center';
        document.body.appendChild(buttons);
        app.counterSpan = document.createElement('span');
        app.updateCounterSpan();
        buttons.appendChild(app.counterSpan);
        app.prevFaceButton = document.createElement('input');
        app.prevFaceButton.type = 'button';
        app.prevFaceButton.value = '< prev';
        app.prevFaceButton.onclick = App.prevFaceOnClick;
        app.prevFaceButton.disabled = true;
        buttons.appendChild(app.prevFaceButton);
        buttons.appendChild(document.createTextNode(' '));
        app.nextFaceButton = document.createElement('input');
        app.nextFaceButton.type = 'button';
        app.nextFaceButton.value = 'next >';
        app.nextFaceButton.onclick = App.nextFaceOnClick;
        buttons.appendChild(app.nextFaceButton);
        buttons.appendChild(document.createTextNode(' '));
        app.solveButton = document.createElement('input');
        app.solveButton.type = 'button';
        app.solveButton.value = '✓ solve';
        app.solveButton.onclick = App.solveOnClick;
        app.updateSolveButton();
        buttons.appendChild(app.solveButton);

        const error = document.createElement('p');
        error.id = 'p-error';
        error.style.textAlign = 'center';
        error.innerText =
            'start with the facing side: red on the facing side, yellow on the right side';
        document.body.appendChild(error);

        const credit = document.createElement('p');
        credit.style.textAlign = 'center';
        credit.appendChild(document.createTextNode('built using '));
        const kewb = document.createElement('a');
        kewb.href = 'https://crates.io/crates/kewb';
        kewb.innerText = 'kewb';
        credit.appendChild(kewb);
        credit.appendChild(document.createTextNode(' and '));
        const three = document.createElement('a');
        three.href = 'http://joews.github.io/rubik-js/';
        three.innerText = 'three.js';
        credit.appendChild(three);
        document.body.appendChild(credit);

        App.render();
    }
}

const app = new App();
// Devtools console access.
window.app = app;

document.addEventListener("DOMContentLoaded", app.createPage);

// vim: shiftwidth=4 softtabstop=4 expandtab:
