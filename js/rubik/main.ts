/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

import * as THREE from 'three';

// eslint-disable-next-line @typescript-eslint/no-unused-vars
document.addEventListener("DOMContentLoaded", async function(event) {
    // Create our page.
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(50, 640 / 480);
    const renderer = new THREE.WebGLRenderer();
    renderer.setSize(640, 480);
    document.body.appendChild(renderer.domElement);

    const geometry = new THREE.BoxGeometry(1, 1, 1);
    const material = new THREE.MeshBasicMaterial({color : 0x00ff00});
    const cube = new THREE.Mesh(geometry, material);
    scene.add(cube);
    camera.position.z = 5;

    // 30 degrees in both directions.
    cube.rotation.x += 3.14 / 6;
    cube.rotation.y += 3.14 / 6;
    renderer.render(scene, camera);
});

// vim: shiftwidth=4 softtabstop=4 expandtab:
