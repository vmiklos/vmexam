/*
 * Copyright 2025 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Provides the 'cube' cmdline tool.
//!
//! Problem: have 8 small cubes, all of their sides are painted using 6 colors. Goal: build a
//! single 2x2x2 cube from the small cubes so that all large sides have the same color.
//!
//! The first 5 sides are easy to do manually, the 6th side is tricky: this solver does all sides
//! for you.

const SLOT_UBL: i8 = 0;
const SLOT_UBR: i8 = 1;
const SLOT_UFR: i8 = 2;
const SLOT_UFL: i8 = 3;
const SLOT_DFL: i8 = 4;
const SLOT_DFR: i8 = 5;
const SLOT_DBR: i8 = 6;
const SLOT_DBL: i8 = 7;

const SIDE_U: i8 = 0;
const SIDE_D: i8 = 1;
const SIDE_R: i8 = 2;
const SIDE_L: i8 = 3;
const SIDE_F: i8 = 4;
const SIDE_B: i8 = 5;

const BLUE: i8 = 0;
const YELLOW: i8 = 1;
const BROWN: i8 = 2;
const GREEN: i8 = 3;
const PURPLE: i8 = 4;
const RED: i8 = 5;

fn color_to_string(color: Option<i8>) -> &'static str {
    match color {
        Some(value) => match value {
            BLUE => "blue",
            YELLOW => "yellow",
            BROWN => "brown",
            GREEN => "green",
            PURPLE => "purple",
            RED => "red",
            _ => unreachable!(),
        },
        None => "",
    }
}

struct Model {
    /// Slots: -1 or 0..7
    /// - order is: UBL, UBR, UFR, UFL, DFL, DFR, DBR, DBL
    /// - e.g. if slot 0 is 2: for UBL, use the 2nd cube
    ///
    /// X, Y and Z rotations: -1 or 0..63 (0..3 each)
    /// - e.g. if rotation 0 is 3: UBL has been rotated 3 times on the Z axis
    solution: [[i8; 8]; 2],
    /// 8 cubes (0th..7th cube), 6 sides: U D R L F B
    /// colors: 0..5 for blue..red
    /// - e.g. if 0.0 is RED, then the up of the 0th cube is red
    colors: [[i8; 6]; 8],
}

struct Position {
    /// Row in the model
    row: usize,
    /// Column in a row
    cell: usize,
}

fn create_model() -> Model {
    Model {
        solution: [
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1],
        ],
        colors: [
            [RED, PURPLE, GREEN, BLUE, BROWN, YELLOW],
            [BROWN, BLUE, GREEN, RED, YELLOW, PURPLE],
            [BROWN, PURPLE, YELLOW, GREEN, RED, BLUE],
            [BROWN, YELLOW, RED, GREEN, BLUE, PURPLE],
            [YELLOW, BLUE, GREEN, PURPLE, RED, BROWN],
            [YELLOW, GREEN, BROWN, BLUE, RED, PURPLE],
            [BLUE, BROWN, PURPLE, GREEN, RED, YELLOW],
            [BLUE, YELLOW, BROWN, GREEN, RED, PURPLE],
        ],
    }
}

fn find_empty(model: &Model) -> Option<Position> {
    for row in 0..2 {
        for cell in 0..8 {
            if model.solution[row][cell] == -1 {
                return Some(Position { row, cell });
            }
        }
    }

    None
}

fn rotate_x(sides: &mut [i8; 6]) {
    let tmp = *sides;
    sides[SIDE_U as usize] = tmp[SIDE_F as usize];
    sides[SIDE_D as usize] = tmp[SIDE_B as usize];
    // no R
    // no L
    sides[SIDE_F as usize] = tmp[SIDE_D as usize];
    sides[SIDE_B as usize] = tmp[SIDE_U as usize];
}

fn rotate_y(sides: &mut [i8; 6]) {
    let tmp = *sides;
    // no U
    // no D
    sides[SIDE_R as usize] = tmp[SIDE_B as usize];
    sides[SIDE_L as usize] = tmp[SIDE_F as usize];
    sides[SIDE_F as usize] = tmp[SIDE_R as usize];
    sides[SIDE_B as usize] = tmp[SIDE_L as usize];
}

fn rotate_z(sides: &mut [i8; 6]) {
    let tmp = *sides;
    sides[SIDE_U as usize] = tmp[SIDE_L as usize];
    sides[SIDE_D as usize] = tmp[SIDE_R as usize];
    sides[SIDE_R as usize] = tmp[SIDE_U as usize];
    sides[SIDE_L as usize] = tmp[SIDE_D as usize];
    // no F
    // no B
}

/// Input is 0..63, which contains the X, Y and Z rotations
fn parse_rotation(rotation: i8) -> (i8, i8, i8) {
    let x = rotation / 16;
    let y = rotation % 16 / 4;
    let z = rotation % 4;
    (x, y, z)
}

fn rotation_to_string(rotation: i8) -> String {
    let (x, y, z) = parse_rotation(rotation);
    format!("x: {x}, y: {y}, z: {z}")
}

fn rotate_color(colors: &[i8; 6], side: i8, rotation: i8) -> Option<i8> {
    if rotation == -1 {
        return None;
    }
    let (x_rotation, y_rotation, z_rotation) = parse_rotation(rotation);
    let mut sides = [SIDE_U, SIDE_D, SIDE_R, SIDE_L, SIDE_F, SIDE_B];
    for _ in 0..x_rotation {
        rotate_x(&mut sides);
    }
    for _ in 0..y_rotation {
        rotate_y(&mut sides);
    }
    for _ in 0..z_rotation {
        rotate_z(&mut sides);
    }
    Some(colors[sides[side as usize] as usize])
}

fn get_candidate_color(model: &Model, slot: i8, side: i8, num: i8) -> Option<i8> {
    rotate_color(
        &model.colors[model.solution[0][slot as usize] as usize],
        side,
        num,
    )
}

fn get_model_color(model: &Model, slot: i8, side: i8) -> Option<i8> {
    rotate_color(
        &model.colors[model.solution[0][slot as usize] as usize],
        side,
        model.solution[1][slot as usize],
    )
}

fn is_valid_color(
    model: &Model,
    model_slot: i8,
    candidate_pos: &Position,
    side: i8,
    candidate_rotation: i8,
) -> bool {
    let candidate_slot = candidate_pos.cell as i8;
    if let Some(model_color) = get_model_color(model, model_slot, side) {
        let candidate_color =
            match get_candidate_color(model, candidate_slot, side, candidate_rotation) {
                Some(value) => value,
                None => {
                    return false;
                }
            };
        if candidate_color != model_color {
            return false;
        }
    }
    true
}

fn is_valid(model: &Model, num: i8, pos: &Position) -> bool {
    if pos.row == 0 {
        for i in 0..8 {
            if model.solution[0][i] == num {
                return false;
            }
        }
    } else {
        match pos.cell as i8 {
            SLOT_UBL => {
                // provides U
                // provides B
                // provides L
            }
            SLOT_UBR => {
                if !is_valid_color(model, SLOT_UBL, pos, SIDE_U, num) {
                    return false;
                }
                if !is_valid_color(model, SLOT_UBL, pos, SIDE_B, num) {
                    return false;
                }
                // provides R
            }
            SLOT_UFR => {
                if !is_valid_color(model, SLOT_UBR, pos, SIDE_U, num) {
                    return false;
                }
                // provides F
                if !is_valid_color(model, SLOT_UBR, pos, SIDE_R, num) {
                    return false;
                }
            }
            SLOT_UFL => {
                if !is_valid_color(model, SLOT_UBL, pos, SIDE_U, num) {
                    return false;
                }
                if !is_valid_color(model, SLOT_UFR, pos, SIDE_F, num) {
                    return false;
                }
                if !is_valid_color(model, SLOT_UBL, pos, SIDE_L, num) {
                    return false;
                }
            }
            SLOT_DFL => {
                // provides D
                if !is_valid_color(model, SLOT_UFR, pos, SIDE_F, num) {
                    return false;
                }
                if !is_valid_color(model, SLOT_UBL, pos, SIDE_L, num) {
                    return false;
                }
            }
            SLOT_DFR => {
                if !is_valid_color(model, SLOT_DFL, pos, SIDE_D, num) {
                    return false;
                }
                if !is_valid_color(model, SLOT_UFR, pos, SIDE_F, num) {
                    return false;
                }
                if !is_valid_color(model, SLOT_UBR, pos, SIDE_R, num) {
                    return false;
                }
            }
            SLOT_DBR => {
                if !is_valid_color(model, SLOT_DFL, pos, SIDE_D, num) {
                    return false;
                }
                if !is_valid_color(model, SLOT_UBL, pos, SIDE_B, num) {
                    return false;
                }
                if !is_valid_color(model, SLOT_UBR, pos, SIDE_R, num) {
                    return false;
                }
            }
            SLOT_DBL => {
                if !is_valid_color(model, SLOT_DFL, pos, SIDE_D, num) {
                    return false;
                }
                if !is_valid_color(model, SLOT_UBL, pos, SIDE_B, num) {
                    return false;
                }
                if !is_valid_color(model, SLOT_UBL, pos, SIDE_L, num) {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }
    true
}

fn solve(model: &mut Model) -> bool {
    let pos = match find_empty(model) {
        Some(value) => value,
        None => {
            return true;
        }
    };
    let limit = if pos.row == 0 { 8 } else { 64 };
    for i in 0..limit {
        if is_valid(model, i, &pos) {
            model.solution[pos.row][pos.cell] = i;
            if solve(model) {
                return true;
            }
            model.solution[pos.row][pos.cell] = -1;
        }
    }
    false
}

fn main() {
    let mut model = create_model();
    let ret = solve(&mut model);
    if !ret {
        println!("found no solutions");
        return;
    }

    println!("found a solution:");
    println!(
        "ubl: use cube {}, then rotate: {}",
        model.solution[0][0] + 1,
        rotation_to_string(model.solution[1][0])
    );
    println!(
        "ubr: use cube {}, then rotate: {}",
        model.solution[0][1] + 1,
        rotation_to_string(model.solution[1][1])
    );
    println!(
        "ufr: use cube {}, then rotate: {}",
        model.solution[0][2] + 1,
        rotation_to_string(model.solution[1][2])
    );
    println!(
        "ufl: use cube {}, then rotate: {}",
        model.solution[0][3] + 1,
        rotation_to_string(model.solution[1][3])
    );
    println!(
        "dfl: use cube {}, then rotate: {}",
        model.solution[0][4] + 1,
        rotation_to_string(model.solution[1][4])
    );
    println!(
        "dfr: use cube {}, then rotate: {}",
        model.solution[0][5] + 1,
        rotation_to_string(model.solution[1][5])
    );
    println!(
        "dbr: use cube {}, then rotate: {}",
        model.solution[0][6] + 1,
        rotation_to_string(model.solution[1][6])
    );
    println!(
        "dbl: use cube {}, then rotate: {}",
        model.solution[0][7] + 1,
        rotation_to_string(model.solution[1][7])
    );

    println!(
        "hint: U will have color {}",
        color_to_string(get_model_color(&model, SLOT_UBL, SIDE_U))
    );
    println!(
        "hint: D will have color {}",
        color_to_string(get_model_color(&model, SLOT_DFL, SIDE_D))
    );
    println!(
        "hint: R will have color {}",
        color_to_string(get_model_color(&model, SLOT_UBR, SIDE_R))
    );
    println!(
        "hint: L will have color {}",
        color_to_string(get_model_color(&model, SLOT_UBL, SIDE_L))
    );
    println!(
        "hint: F will have color {}",
        color_to_string(get_model_color(&model, SLOT_UFR, SIDE_F))
    );
    println!(
        "hint: B will have color {}",
        color_to_string(get_model_color(&model, SLOT_UBL, SIDE_B))
    );
}
