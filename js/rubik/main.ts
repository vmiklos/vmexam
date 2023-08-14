/*
 * From <https://github.com/Aaron-Bird/rubiks-cube>.
 *
 * For my additions:
 *
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

import TWEEN from '@tweenjs/tween.js';
import * as THREE from 'three';

declare global
{
    interface Window
    {
        app: App;
    }
}

const c: {[index: string]: string} = {
    'U' : '#01499b', // Blue
    'F' : '#7c0000', // Red
    'R' : '#e1c501', // Yellow
    'B' : '#f55225', // Orange
    'L' : '#b9d0d8', // White
    'D' : '#028d76', // Green
    'X' : '#141517', // Gray
};

// Used by RubikCube::move().
const notationSwapTable:
    {[indexOf: string]: [ number, number, number, number ][];} = {
        L : [
            [ 0, 18, 27, 53 ], [ 3, 21, 30, 50 ], [ 6, 24, 33, 47 ],
            [ 36, 38, 44, 42 ], [ 37, 41, 43, 39 ]
        ],
        R : [
            [ 20, 2, 51, 29 ], [ 23, 5, 48, 32 ], [ 26, 8, 45, 35 ],
            [ 9, 11, 17, 15 ], [ 10, 14, 16, 12 ]
        ],
        U : [
            [ 9, 18, 36, 45 ], [ 10, 19, 37, 46 ], [ 11, 20, 38, 47 ],
            [ 0, 2, 8, 6 ], [ 1, 5, 7, 3 ]
        ],
        D : [
            [ 15, 51, 42, 24 ], [ 16, 52, 43, 25 ], [ 17, 53, 44, 26 ],
            [ 27, 29, 35, 33 ], [ 28, 32, 34, 30 ]
        ],
        F : [
            [ 6, 9, 29, 44 ], [ 7, 12, 28, 41 ], [ 8, 15, 27, 38 ],
            [ 18, 20, 26, 24 ], [ 19, 23, 25, 21 ]
        ],
        B : [
            [ 2, 36, 33, 17 ], [ 1, 39, 34, 14 ], [ 0, 42, 35, 11 ],
            [ 45, 47, 53, 51 ], [ 46, 50, 52, 48 ]
        ],
    };

// One cubelet is one piece of the cube, there are 27 cubelets in total.
interface Cubelet
{
    x: number, y: number, z: number, num: number, type: string,
        color?: {[index: string]: string},
}

// Base class for RubikCubeModel.
class RubikCube
{
    cubelets: Cubelet[] = [];
    colors: string[];
    constructor(colorStr?: string)
    {
        this.colors = colorStr.trim().split('');

        this.generateCoords();
        this.generateColors();
    }

    // Used by the RubikCube ctor.
    generateCoords()
    {
        let num = 0;
        for (let y = 1; y >= -1; y--)
        {
            for (let z = -1; z <= 1; z++)
            {
                for (let x = -1; x <= 1; x++)
                {
                    const n = [ x, y, z ].filter(Boolean).length;
                    let type;
                    if (n === 3)
                        type = 'corner'; // Corner block
                    if (n === 2)
                        type = 'edge'; // Edge block
                    if (n === 1)
                        type = 'center'; // Center block

                    this.cubelets.push({x, y, z, num, type});
                    num++;
                }
            }
        }
    }

    // Used by the RubikCube ctor.
    generateColors()
    {
        const colorNames = 'URFDLB'.split('');
        interface FaceColor
        {
            [index: string]: string[];
        }
        const faceColor: FaceColor = {};
        for (let i = 0; i < colorNames.length; i++)
        {
            const name = colorNames[i];
            const start = i * 9;
            const end = start + 9;
            faceColor[name] = this.colors.slice(start, end);
        }

        for (const cubelet of this.cubelets)
        {
            const cubeColor: {[index: string]: string} = {};
            const {x, y, z, num} = cubelet;

            // Up
            if (y === 1)
            {
                const i = num;
                cubeColor['U'] = c[faceColor['U'][i]];
            }

            // Down
            if (y === -1)
            {
                const n = num - 18;
                const i = Math.floor((8 - n) / 3) * 3 + (3 - (8 - n) % 3) - 1;
                cubeColor['D'] = c[faceColor['D'][i]];
            }

            // Right
            if (x === 1)
            {
                const n = (num + 1) / 3 - 1;
                const i = Math.floor(n / 3) * 3 + (3 - n % 3) - 1;
                cubeColor['R'] = c[faceColor['R'][i]];
            }

            // Left
            if (x === -1)
            {
                const i = num / 3;
                cubeColor['L'] = c[faceColor['L'][i]];
            }

            // Front
            if (z === 1)
            {
                const i = Math.floor((num - 6) / 7) + ((num - 6) % 7);
                cubeColor['F'] = c[faceColor['F'][i]];
            }

            // Back
            if (z === -1)
            {
                const n = Math.floor(num / 7) + (num % 7);
                const i = Math.floor(n / 3) * 3 + (3 - n % 3) - 1;
                cubeColor['B'] = c[faceColor['B'][i]];
            }
            cubelet.color = cubeColor;
        }
    }

    // Used by rotate().
    move(notationStr: string)
    {
        const notations = notationStr.trim().split(' ');
        for (const i of notations)
        {
            let toward = 1;
            let rotationTimes = 1;
            const notation = i[0];
            const secondNota = i[1];
            if (secondNota)
            {
                if (secondNota === `'`)
                {
                    toward = -1;
                }
                else if (secondNota === `2`)
                {
                    rotationTimes = 2;
                }
                else
                {
                    throw new Error(`Wrong secondNota: ${secondNota}`);
                }
            }

            for (let j = 0; j < rotationTimes; j++)
            {
                const actions = notationSwapTable[notation];
                for (const k of actions)
                {
                    this.swapFaceColor(k, toward);
                }
            }
        }
    }

    // Used by move().
    swapFaceColor(faceColorNums: number[], toward: number)
    {
        const [a, b, c, d] = faceColorNums;
        const colors = this.colors;
        const aColor = colors[a];
        if (toward === -1)
        {
            colors[a] = colors[b];
            colors[b] = colors[c];
            colors[c] = colors[d];
            colors[d] = aColor;
        }
        else if (toward === 1)
        {
            colors[a] = colors[d];
            colors[d] = colors[c];
            colors[c] = colors[b];
            colors[b] = aColor;
        }
        else
        {
            throw new Error(`Wrong toward: ${toward}`);
        }
    }
}

// Used by RubikCubeModel::generateCubeletModel().
function roundedEdgeBox(width = 1, height = 1, depth = 1, radius0 = 0.1,
                        smoothness = 4)
{
    const shape = new THREE.Shape();
    const eps = 0.00001;
    const radius = radius0 - eps;
    shape.absarc(eps, eps, eps, -Math.PI / 2, -Math.PI, true);
    shape.absarc(eps, height - radius * 2, eps, Math.PI, Math.PI / 2, true);
    shape.absarc(width - radius * 2, height - radius * 2, eps, Math.PI / 2, 0,
                 true);
    shape.absarc(width - radius * 2, eps, eps, 0, -Math.PI / 2, true);
    const geometry = new THREE.ExtrudeGeometry(shape, {
        depth : depth - radius0 * 2,
        bevelEnabled : true,
        bevelSegments : smoothness * 2,
        steps : 1,
        bevelSize : radius,
        bevelThickness : radius0,
        curveSegments : smoothness,
    });
    geometry.center();
    return geometry;
}

// Used by RubikCubeModel::generateCubeletModel().
function roundedPlane(x = 0, y = 0, width = 0.9, height = 0.9, radius = 0.1)
{
    const shape = new THREE.Shape();
    const center = new THREE.Vector2(-(x + width / 2), -(y + height / 2));
    shape.moveTo(center.x, center.y + radius);
    shape.lineTo(center.x, center.y + height - radius);
    shape.quadraticCurveTo(center.x, center.y + height, center.x + radius,
                           center.y + height);
    shape.lineTo(center.x + width - radius, center.y + height);
    shape.quadraticCurveTo(center.x + width, center.y + height,
                           center.x + width, center.y + height - radius);
    shape.lineTo(center.x + width, center.y + radius);
    shape.quadraticCurveTo(center.x + width, center.y,
                           center.x + width - radius, center.y);
    shape.lineTo(center.x + radius, center.y);
    shape.quadraticCurveTo(center.x, center.y, center.x, center.y + radius);
    const geometry = new THREE.ShapeGeometry(shape);
    return geometry;
}

// Used by RubikCubeModel::generateCubeletModel().
interface CubeletModel extends THREE.Mesh
{
    cubeType?: string;
    num?: number;
    initPosition?: THREE.Vector3;
}

// Used by RuikCubeModel::generateCubeletModel().
const faceInfo: {
    [index: string]: {
        position: [ number, number, number ],
        rotation: [ number, number, number ]
    }
} = {
    U : {position : [ 0, 0.51, 0 ], rotation : [ -Math.PI / 2, 0, 0 ]},
    D : {position : [ 0, -0.51, 0 ], rotation : [ Math.PI / 2, 0, 0 ]},
    F : {position : [ 0, 0, 0.51 ], rotation : [ 0, 0, 0 ]},
    B : {position : [ 0, 0, -0.51 ], rotation : [ Math.PI, 0, 0 ]},
    L : {position : [ -0.51, 0, 0 ], rotation : [ 0, -Math.PI / 2, 0 ]},
    R : {position : [ 0.51, 0, 0 ], rotation : [ 0, Math.PI / 2, 0 ]},
};

// The actual 3x3 Rubik cube.
class RubikCubeModel extends RubikCube
{
    model = new THREE.Group();
    constructor(fb?: string)
    {
        super(fb);
        let i = 0;
        for (const cubeInfo of this.cubelets)
        {
            const cubeletModel = this.generateCubeletModel(cubeInfo, i);
            cubeletModel.name = 'cubelet';
            cubeletModel.cubeType = cubeInfo.type;
            cubeletModel.num = cubeInfo.num;
            cubeletModel.position.set(cubeInfo.x, cubeInfo.y, cubeInfo.z);
            cubeletModel.initPosition =
                new THREE.Vector3().set(cubeInfo.x, cubeInfo.y, cubeInfo.z);
            this.model.add(cubeletModel);
            i++;
        }
    }

    // Used by the RubikCubeModel ctor.
    generateCubeletModel(info: Cubelet, index: number)
    {
        const geometry = roundedEdgeBox(1, 1, 1, 0.05, 4);
        const materials = new THREE.MeshLambertMaterial(
            {emissive : '#333', transparent : true});
        const cubeletModel =
            new THREE.Mesh(geometry, materials) as CubeletModel;
        const color = info.color;
        for (const key of Object.keys(color))
        {
            const planeGeometry = roundedPlane(0, 0, 0.9, 0.9, 0.1);
            const planeMaterial = new THREE.MeshLambertMaterial(
                {emissive : color[key], transparent : true});
            const plane = new THREE.Mesh(planeGeometry, planeMaterial);
            plane.rotation.fromArray(faceInfo[key].rotation);
            plane.position.fromArray(faceInfo[key].position);
            plane.name = 'faceOfCubelet' + index;
            cubeletModel.attach(plane);
        }
        return cubeletModel;
    }
}

type Axis = 'x'|'y'|'z';

type AxisValue = number;

type Toward = 1|- 1;

/// Can rotate one face of the cube or the entire cube.
class LayerModel extends THREE.Group
{
    // Used by rotate().
    group(axis: Axis, value: AxisValue, cubelets: THREE.Object3D[])
    {
        // Each Object3d can only have one parent.
        // Object3d will be removed from cubeletModels when it is added to
        // layerGroup. for (let i = 0; i < cubeletModels.length; i++) {
        for (let i = cubelets.length - 1; i >= 0; i--)
        {
            if (cubelets[i].position[axis] === value)
            {
                this.add(cubelets[i]);
            }
        }
    }

    // Used by rotate().
    groupAll(axis: Axis, cubelets: THREE.Object3D[])
    {
        // Each Object3d can only have one parent.
        // Object3d will be removed from cubeletModels when it is added to
        // layerGroup. for (let i = 0; i < cubeletModels.length; i++) {
        for (let i = cubelets.length - 1; i >= 0; i--)
        {
            this.add(cubelets[i]);
        }
    }

    // Used by rotationTransition().
    ungroup(target: THREE.Object3D)
    {
        if (!this.children.length)
        {
            return;
        }
        // Updates the global transform If you need to get rotation immediately
        // when rotation Object3d
        this.updateWorldMatrix(false, false);

        for (let i = this.children.length - 1; i >= 0; i--)
        {
            const obj = this.children[i];

            const position = new THREE.Vector3();
            obj.getWorldPosition(position);

            const quaternion = new THREE.Quaternion();
            obj.getWorldQuaternion(quaternion);

            this.remove(obj);

            position.x = parseFloat((position.x).toFixed(15));
            position.y = parseFloat((position.y).toFixed(15));
            position.z = parseFloat((position.z).toFixed(15));

            obj.position.copy(position);
            obj.quaternion.copy(quaternion);

            target.add(obj);
        }
    }

    // Used by rotationTransition().
    resetRotation()
    {
        this.rotation.x = 0;
        this.rotation.y = 0;
        this.rotation.z = 0;
    }

    // Used by rotationTransition().
    async rotationAnimation(axis: Axis, endRad: number)
    {
        if (!['x', 'y', 'z'].includes(axis))
        {
            throw new Error(`Wrong axis: ${axis}`);
        }

        // The rotation degree may be greater than 360
        // Like: 361 -> 0
        const startRad = this.rotation[axis] % (Math.PI * 2);
        if (startRad === endRad)
        {
            return;
        }

        const current = {rad : startRad};
        const end = {rad : endRad};
        const time = Math.abs(endRad - startRad) * (500 / Math.PI);

        return new Promise((resolve, reject) => {
            try
            {
                new TWEEN.Tween(current)
                    .to(end, time)
                    .easing(TWEEN.Easing.Quadratic.Out)
                    .onUpdate(() => {
                        this.rotation[axis] = current.rad;
                        // Updates the global transform If you need to get
                        // rotation immediately this.updateWorldMatrix(false,
                        // false);
                    })
                    .onComplete(resolve)
                    // Parameter 'undefined' is needed in version 18.6.0
                    // Reference: https://github.com/tweenjs/tween.js/pull/550
                    .start(undefined);
            }
            catch (err)
            {
                reject(err);
            }
        });
    }
}

type NotationBase = 'L'|'R'|'D'|'U'|'B'|'F';
type NotationExtra = ''|`'`|'2';

// Used by toRotation().
const axisTable: {[key in NotationBase]: [ Axis, AxisValue, Toward ]} = {
    L : [ 'x', -1, 1 ],
    R : [ 'x', 1, -1 ],
    D : [ 'y', -1, 1 ],
    U : [ 'y', 1, -1 ],
    B : [ 'z', -1, 1 ],
    F : [ 'z', 1, -1 ],
};

// Used by rotate().
function toRotation(notation: string): [ Axis, number, number ]
{
    notation = notation.trim();

    const base = notation[0] as NotationBase;
    const extra = notation[1] as NotationExtra;

    if (!axisTable[base])
    {
        throw new Error(`Wrong notation: ${notation}`);
    }

    const [axis, axisValue, toward] = axisTable[base];
    let rad = (Math.PI / 2) * toward;

    if (extra)
    {
        if (extra === `'`)
        {
            rad *= -1;
        }
        else if (extra === '2')
        {
            rad *= 2;
        }
        else
        {
            throw new Error(`Wrong notation: ${notation}`);
        }
    }
    return [ axis, axisValue, rad ];
}

// Used by rotate().
async function rotationTransition(axis: Axis, endRad: number)
{
    await app.layerGroup.rotationAnimation(axis, endRad);
    app.layerGroup.ungroup(app.rubikCube.model);
    app.layerGroup.resetRotation();
}

// Used by e.g. nextFaceOnClick().
async function rotate(notation: string, cube = true)
{
    const [layerRorationAxis, axisValue, rotationRad] = toRotation(notation);
    app.rubikCube.move(notation);
    if (cube)
    {
        app.layerGroup.groupAll(layerRorationAxis, app.cubeletModels);
    }
    else
    {
        app.layerGroup.group(layerRorationAxis, axisValue, app.cubeletModels);
    }
    const p = app.prevFaceButton.disabled;
    app.prevFaceButton.disabled = true;
    const n = app.nextFaceButton.disabled;
    app.nextFaceButton.disabled = true;
    await rotationTransition(layerRorationAxis, rotationRad);
    app.prevFaceButton.disabled = p;
    app.nextFaceButton.disabled = n;
}

// Used by e.g. nextFaceOnClick().
function updateSolveButton()
{
    let validCounts = true;
    // UFRBLD is the order of the color picker cells.
    ['U', 'F', 'R', 'B', 'L', 'D'].forEach(face => {
        const count = app.faces.filter(i => i === face).length;
        if (count != 9)
        {
            validCounts = false;
        }
        const div = document.getElementById('cell-div-' + face);
        div.innerText = String(count);
    });

    if (app.pickingFace == 5 && validCounts)
    {
        app.solveButton.disabled = false;
    }
    else
    {
        app.solveButton.disabled = true;
    }
}

// Used by createPage().
async function nextFaceOnClick()
{
    if (app.solution.length)
    {
        const notation = app.solution[app.solutionIndex];
        app.solutionIndex++;
        if (app.solutionIndex == app.solution.length)
        {
            app.nextFaceButton.disabled = true;
        }
        updateCounterSpan();
        await rotate(notation, false);
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
    updateCounterSpan();
    updateSolveButton();

    await rotate(notation);
}

// RubikResult represents the result from the solver.
interface RubikResult
{
    solution: string;
    error: string;
}

// Used by createPage().
async function solveOnClick()
{
    const url =
        'https://share.vmiklos.hu/apps/rubik/?facelet=' + app.faces.join('');
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
        await rotate('U');
        app.camera.position.y = 2.5;
        app.camera.rotation.x = -Math.PI / 6;
        app.prevFaceButton.disabled = true;
        app.nextFaceButton.disabled = false;
        updateCounterSpan();
    }
    catch (reason)
    {
        const p = document.getElementById('p-error');
        p.innerText = 'failed to fetch from solver: ' + reason;
    }
}

// Used by createPage().
async function prevFaceOnClick()
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
    updateCounterSpan();

    await rotate(notation);
}

// Used by createPage().
function createPickerCell(row: HTMLTableRowElement, cName: string,
                          cValue: string)
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

function updateCounterSpan()
{
    if (app.solution.length)
    {
        app.counterSpan.innerText = String(app.solutionIndex + 1) + ' / ' +
                                    (app.solution.length + 1) + ' ';
        return;
    }

    app.counterSpan.innerText = String(app.pickingFace + 1) + ' / 6 ';
}

// Creates the initial DOM nodes once the empty DOM is ready.
function createPage()
{
    document.body.style.backgroundColor = '#ffffff';
    // Create our page: the cube.
    app.scene.background = new THREE.Color('#ffffff');
    app.camera.position.x = 0;
    app.camera.position.y = 0;
    app.camera.position.z = 5;
    app.camera.aspect = app.screenWidth / app.screenHeight;
    app.camera.updateProjectionMatrix();
    app.renderer.setSize(app.screenWidth, app.screenHeight);
    app.renderer.setPixelRatio(window.devicePixelRatio);
    document.body.appendChild(app.renderer.domElement);
    app.scene.add(app.rubikCube.model);
    app.scene.add(app.layerGroup);

    // Color picker table: White, green, red; then blue, orange, yellow; 10%
    // height in total.
    const colorsTable = document.createElement('table');
    colorsTable.style.marginLeft = 'auto';
    colorsTable.style.marginRight = 'auto';
    document.body.appendChild(colorsTable);
    const colorsRow1 = document.createElement('tr');
    colorsTable.appendChild(colorsRow1);
    createPickerCell(colorsRow1, 'U', c['U']);
    createPickerCell(colorsRow1, 'F', c['F']);
    createPickerCell(colorsRow1, 'R', c['R']);
    const colorsRow2 = document.createElement('tr');
    colorsTable.appendChild(colorsRow2);
    createPickerCell(colorsRow2, 'B', c['B']);
    createPickerCell(colorsRow2, 'L', c['L']);
    createPickerCell(colorsRow2, 'D', c['D']);

    const buttons = document.createElement('p');
    buttons.style.textAlign = 'center';
    document.body.appendChild(buttons);
    app.counterSpan = document.createElement('span');
    updateCounterSpan();
    buttons.appendChild(app.counterSpan);
    app.prevFaceButton = document.createElement('input');
    app.prevFaceButton.type = 'button';
    app.prevFaceButton.value = '< prev';
    app.prevFaceButton.onclick = prevFaceOnClick;
    app.prevFaceButton.disabled = true;
    buttons.appendChild(app.prevFaceButton);
    buttons.appendChild(document.createTextNode(' '));
    app.nextFaceButton = document.createElement('input');
    app.nextFaceButton.type = 'button';
    app.nextFaceButton.value = 'next >';
    app.nextFaceButton.onclick = nextFaceOnClick;
    buttons.appendChild(app.nextFaceButton);
    buttons.appendChild(document.createTextNode(' '));
    app.solveButton = document.createElement('input');
    app.solveButton.type = 'button';
    app.solveButton.value = '✓ solve';
    app.solveButton.onclick = solveOnClick;
    updateSolveButton();
    buttons.appendChild(app.solveButton);

    const error = document.createElement('p');
    error.id = 'p-error';
    error.style.textAlign = 'center';
    error.innerText =
        'start with the facing side: red on the facing side, yellow on the right side';
    document.body.appendChild(error);

    animate();
}

// Handles clicks on the cube (scene).
function cubeOnClick(event: MouseEvent)
{
    if (app.solution.length)
    {
        // Showing solution, not painting.
        return;
    }

    const x = (event.clientX / app.screenWidth) * 2 - 1;
    const y = -(event.clientY / app.screenHeight) * 2 + 1;
    const raycaster = new THREE.Raycaster();
    raycaster.setFromCamera(new THREE.Vector2(x, y), app.camera);
    const intersects = raycaster.intersectObjects(app.rubikCube.model.children);
    const faceOfCubelets = intersects.filter(
        (intersect) => intersect.object.name.startsWith('faceOfCubelet'));
    if (!faceOfCubelets.length)
    {
        return;
    }

    const face = faceOfCubelets[0].object as THREE.Mesh;

    // Update the facelet model.
    const cubeletIndex = Number(face.name.substr('faceOfCubelet'.length));
    // F -> R -> U -> B -> D -> L
    // Hit testing gives us a cubelet index. Depending on what face we see,
    // different cubelet indexes (0..26) refer to different face indexes
    // (0..53). We need face indexes to construct a facelet string, which will
    // be the input for the solver.
    const cubeToFaceMap: {[index: number]: {[index: number]: number}} = {
        0 : {
            // F
            6 : 18,
            7 : 19,
            8 : 20,
            15 : 21,
            16 : 22,
            17 : 23,
            24 : 24,
            25 : 25,
            26 : 26,
        },
        1 : {
            // R
            8 : 9,
            5 : 10,
            2 : 11,
            17 : 12,
            14 : 13,
            11 : 14,
            26 : 15,
            23 : 16,
            20 : 17,
        },
        2 : {
            // U
            0 : 0,
            1 : 1,
            2 : 2,
            3 : 3,
            4 : 4,
            5 : 5,
            6 : 6,
            7 : 7,
            8 : 8,
        },
        3 : {
            // B
            2 : 45,
            1 : 46,
            0 : 47,
            11 : 48,
            10 : 49,
            9 : 50,
            20 : 51,
            19 : 52,
            18 : 53,
        },
        4 : {
            // D
            24 : 27,
            25 : 28,
            26 : 29,
            21 : 30,
            22 : 31,
            23 : 32,
            18 : 33,
            19 : 34,
            20 : 35,
        },
        5 : {
            // L
            0 : 36,
            3 : 37,
            6 : 38,
            9 : 39,
            12 : 40,
            15 : 41,
            18 : 42,
            21 : 43,
            24 : 44,
        },
    };
    const faceIndex = cubeToFaceMap[app.pickingFace][cubeletIndex];
    app.faces[faceIndex] = app.colorName;
    updateSolveButton();

    // Update the view.
    face.material = new THREE.MeshLambertMaterial(
        {emissive : app.colorValue, transparent : true});
}

// Calls render() periodically.
function animate(time?: number)
{
    requestAnimationFrame(animate);
    TWEEN.update(time);
    app.renderer.render(app.scene, app.camera);
}

// Encapsulates global variables.
class App
{
    // Painting the faces alterns this. Format:
    // UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB
    // Once this has no X in it, we can send it to the solver.
    faces: string[];

    // The solution we got from the solver. If empty, then we paint faces.
    // Otherwise we show a solution.
    solution: string[] = [];
    // If solution is not empty we show this state.
    // If solution has N moves, this goes from 0..N (inclusive).
    solutionIndex = 0;

    rubikCube: RubikCubeModel;
    // 27 children.
    cubeletModels: THREE.Object3D<THREE.Event>[];
    renderer = new THREE.WebGLRenderer();
    scene = new THREE.Scene();
    screenWidth = window.innerWidth;
    screenHeight = 480;
    camera: THREE.PerspectiveCamera = null;
    layerGroup = new LayerModel();

    // We want to paint the face to colorName / colorValue.
    colorName = 'U';
    colorValue = c['U'];

    colorPickerCells: HTMLTableCellElement[] = [];
    prevFaceButton: HTMLInputElement;
    nextFaceButton: HTMLInputElement;
    solveButton: HTMLInputElement;

    counterSpan: HTMLSpanElement;

    // The picker is used on this face: 0..5 (FRUBDL).
    pickingFace = 0;

    constructor()
    {
        let faces = 'XXXXXXXXXXXXXRXXXXXXXXFXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX';
        const urlParams = new URLSearchParams(window.location.search);
        const facesParam = urlParams.get('faces');
        if (facesParam != null)
        {
            faces = facesParam;
        }

        this.faces = [...faces ];
        this.rubikCube = new RubikCubeModel(faces);
        this.cubeletModels = this.rubikCube.model.children;
        this.renderer.domElement.addEventListener('click', cubeOnClick);
        this.camera = new THREE.PerspectiveCamera(
            75, this.screenWidth / this.screenHeight, 0.1, 30);

        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        document.addEventListener("DOMContentLoaded",
                                  async function() { createPage(); });

        window.app = this;
    }
}

const app = new App();

// vim: shiftwidth=4 softtabstop=4 expandtab:
