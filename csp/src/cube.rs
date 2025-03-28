/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Problem: have 8 small cubes, all of their sides are painted using 6 colors. Goal: build a
//! single 2x2x2 cube from the small cubes so that all large sides have the same color.
//!
//! The first 5 sides are easy to do manually, the 6th side is tricky: this solver does all sides
//! for you.

/// Up-bottom-left corner.
pub const SLOT_UBL: i8 = 0;
/// Up-bottom-right corner.
pub const SLOT_UBR: i8 = 1;
/// Up-front-right corner.
pub const SLOT_UFR: i8 = 2;
/// Up-front-left corner.
pub const SLOT_UFL: i8 = 3;
/// Down-front-left corner.
pub const SLOT_DFL: i8 = 4;
/// Down-front-right corner.
pub const SLOT_DFR: i8 = 5;
/// Down-bottom-right corner.
pub const SLOT_DBR: i8 = 6;
/// Down-bottom-left corner.
pub const SLOT_DBL: i8 = 7;

/// Upper side.
pub const SIDE_U: i8 = 0;
/// Down side.
pub const SIDE_D: i8 = 1;
/// Right side.
pub const SIDE_R: i8 = 2;
/// Left side.
pub const SIDE_L: i8 = 3;
/// Front side.
pub const SIDE_F: i8 = 4;
/// Back side.
pub const SIDE_B: i8 = 5;

struct Position {
    /// Row in the model
    row: usize,
    /// Column in a row
    cell: usize,
}

/// Contains the calculated `solution` for the problem specified by `colors`.
pub struct Model {
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
    /// List of the 6 color names
    color_names: Vec<String>,
}

impl Model {
    /// Creates a model from an input string.
    pub fn new(problem: &str) -> Model {
        let mut colors: [[i8; 6]; 8] = [[0; 6]; 8];
        let mut color_names: Vec<String> = Vec::new();
        let lines = problem.split('\n');
        for (line_index, line) in lines.enumerate() {
            if line_index >= 8 {
                break;
            }

            let mut row: [i8; 6] = [0; 6];
            let tokens = line.split(',');
            for (index, color) in tokens.enumerate() {
                let color = color.to_string();
                let color_num = match color_names.iter().position(|i| i == &color) {
                    Some(value) => value,
                    None => {
                        color_names.push(color);
                        color_names.len() - 1
                    }
                };
                row[index] = color_num as i8;
            }
            colors[line_index] = row;
        }

        Model {
            solution: [
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
            ],
            colors,
            color_names,
        }
    }

    /// Gets the name of a color index.
    pub fn get_color_string(&self, slot: i8, side: i8) -> String {
        let color = self.get_color_index(slot, side);
        match color {
            Some(value) => self.color_names[value as usize].to_string(),
            None => "".to_string(),
        }
    }

    /// Gets what cube index to use for a specific corner.
    pub fn get_cube_index(&self, slot: i8) -> i8 {
        self.solution[0][slot as usize] + 1
    }

    /// Solves a specified, but not calcualted model.
    pub fn solve(&mut self) -> bool {
        let pos = match self.find_empty() {
            Some(value) => value,
            None => {
                return true;
            }
        };
        let limit = if pos.row == 0 { 8 } else { 64 };
        for i in 0..limit {
            if self.is_valid(i, &pos) {
                self.solution[pos.row][pos.cell] = i;
                if self.solve() {
                    return true;
                }
                self.solution[pos.row][pos.cell] = -1;
            }
        }
        false
    }

    /// Gets the color index of a given corner's given side.
    fn get_color_index(&self, slot: i8, side: i8) -> Option<i8> {
        rotate_color(
            &self.colors[self.solution[0][slot as usize] as usize],
            side,
            self.solution[1][slot as usize],
        )
    }

    fn find_empty(&self) -> Option<Position> {
        for row in 0..2 {
            for cell in 0..8 {
                if self.solution[row][cell] == -1 {
                    return Some(Position { row, cell });
                }
            }
        }

        None
    }

    fn get_candidate_color(&self, slot: i8, side: i8, num: i8) -> Option<i8> {
        rotate_color(
            &self.colors[self.solution[0][slot as usize] as usize],
            side,
            num,
        )
    }

    fn is_valid_color(
        &self,
        model_slot: i8,
        candidate_pos: &Position,
        side: i8,
        candidate_rotation: i8,
    ) -> bool {
        let candidate_slot = candidate_pos.cell as i8;
        if let Some(model_color) = self.get_color_index(model_slot, side) {
            let candidate_color =
                match self.get_candidate_color(candidate_slot, side, candidate_rotation) {
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

    fn is_valid(&self, num: i8, pos: &Position) -> bool {
        if pos.row == 0 {
            for i in 0..8 {
                if self.solution[0][i] == num {
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
                    if !self.is_valid_color(SLOT_UBL, pos, SIDE_U, num) {
                        return false;
                    }
                    if !self.is_valid_color(SLOT_UBL, pos, SIDE_B, num) {
                        return false;
                    }
                    // provides R
                }
                SLOT_UFR => {
                    if !self.is_valid_color(SLOT_UBR, pos, SIDE_U, num) {
                        return false;
                    }
                    // provides F
                    if !self.is_valid_color(SLOT_UBR, pos, SIDE_R, num) {
                        return false;
                    }
                }
                SLOT_UFL => {
                    if !self.is_valid_color(SLOT_UBL, pos, SIDE_U, num) {
                        return false;
                    }
                    if !self.is_valid_color(SLOT_UFR, pos, SIDE_F, num) {
                        return false;
                    }
                    if !self.is_valid_color(SLOT_UBL, pos, SIDE_L, num) {
                        return false;
                    }
                }
                SLOT_DFL => {
                    // provides D
                    if !self.is_valid_color(SLOT_UFR, pos, SIDE_F, num) {
                        return false;
                    }
                    if !self.is_valid_color(SLOT_UBL, pos, SIDE_L, num) {
                        return false;
                    }
                }
                SLOT_DFR => {
                    if !self.is_valid_color(SLOT_DFL, pos, SIDE_D, num) {
                        return false;
                    }
                    if !self.is_valid_color(SLOT_UFR, pos, SIDE_F, num) {
                        return false;
                    }
                    if !self.is_valid_color(SLOT_UBR, pos, SIDE_R, num) {
                        return false;
                    }
                }
                SLOT_DBR => {
                    if !self.is_valid_color(SLOT_DFL, pos, SIDE_D, num) {
                        return false;
                    }
                    if !self.is_valid_color(SLOT_UBL, pos, SIDE_B, num) {
                        return false;
                    }
                    if !self.is_valid_color(SLOT_UBR, pos, SIDE_R, num) {
                        return false;
                    }
                }
                SLOT_DBL => {
                    if !self.is_valid_color(SLOT_DFL, pos, SIDE_D, num) {
                        return false;
                    }
                    if !self.is_valid_color(SLOT_UBL, pos, SIDE_B, num) {
                        return false;
                    }
                    if !self.is_valid_color(SLOT_UBL, pos, SIDE_L, num) {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }
        true
    }
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
