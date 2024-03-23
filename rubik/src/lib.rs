/*
 * Copyright 2024 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Library, related to <https://en.wikipedia.org/wiki/Rubik%27s_Cube>.

use rand::Rng as _;

/// Produces a scramble, i.e. rotate the cube in 24 random steps.
pub fn shuffle() -> anyhow::Result<String> {
    let mut ret: Vec<String> = Vec::new();
    let mut prev_side = "".to_string();
    for step in 1..25 {
        let mut side;
        loop {
            // Randomly pick one side of the cube.
            side = match rand::thread_rng().gen_range(1..7) {
                1 => "F",
                2 => "B",
                3 => "R",
                4 => "L",
                5 => "U",
                6 => "D",
                _ => {
                    unreachable!();
                }
            }
            .to_string();
            if side != prev_side {
                break;
            }
            // Side would be the same as the previous, try again.
        }
        prev_side = side.to_string();
        // Randomly pick a direction.
        let direction = match rand::thread_rng().gen_range(1..4) {
            1 => " ",
            2 => "'",
            3 => "2",
            _ => {
                unreachable!();
            }
        };
        ret.push(format!("{side}{direction} "));
        if step % 4 == 0 {
            ret.push(" ".to_string());
        }
        if step % 12 == 0 {
            ret.push("\n".into());
        }
    }
    Ok(ret.join(""))
}
