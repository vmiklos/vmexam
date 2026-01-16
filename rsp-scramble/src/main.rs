/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! This is a simple scambler for rotate & slide puzzle games.

use rand::Rng as _;

fn main() {
    let mut ret: Vec<String> = Vec::new();
    for step in 1..25 {
        // Layer index, rotation count, slide index.
        // Rotate this layer: 1-4.
        let layer = rand::rng().random_range(1..5);
        // Rotate the layer by this many slots: 1-7.
        let rotation = rand::rng().random_range(1..8);
        // Slide the pieces so the free slot is on this layer, ignoring the free slot: 1-3.
        let slide = rand::rng().random_range(1..4);
        ret.push(format!("l{layer}r{rotation}s{slide} "));
        if step % 12 == 0 {
            ret.push("\n".into());
        }
    }
    print!("{}", ret.join(""));
}
