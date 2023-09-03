/*
 * Copyright 2013 Joe Whitfield-Seed
 *
 * SPDX-License-Identifier: MIT
 */

import * as THREE from 'three';

// Keys of a THREE.Vector3.
type Axis = 'x'|'y'|'z';

// Starting part of a notation (no "'" or "2").
type NotationBase = 'L'|'R'|'D'|'U'|'B'|'F';

interface Move
{
    cube: THREE.Mesh, vector: THREE.Vector3, axis: Axis, direction: number,
        all: boolean,
}

interface Cubelet extends
    THREE.Mesh<THREE.BoxGeometry, THREE.MeshLambertMaterial[]>
{
    rubikPosition: THREE.Vector3;
}

// From <https://github.com/joews/rubik-js>.
class Rubik
{
    scene: THREE.Scene;

    camera: THREE.PerspectiveCamera;

    renderer: THREE.WebGLRenderer;

    clickVector: THREE.Vector3;

    transitions: {[index: string]: {[index: string]: string}};

    cubeSize = 3;

    allCubes: THREE.Mesh[];

    moveQueue: Move[];

    currentMove: Move;

    isMoving: boolean;

    moveAxis: Axis;

    moveDirection: number;

    rotationSpeed = 0.2;

    pivot: THREE.Object3D;

    activeGroup: THREE.Mesh[];

    SCREEN_HEIGHT: number;
    SCREEN_WIDTH: number;

    cubeletTexture: THREE.CanvasTexture;

    init(faceletStr: string, colorValues: {[index: string]: string})
    {

        this.SCREEN_WIDTH = window.innerWidth;
        this.SCREEN_HEIGHT = 480;

        /*** three.js boilerplate ***/
        this.scene = new THREE.Scene();
        this.camera = new THREE.PerspectiveCamera(
            75, this.SCREEN_WIDTH / this.SCREEN_HEIGHT, 0.1, 30);
        this.renderer = new THREE.WebGLRenderer({antialias : true});

        this.renderer.setClearColor('#ffffff', 1.0);
        this.renderer.setSize(this.SCREEN_WIDTH, this.SCREEN_HEIGHT);
        this.renderer.shadowMap.enabled = true;
        document.body.appendChild(this.renderer.domElement);

        this.camera.position.x = 0;
        this.camera.position.y = 0;
        this.camera.position.z = 15;

        /*** Click handling ***/

        // For each mouse down, track the position of the cube that
        //  we clicked (clickVector)
        this.clickVector = null;

        // Matrix of the axis that we should rotate for
        // each face-drag action
        //    F a c e
        // D    X Y Z
        // r  X - Z Y
        // a  Y Z - X
        // g  Z Y X -
        this.transitions = {
            'x' : {'y' : 'z', 'z' : 'y'},
            'y' : {'x' : 'z', 'z' : 'x'},
            'z' : {'x' : 'y', 'y' : 'x'}
        }

        /*** Build 27 cubes ***/
        const increment = this.cubeSize;
        this.allCubes = [];

        // https://discourse.threejs.org/t/how-to-add-solid-border-into-cube-edge-in-three-js/47878/2
        const canvas = document.createElement('canvas');
        canvas.width = 512;
        canvas.height = 512;
        const canvasContext = canvas.getContext('2d');
        canvasContext.fillStyle = 'white';
        canvasContext.fillRect(0, 0, 512, 512);
        canvasContext.strokeStyle = 'black';
        canvasContext.lineWidth = 32;
        canvasContext.strokeRect(16, 16, 512 - 32, 512 - 32);
        this.cubeletTexture = new THREE.CanvasTexture(canvas);

        const positionOffset = 1;
        let cubeIndex = 0;
        for (let i = 0; i < 3; i++)
        {
            for (let j = 0; j < 3; j++)
            {
                for (let k = 0; k < 3; k++)
                {

                    const x = (i - positionOffset) * increment,
                          y = (j - positionOffset) * increment,
                          z = (k - positionOffset) * increment;

                    this.newCube(x, y, z, cubeIndex, faceletStr, colorValues);
                    cubeIndex++;
                }
            }
        }

        // Maintain a queue of moves so we can perform compound actions like
        // shuffle and solve
        this.moveQueue = [];
        this.currentMove = null;

        // Are we in the middle of a transition?
        this.isMoving = false;
        this.moveAxis = null;
        this.moveDirection = null;

        // http://stackoverflow.com/questions/20089098/three-js-adding-and-removing-children-of-rotated-objects
        this.pivot = new THREE.Object3D();
        this.activeGroup = [];
    }

    faceTurn(notation: string)
    {
        const all = false;
        this.faceTurnImpl(notation, all);
    }

    cubeTurn(notation: string)
    {
        const all = true;
        this.faceTurnImpl(notation, all);
    }

    isMouseOverCube(mouseX: number, mouseY: number)
    {
        const directionVector = new THREE.Vector3();

        // Normalise mouse x and y
        const x = (mouseX / this.SCREEN_WIDTH) * 2 - 1;
        const y = -(mouseY / this.SCREEN_HEIGHT) * 2 + 1;

        directionVector.set(x, y, 1);

        directionVector.sub(this.camera.position);
        directionVector.normalize();
        const raycaster = new THREE.Raycaster();
        raycaster.setFromCamera(new THREE.Vector2(x, y), this.camera);

        return raycaster.intersectObjects(this.allCubes, true).length > 0;
    }

    getColorValue(faceIndex: string,
                  colorValues: {[index: string]: string}): string
    {
        if (faceIndex === undefined)
        {
            return colorValues['X'];
        }

        return colorValues[faceIndex];
    }

    newCube(x: number, y: number, z: number, index: number, faceletStr: string,
            colorValues: {[index: string]: string})
    {
        // Face indexes. Facelet string order is U, R, F, D, L, B.
        const NP: number = undefined; // not painted
        const U1 = 0, U2 = 1, U3 = 2, U4 = 3, U5 = 4;
        const U6 = 5, U7 = 6, U8 = 7, U9 = 8;
        const R1 = 9, R2 = 10, R3 = 11, R4 = 12, R5 = 13;
        const R6 = 14, R7 = 15, R8 = 16, R9 = 17;
        const F1 = 18, F2 = 19, F3 = 20, F4 = 21, F5 = 22;
        const F6 = 23, F7 = 24, F8 = 25, F9 = 26;
        const D1 = 27, D2 = 28, D3 = 29, D4 = 30, D5 = 31;
        const D6 = 32, D7 = 33, D8 = 34, D9 = 35;
        const L1 = 36, L2 = 37, L3 = 38, L4 = 39, L5 = 40;
        const L6 = 41, L7 = 42, L8 = 43, L9 = 44;
        const B1 = 45, B2 = 46, B3 = 47, B4 = 48, B5 = 49;
        const B6 = 50, B7 = 51, B8 = 52, B9 = 53;
        const cubeGeometry =
            new THREE.BoxGeometry(this.cubeSize, this.cubeSize, this.cubeSize);
        const cubeIndexToFaceIndexes: {[index: number]: number[]} = {
            0 : [ NP, L7, NP, D7, NP, B9 ],
            1 : [ NP, L8, NP, D4, NP, NP ],
            2 : [ NP, L9, NP, D1, F7, NP ],
            3 : [ NP, L4, NP, NP, NP, B6 ],
            4 : [ NP, L5, NP, NP, NP, NP ],
            5 : [ NP, L6, NP, NP, F4, NP ],
            6 : [ NP, L1, U1, NP, NP, B3 ],
            7 : [ NP, L2, U4, NP, NP, NP ],
            8 : [ NP, L3, U7, NP, F1, NP ],
            9 : [ NP, NP, NP, D8, NP, B8 ],
            10 : [ NP, NP, NP, D5, NP, NP ],
            11 : [ NP, NP, NP, D2, F8, NP ],
            12 : [ NP, NP, NP, NP, NP, B5 ],
            13 : [ NP, NP, NP, NP, NP, NP ],
            14 : [ NP, NP, NP, NP, F5, NP ],
            15 : [ NP, NP, U2, NP, NP, B2 ],
            16 : [ NP, NP, U5, NP, NP, NP ],
            17 : [ NP, NP, U8, NP, F2, NP ],
            18 : [ R9, NP, NP, D9, NP, B7 ],
            19 : [ R8, NP, NP, D6, NP, NP ],
            20 : [ R7, NP, NP, D3, F9, NP ],
            21 : [ R6, NP, NP, NP, NP, B4 ],
            22 : [ R5, NP, NP, NP, NP, NP ],
            23 : [ R4, NP, NP, NP, F6, NP ],
            24 : [ R3, NP, U3, NP, NP, B1 ],
            25 : [ R2, NP, U6, NP, NP, NP ],
            26 : [ R1, NP, U9, NP, F3, NP ],
        };
        const faceIndexes = cubeIndexToFaceIndexes[index];
        const cubeMaterials = [];
        for (let i = 0; i < 6; i++)
        {
            cubeMaterials.push(new THREE.MeshLambertMaterial({
                emissive :
                    this.getColorValue(faceletStr[faceIndexes[i]], colorValues),
                emissiveMap : this.cubeletTexture
            }));
        }
        const cube = new THREE.Mesh(cubeGeometry, cubeMaterials) as Cubelet;
        cube.castShadow = true;
        cube.name = 'cubelet' + index;

        cube.position.x = x;
        cube.position.y = y;
        cube.position.z = z;
        cube.rubikPosition = cube.position.clone();

        this.scene.add(cube);
        this.allCubes.push(cube);
    }

    /*** Manage transition states ***/

    nearlyEqual(a: number, b: number): boolean
    {
        const d = 0.001;
        return Math.abs(a - b) <= d;
    }

    // Select the plane of cubes that aligns with clickVector
    //  on the given axis
    setActiveGroup(axis: Axis, all: boolean)
    {
        if (this.clickVector)
        {
            this.activeGroup = [];

            for (const i of this.allCubes)
            {
                const cube = i as Cubelet;
                if (this.nearlyEqual(cube.rubikPosition[axis],
                                     this.clickVector[axis]) ||
                    all)
                {
                    this.activeGroup.push(cube);
                }
            }
        }
    }

    pushMove(cube: THREE.Mesh, clickVector: THREE.Vector3, axis: Axis,
             direction: number, all: boolean)
    {
        // 'all' means not just the face, but the entire cube.
        if (all === undefined)
        {
            all = false;
        }
        this.moveQueue.push({
            cube : cube,
            vector : clickVector,
            axis : axis,
            direction : direction,
            all : all
        });
    }

    startNextMove()
    {
        const nextMove = this.moveQueue.pop();

        if (nextMove)
        {
            this.clickVector = nextMove.vector;

            const direction = nextMove.direction || 1;
            const axis = nextMove.axis;
            const all = nextMove.all;

            if (this.clickVector)
            {

                if (!this.isMoving)
                {
                    this.isMoving = true;
                    this.moveAxis = axis;
                    this.moveDirection = direction;

                    this.setActiveGroup(axis, all);

                    this.pivot.rotation.set(0, 0, 0);
                    this.pivot.updateMatrixWorld();
                    this.scene.add(this.pivot);

                    for (const e of this.activeGroup)
                    {
                        // Attach.
                        e.applyMatrix4(new THREE.Matrix4()
                                           .copy(this.pivot.matrixWorld)
                                           .invert());
                        this.scene.remove(e);
                        this.pivot.add(e);
                    }

                    this.currentMove = nextMove;
                }
            }
        }
    }

    doMove()
    {
        // Move a quarter turn then stop
        if (this.pivot.rotation[this.moveAxis] >= Math.PI / 2)
        {
            // Compensate for overshoot. TODO: use a tweening library
            this.pivot.rotation[this.moveAxis] = Math.PI / 2;
            this.moveComplete();
        }
        else if (this.pivot.rotation[this.moveAxis] <= Math.PI / -2)
        {
            this.pivot.rotation[this.moveAxis] = Math.PI / -2;
            this.moveComplete()
        }
        else
        {
            this.pivot.rotation[this.moveAxis] +=
                (this.moveDirection * this.rotationSpeed);
        }
    }

    moveComplete()
    {
        this.isMoving = false;
        this.moveAxis, this.moveDirection = undefined;
        this.clickVector = undefined;

        this.pivot.updateMatrixWorld();
        this.scene.remove(this.pivot);
        for (const i of this.activeGroup)
        {
            const cube = i as Cubelet;
            cube.updateMatrixWorld();

            cube.rubikPosition = cube.position.clone();
            cube.rubikPosition.applyMatrix4(this.pivot.matrixWorld);

            // Detach.
            cube.applyMatrix4(this.pivot.matrixWorld);
            this.pivot.remove(cube);
            this.scene.add(cube);
        }

        // Are there any more queued moves?
        this.startNextMove();
    }

    /*** Util ***/
    faceTurnImpl(notation: string, all: boolean)
    {
        // https://meep.cubing.net/wcanotation.html
        const notationBase = notation[0] as NotationBase;
        const notationExtra = notation[1];
        const faceTurnToMove: {
            [index: string]: {cubeIndex: number, axis: Axis, direction: number}
        } = {
            L : {cubeIndex : 4, axis : 'x', direction : 1},
            R : {cubeIndex : 22, axis : 'x', direction : -1},
            D : {cubeIndex : 10, axis : 'y', direction : 1},
            U : {cubeIndex : 16, axis : 'y', direction : -1},
            B : {cubeIndex : 12, axis : 'z', direction : 1},
            F : {cubeIndex : 14, axis : 'z', direction : -1},
        };
        const move = faceTurnToMove[notationBase];
        let twice = false;
        if (notationExtra)
        {
            if (notationExtra == `'`)
            {
                move.direction *= -1;
            }
            else if (notationExtra == '2')
            {
                twice = true;
            }
        }
        const cube = this.allCubes[move.cubeIndex];
        this.pushMove(cube, cube.position.clone(), move.axis, move.direction,
                      all);
        if (twice)
        {
            this.pushMove(cube, cube.position.clone(), move.axis,
                          move.direction, all);
        }
        this.startNextMove();
    }
}

export {Rubik};

// vim: shiftwidth=4 softtabstop=4 expandtab:
