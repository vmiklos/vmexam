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
        const cubeGeometry =
            new THREE.BoxGeometry(this.cubeSize, this.cubeSize, this.cubeSize);
        // R, L, U, D, F, B are the columns.
        // Facelet string order is U, R, F, D, L, B.
        const cubeIndexToFaceIndexes: {[index: number]: number[]} = {
            0 : [
                undefined, /*L7=*/ 42, undefined, /*D7=*/ 33, undefined,
                /*B9=*/ 53
            ],
            1 : [
                undefined, /*L8=*/ 43, undefined, /*D4=*/ 30, undefined,
                undefined
            ],
            2 : [
                undefined, /*L9=*/ 44, undefined, /*D1=*/ 27, /*F7=*/ 24,
                undefined
            ],
            3 : [
                undefined, /*L4=*/ 39, undefined, undefined, undefined,
                /*B6=*/ 50
            ],
            4 : [
                undefined, /*L5=*/ 40, undefined, undefined, undefined,
                undefined
            ],
            5 : [
                undefined, /*L6=*/ 41, undefined, undefined, /*F4=*/ 21,
                undefined
            ],
            6 : [
                undefined, /*L1=*/ 36, /*U1=*/ 0, undefined, undefined,
                /*B3=*/ 47
            ],
            7 : [
                undefined, /*L2=*/ 37, /*U4=*/ 3, undefined, undefined,
                undefined
            ],
            8 : [
                undefined, /*L3=*/ 38, /*U7=*/ 6, undefined, /*F1=*/ 18,
                undefined
            ],
            9 : [
                undefined, undefined, undefined, /*D8=*/ 34, undefined,
                /*B8=*/ 52
            ],
            10 : [
                undefined, undefined, undefined, /*D5=*/ 31, undefined,
                undefined
            ],
            11 : [
                undefined, undefined, undefined, /*D2=*/ 28, /*F8=*/ 25,
                undefined
            ],
            12 : [
                undefined, undefined, undefined, undefined, undefined,
                /*B5=*/ 49
            ],
            13 : [
                undefined, undefined, undefined, undefined, undefined, undefined
            ],
            14 : [
                undefined, undefined, undefined, undefined, /*F5=*/ 22,
                undefined
            ],
            15 : [
                undefined, undefined, /*U2=*/ 1, undefined, undefined,
                /*B2=*/ 46
            ],
            16 : [
                undefined, undefined, /*U5=*/ 4, undefined, undefined, undefined
            ],
            17 : [
                undefined, undefined, /*U8=*/ 7, undefined, /*F2=*/ 19,
                undefined
            ],
            18 : [
                /*R9=*/ 17, undefined, undefined, /*D9=*/ 35, undefined,
                /*B7=*/ 51
            ],
            19 : [
                /*R8=*/ 16, undefined, undefined, /*D6=*/ 32, undefined,
                undefined
            ],
            20 : [
                /*R7=*/ 15, undefined, undefined, /*D3=*/ 29, /*F9=*/ 26,
                undefined
            ],
            21 : [
                /*R6=*/ 14, undefined, undefined, undefined, undefined,
                /*B4=*/ 48
            ],
            22 : [
                /*R5=*/ 13, undefined, undefined, undefined, undefined,
                undefined
            ],
            23 : [
                /*R4=*/ 12, undefined, undefined, undefined, /*F6=*/ 23,
                undefined
            ],
            24 : [
                /*R3=*/ 11, undefined, /*U3=*/ 2, undefined, undefined,
                /*B1=*/ 45
            ],
            25 : [
                /*R2=*/ 10, undefined, /*U6=*/ 5, undefined, undefined,
                undefined
            ],
            26 : [
                /*R1=*/ 9, undefined, /*U9=*/ 8, undefined, /*F3=*/ 20,
                undefined
            ],
        };
        const faceIndexes = cubeIndexToFaceIndexes[index];
        const cubeMaterials = [
            new THREE.MeshLambertMaterial({
                emissive :
                    this.getColorValue(faceletStr[faceIndexes[0]], colorValues),
                emissiveMap : this.cubeletTexture
            }),
            new THREE.MeshLambertMaterial({
                emissive :
                    this.getColorValue(faceletStr[faceIndexes[1]], colorValues),
                emissiveMap : this.cubeletTexture
            }),
            new THREE.MeshLambertMaterial({
                emissive :
                    this.getColorValue(faceletStr[faceIndexes[2]], colorValues),
                emissiveMap : this.cubeletTexture
            }),
            new THREE.MeshLambertMaterial({
                emissive :
                    this.getColorValue(faceletStr[faceIndexes[3]], colorValues),
                emissiveMap : this.cubeletTexture
            }),
            new THREE.MeshLambertMaterial({
                emissive :
                    this.getColorValue(faceletStr[faceIndexes[4]], colorValues),
                emissiveMap : this.cubeletTexture
            }),
            new THREE.MeshLambertMaterial({
                emissive :
                    this.getColorValue(faceletStr[faceIndexes[5]], colorValues),
                emissiveMap : this.cubeletTexture
            }),
        ];
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
