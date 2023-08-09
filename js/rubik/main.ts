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

const c: {[index: string]: string} = {
    'U' : '#01499b', // Blue
    'F' : '#b12118', // Red
    'R' : '#e1c501', // Yellow
    'B' : '#ee3300', // Orange
    'L' : '#b9d0d8', // White
    'D' : '#028d76', // Green
    'X' : '#141517', // Gray
};

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

interface Cubelet
{
    x: number, y: number, z: number, num: number, type: string,
        color?: {[index: string]: string},
}

class RubikCube
{
    cubelets: Cubelet[] = [];
    colors: string[];
    constructor(colorStr?: string)
    {
        if (!colorStr)
        {
            colorStr = 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX';
        }
        this.colors = colorStr.trim().split('');

        this.generateCoords();
        this.generateColors();
    }

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

    asString() { return this.colors.join(''); }

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

export function roundedPlane(x = 0, y = 0, width = 0.9, height = 0.9,
                             radius = 0.1)
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

interface CubeletModel extends THREE.Mesh
{
    cubeType?: string;
    num?: number;
    initPosition?: THREE.Vector3;
}

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

    dispose()
    {
        for (const cubeletModel of (this.model.children as CubeletModel[]))
        {
            if (cubeletModel.material instanceof THREE.Material)
            {
                cubeletModel.material.dispose();
            }
            cubeletModel.geometry.dispose();
            for (const plan of (cubeletModel.children as THREE.Mesh[]))
            {
                if (plan.material instanceof THREE.Material)
                {
                    plan.material.dispose();
                }
                (plan as THREE.Mesh).geometry.dispose();
            }
        }
    }
}

export type Axis = 'x'|'y'|'z';

export type AxisValue = number;

export type Toward = 1|- 1;

export class LayerModel extends THREE.Group
{
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

    initRotation()
    {
        this.rotation.x = 0;
        this.rotation.y = 0;
        this.rotation.z = 0;
    }

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

export type NotationBase = 'L'|'R'|'D'|'U'|'B'|'F';
export type NotationExtra = ''|`'`|'2';

const axisTable: {[key in NotationBase]: [ Axis, AxisValue, Toward ]} = {
    L : [ 'x', -1, 1 ],
    R : [ 'x', 1, -1 ],
    D : [ 'y', -1, 1 ],
    U : [ 'y', 1, -1 ],
    B : [ 'z', -1, 1 ],
    F : [ 'z', 1, -1 ],
};

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

async function rotationTransition(axis: Axis, endRad: number)
{
    await layerGroup.rotationAnimation(axis, endRad);
    layerGroup.ungroup(rubikCube.model);
    layerGroup.initRotation();
}

async function rotateAll()
{
    const notation = 'U';
    const [layerRorationAxis, /*axisValue*/, rotationRad] =
        toRotation(notation);
    rubikCube.move(notation);
    layerGroup.groupAll(layerRorationAxis, cubeletModels);
    await rotationTransition(layerRorationAxis, rotationRad);
}

function createPickerCell(row: HTMLTableRowElement, cName: string,
                          cValue: string)
{
    const cell = document.createElement('td');
    cell.style.width = String(screenWidth / 30) + 'px';
    cell.style.height = String(screenHeight / 20) + 'px';
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
        colorName = cName;
        colorValue = cValue;
        colorPickerCells.forEach(function(c) {
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
    row.appendChild(cell);
    colorPickerCells.push(cell);
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
document.addEventListener("DOMContentLoaded", async function(event) {
    document.body.style.backgroundColor = '#ffffff';
    // Create our page: the cube.
    scene.background = new THREE.Color('#ffffff');
    camera.position.x = 0;
    camera.position.y = 0;
    camera.position.z = 5;
    camera.aspect = screenWidth / screenHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(screenWidth, screenHeight);
    renderer.setPixelRatio(window.devicePixelRatio);
    document.body.appendChild(renderer.domElement);
    scene.add(rubikCube.model);
    scene.add(layerGroup);
    renderer.render(scene, camera);

    const rotateAllButton = document.createElement('input');
    rotateAllButton.type = 'button';
    rotateAllButton.value = 'rotate all';
    rotateAllButton.onclick = rotateAll;
    document.body.appendChild(rotateAllButton);

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
});

function cubeOnClick(event: MouseEvent)
{
    const x = (event.clientX / screenWidth) * 2 - 1;
    const y = -(event.clientY / screenHeight) * 2 + 1;
    const raycaster = new THREE.Raycaster();
    raycaster.setFromCamera(new THREE.Vector2(x, y), camera);
    const intersects = raycaster.intersectObjects(rubikCube.model.children);
    const faceOfCubelets = intersects.filter(
        (intersect) => intersect.object.name.startsWith('faceOfCubelet'));
    if (!faceOfCubelets.length)
    {
        return;
    }

    let face = faceOfCubelets[0].object as THREE.Mesh;

    // Update the facelet model.
    let cubeletIndex = Number(face.name.substr('faceOfCubelet'.length));
    // Assume that we always see 'F' for now.
    let cubeToFaceMap: {[index: number]: number} = {
        6 : 18,
        7 : 19,
        8 : 20,
        15 : 21,
        16 : 22,
        17 : 23,
        24 : 24,
        25 : 25,
        26 : 26,
    };
    let faceIndex = cubeToFaceMap[cubeletIndex];
    faces[faceIndex] = colorName;

    // Update the view.
    face.material = new THREE.MeshLambertMaterial(
        {emissive : colorValue, transparent : true});
}

function animate(time?: number)
{
    requestAnimationFrame(animate);
    TWEEN.update(time);
    renderer.render(scene, camera);
}

const rubikCube = new RubikCubeModel();
// 27 children.
const cubeletModels = rubikCube.model.children;
const renderer = new THREE.WebGLRenderer();
renderer.domElement.addEventListener('click', cubeOnClick);
const scene = new THREE.Scene();
const screenWidth = window.innerWidth;
const screenHeight = window.innerHeight * 0.8;
const camera =
    new THREE.PerspectiveCamera(75, screenWidth / screenHeight, 0.1, 30);
const layerGroup = new LayerModel();
// UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB
let faces = [...'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX' ];

// We want to paint the face to colorName / colorValue.
let colorName = 'U';
let colorValue = c['U'];
let colorPickerCells: HTMLTableCellElement[] = [];

animate();

// vim: shiftwidth=4 softtabstop=4 expandtab:
