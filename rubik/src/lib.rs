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

fn pick_side(index: u8, lang: &str, megaminx: bool) -> String {
    if lang == "hu" {
        match index {
            1 => "S",
            2 => "T",
            3 => "K",
            4 => "N",
            5 => "F",
            6 => "L",
            _ => {
                unreachable!();
            }
        }
        .to_string()
    } else if megaminx {
        match index {
            1 => "white",
            2 => "red",
            3 => "green",
            4 => "purple",
            5 => "yellow",
            6 => "blue",
            7 => "grey",
            8 => "orange",
            9 => "limegreen",
            10 => "pink",
            11 => "lightyellow",
            12 => "darkblue",
            _ => {
                unreachable!();
            }
        }
        .to_string()
    } else {
        match index {
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
        .to_string()
    }
}

/// Produces a scramble, i.e. rotate the cube in 24 random steps.
///
/// * `wide` - allow wide turns, useful for 4x4, not relevant for 3x3.
/// * `megaminx` - allow megaminx turns, 12 sides and 4 kind of turns
pub fn shuffle(lang: &str, wide: bool, megaminx: bool) -> anyhow::Result<String> {
    let mut ret: Vec<String> = Vec::new();
    let mut prev_side = "".to_string();
    let steps = if megaminx { 49 } else { 25 };
    for step in 1..steps {
        let mut side;
        loop {
            // Randomly pick one side of the cube.
            let sides = if megaminx { 13 } else { 7 };
            side = pick_side(rand::rng().random_range(1..sides), lang, megaminx);
            if side != prev_side {
                break;
            }
            // Side would be the same as the previous, try again.
        }
        prev_side = side.to_string();
        // Wide turn?
        let wide = if wide {
            match rand::rng().random_range(1..4) {
                1 => "",
                2 => "w",
                3 => {
                    side = side.to_lowercase();
                    ""
                }
                _ => {
                    unreachable!();
                }
            }
        } else {
            ""
        };
        // Randomly pick a direction.
        let direction = if megaminx {
            rand::rng().random_range(1..5).to_string()
        } else {
            match rand::rng().random_range(1..4) {
                1 => "",
                2 => "'",
                3 => "2",
                _ => {
                    unreachable!();
                }
            }
            .to_string()
        };
        let turn = format!("{side}{wide}{direction}");
        ret.push(if megaminx {
            format!("{turn: <13}")
        } else {
            format!("{turn: <4}")
        });
        if step % 4 == 0 {
            ret.push(" ".to_string());
        }
        if step % 12 == 0 {
            ret.push("\n".into());
        }
    }
    Ok(ret.join(""))
}
