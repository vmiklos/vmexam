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
pub const SLOT_UBL: usize = 0;
/// Up-bottom-right corner.
pub const SLOT_UBR: usize = 1;
/// Up-front-right corner.
pub const SLOT_UFR: usize = 2;
/// Up-front-left corner.
pub const SLOT_UFL: usize = 3;
/// Down-front-left corner.
pub const SLOT_DFL: usize = 4;
/// Down-front-right corner.
pub const SLOT_DFR: usize = 5;
/// Down-bottom-right corner.
pub const SLOT_DBR: usize = 6;
/// Down-bottom-left corner.
pub const SLOT_DBL: usize = 7;

/// Upper side.
pub const SIDE_U: usize = 0;
/// Down side.
pub const SIDE_D: usize = 1;
/// Right side.
pub const SIDE_R: usize = 2;
/// Left side.
pub const SIDE_L: usize = 3;
/// Front side.
pub const SIDE_F: usize = 4;
/// Back side.
pub const SIDE_B: usize = 5;

struct Position {
    /// Row in the model
    row: usize,
    /// Column in a row
    cell: usize,
}

/// Contains the calculated `solution` for the problem specified by `colors`.
pub struct Model {
    /// Slots: 0 or 1..8
    /// - order is: UBL, UBR, UFR, UFL, DFL, DFR, DBR, DBL
    /// - e.g. if slot 0 is 2: for UBL, use the 2nd cube
    ///
    /// Cube rotations: 0 or 1..24
    /// - e.g. if rotation 0 is 3: UBL has been rotated according to row 3 in rotate_color()
    solution: [[usize; 8]; 2],
    /// 8 cubes (0th..7th cube), 6 sides: U D R L F B
    /// colors: 0..5 for blue..red
    /// - e.g. if 0.0 is RED, then the up of the 0th cube is red
    colors: [[usize; 6]; 8],
    /// List of the 6 color names
    color_names: Vec<String>,
}

impl Model {
    /// Creates a model from an input string.
    pub fn new(problem: &str) -> Model {
        let mut colors: [[usize; 6]; 8] = [[0; 6]; 8];
        let mut color_names: Vec<String> = Vec::new();
        let lines = problem.split('\n');
        for (line_index, line) in lines.enumerate() {
            if line_index >= 8 {
                break;
            }

            let mut row: [usize; 6] = [0; 6];
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
                row[index] = color_num;
            }
            colors[line_index] = row;
        }

        Model {
            solution: [[0; 8]; 2],
            colors,
            color_names,
        }
    }

    /// Gets the name of a color index.
    pub fn get_color_string(&self, slot: usize, side: usize) -> String {
        let color = self.get_color_index(slot, side);
        match color {
            Some(value) => self.color_names[value].to_string(),
            None => "".to_string(),
        }
    }

    /// Gets what cube index to use for a specific corner.
    pub fn get_cube_index(&self, slot: usize) -> usize {
        self.solution[0][slot]
    }

    /// Solves a specified, but not calcualted model.
    pub fn solve(&mut self) -> bool {
        let pos = match self.find_empty() {
            Some(value) => value,
            None => {
                return true;
            }
        };
        let limit = if pos.row == 0 { 8 } else { 24 };
        for i in 1..=limit {
            if self.is_valid(i, &pos) {
                self.solution[pos.row][pos.cell] = i;
                if self.solve() {
                    return true;
                }
                self.solution[pos.row][pos.cell] = 0;
            }
        }
        false
    }

    /// Gets the color index of a given corner's given side.
    fn get_color_index(&self, slot: usize, side: usize) -> Option<usize> {
        rotate_color(
            &self.colors[self.solution[0][slot] - 1],
            side,
            self.solution[1][slot],
        )
    }

    fn find_empty(&self) -> Option<Position> {
        for row in 0..2 {
            for cell in 0..8 {
                if self.solution[row][cell] == 0 {
                    return Some(Position { row, cell });
                }
            }
        }

        None
    }

    fn get_candidate_color(&self, slot: usize, side: usize, num: usize) -> Option<usize> {
        rotate_color(&self.colors[self.solution[0][slot] - 1], side, num)
    }

    fn is_valid_color(
        &self,
        model_slot: usize,
        candidate_pos: &Position,
        side: usize,
        candidate_rotation: usize,
    ) -> bool {
        let candidate_slot = candidate_pos.cell;
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

    fn is_valid(&self, num: usize, pos: &Position) -> bool {
        if pos.row == 0 {
            for i in 0..8 {
                if self.solution[0][i] == num {
                    return false;
                }
            }
        } else {
            match pos.cell {
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

fn rotate_color(colors: &[usize; 6], side: usize, rotation: usize) -> Option<usize> {
    if rotation == 0 {
        return None;
    }
    // First is no rotation, the the rest of the 24 combinations.
    let rotations = [
        [SIDE_U, SIDE_D, SIDE_R, SIDE_L, SIDE_F, SIDE_B],
        [SIDE_U, SIDE_D, SIDE_B, SIDE_F, SIDE_R, SIDE_L],
        [SIDE_U, SIDE_D, SIDE_L, SIDE_R, SIDE_B, SIDE_F],
        [SIDE_U, SIDE_D, SIDE_F, SIDE_B, SIDE_L, SIDE_R],
        [SIDE_D, SIDE_U, SIDE_L, SIDE_R, SIDE_F, SIDE_B],
        [SIDE_D, SIDE_U, SIDE_B, SIDE_F, SIDE_L, SIDE_R],
        [SIDE_D, SIDE_U, SIDE_R, SIDE_L, SIDE_B, SIDE_F],
        [SIDE_D, SIDE_U, SIDE_F, SIDE_B, SIDE_R, SIDE_L],
        [SIDE_R, SIDE_L, SIDE_D, SIDE_U, SIDE_F, SIDE_B],
        [SIDE_R, SIDE_L, SIDE_B, SIDE_F, SIDE_D, SIDE_U],
        [SIDE_R, SIDE_L, SIDE_U, SIDE_D, SIDE_B, SIDE_F],
        [SIDE_R, SIDE_L, SIDE_F, SIDE_B, SIDE_U, SIDE_D],
        [SIDE_L, SIDE_R, SIDE_U, SIDE_D, SIDE_F, SIDE_B],
        [SIDE_L, SIDE_R, SIDE_B, SIDE_F, SIDE_U, SIDE_D],
        [SIDE_L, SIDE_R, SIDE_D, SIDE_U, SIDE_B, SIDE_F],
        [SIDE_L, SIDE_R, SIDE_F, SIDE_B, SIDE_D, SIDE_U],
        [SIDE_F, SIDE_B, SIDE_R, SIDE_L, SIDE_D, SIDE_U],
        [SIDE_F, SIDE_B, SIDE_U, SIDE_D, SIDE_R, SIDE_L],
        [SIDE_F, SIDE_B, SIDE_L, SIDE_R, SIDE_U, SIDE_D],
        [SIDE_F, SIDE_B, SIDE_D, SIDE_U, SIDE_L, SIDE_R],
        [SIDE_B, SIDE_F, SIDE_R, SIDE_L, SIDE_U, SIDE_D],
        [SIDE_B, SIDE_F, SIDE_D, SIDE_U, SIDE_R, SIDE_L],
        [SIDE_B, SIDE_F, SIDE_L, SIDE_R, SIDE_D, SIDE_U],
        [SIDE_B, SIDE_F, SIDE_U, SIDE_D, SIDE_L, SIDE_R],
    ];
    let sides = rotations[rotation - 1];
    Some(colors[sides[side]])
}
