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

use anyhow::Context as _;

/// Corners of a cube.
#[derive(Clone, Copy)]
pub enum Slot {
    /// Up-bottom-left corner.
    UBL = 0,
    /// Up-bottom-right corner.
    UBR = 1,
    /// Up-front-right corner.
    UFR = 2,
    /// Up-front-left corner.
    UFL = 3,
    /// Down-front-left corner.
    DFL = 4,
    /// Down-front-right corner.
    DFR = 5,
    /// Down-bottom-right corner.
    DBR = 6,
    /// Down-bottom-left corner.
    DBL = 7,
}
const SLOTS_COUNT: usize = 8;

impl TryFrom<usize> for Slot {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Slot::UBL),
            1 => Ok(Slot::UBR),
            2 => Ok(Slot::UFR),
            3 => Ok(Slot::UFL),
            4 => Ok(Slot::DFL),
            5 => Ok(Slot::DFR),
            6 => Ok(Slot::DBR),
            7 => Ok(Slot::DBL),
            _ => Err(anyhow::anyhow!("invalid slot")),
        }
    }
}

impl From<Slot> for usize {
    fn from(val: Slot) -> Self {
        val as usize
    }
}

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
const SIDES_COUNT: usize = 6;

const ROW_SLOTS: usize = 0;
const ROW_ROTATIONS: usize = 1;
const ROWS_COUNT: usize = 2;
const ROTATIONS_COUNT: usize = 24;

#[derive(Clone)]
struct Position {
    /// Row in the model
    row: usize,
    /// Column in a row
    cell: usize,
}

struct Constraint {
    model_corner: Slot,
    candidate_corner: Position,
    side: usize,
    candidate_color: usize,
}

impl Constraint {
    fn new(
        model_corner: Slot,
        candidate_corner: &Position,
        side: usize,
        candidate_color: usize,
    ) -> Self {
        Constraint {
            model_corner,
            candidate_corner: candidate_corner.clone(),
            side,
            candidate_color,
        }
    }
}

/// Contains the calculated `solution` for the problem specified by `colors`.
pub struct Model {
    /// Slots: 0 or 1..SLOTS_SIZE
    /// - order is: UBL, UBR, UFR, UFL, DFL, DFR, DBR, DBL
    /// - e.g. if slot 0 is 2: for UBL, use the 2nd cube
    ///
    /// Cube rotations: 0 or 1..ROTATIONS_COUNT
    /// - e.g. if rotation 0 is 3: UBL has been rotated according to row 3 in rotate_color()
    solution: [[usize; SLOTS_COUNT]; ROWS_COUNT],
    /// SLOTS_SIZE cubes (0th..7th cube), SIDES_COUNT sides: U D R L F B
    /// colors: 0..5 for blue..red
    /// - e.g. if 0.0 is RED, then the up of the 0th cube is red
    colors: [[usize; SIDES_COUNT]; SLOTS_COUNT],
    /// List of the SIDES_COUNT color names
    color_names: Vec<String>,
}

impl Model {
    /// Creates a model from an input string.
    pub fn new(problem: &str) -> Model {
        let mut colors: [[usize; SIDES_COUNT]; SLOTS_COUNT] = [[0; SIDES_COUNT]; SLOTS_COUNT];
        let mut color_names: Vec<String> = Vec::new();
        let lines = problem.split('\n');
        for (line_index, line) in lines.enumerate() {
            if line_index >= SLOTS_COUNT {
                break;
            }

            let mut row: [usize; SIDES_COUNT] = [0; SIDES_COUNT];
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
            solution: [[0; SLOTS_COUNT]; ROWS_COUNT],
            colors,
            color_names,
        }
    }

    /// Gets the name of a color index.
    pub fn get_color_string(&self, slot: Slot, side: usize) -> anyhow::Result<String> {
        let index = self.get_color_index(slot, side).context("no color")?;
        Ok(self.color_names[index].to_string())
    }

    /// Gets what cube index to use for a specific corner.
    pub fn get_cube_index(&self, slot: Slot) -> usize {
        let slot: usize = slot.into();
        self.solution[ROW_SLOTS][slot]
    }

    /// Solves a specified, but not calcualted model.
    pub fn solve(&mut self) -> anyhow::Result<bool> {
        let pos = match self.find_empty() {
            Some(value) => value,
            None => {
                return Ok(true);
            }
        };
        let limit = if pos.row == ROW_SLOTS {
            SLOTS_COUNT
        } else {
            ROTATIONS_COUNT
        };
        for i in 1..=limit {
            if self.is_valid(i, &pos)? {
                self.solution[pos.row][pos.cell] = i;
                if self.solve()? {
                    return Ok(true);
                }
                self.solution[pos.row][pos.cell] = 0;
            }
        }
        Ok(false)
    }

    /// Gets the color index of a given corner's given side.
    fn get_color_index(&self, slot: Slot, side: usize) -> Option<usize> {
        let slot: usize = slot.into();
        rotate_color(
            &self.colors[self.solution[ROW_SLOTS][slot] - 1],
            side,
            self.solution[ROW_ROTATIONS][slot],
        )
    }

    fn find_empty(&self) -> Option<Position> {
        for row in 0..ROWS_COUNT {
            for cell in 0..SLOTS_COUNT {
                if self.solution[row][cell] == 0 {
                    return Some(Position { row, cell });
                }
            }
        }

        None
    }

    fn get_candidate_color(&self, slot: usize, side: usize, num: usize) -> Option<usize> {
        rotate_color(&self.colors[self.solution[ROW_SLOTS][slot] - 1], side, num)
    }

    fn is_valid_color(&self, constraint: &Constraint) -> bool {
        if let Some(model_color) = self.get_color_index(constraint.model_corner, constraint.side) {
            let candidate_color = match self.get_candidate_color(
                constraint.candidate_corner.cell,
                constraint.side,
                constraint.candidate_color,
            ) {
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

    fn is_valid_slot(&self, num: usize) -> bool {
        for i in 0..SLOTS_COUNT {
            if self.solution[ROW_SLOTS][i] == num {
                return false;
            }
        }

        true
    }

    fn is_valid(&self, num: usize, pos: &Position) -> anyhow::Result<bool> {
        if pos.row == ROW_SLOTS {
            return Ok(self.is_valid_slot(num));
        }
        let mut constraints: Vec<Constraint> = Vec::new();
        match Slot::try_from(pos.cell)? {
            Slot::UBL => {
                // provides U, B & L
            }
            Slot::UBR => {
                constraints.push(Constraint::new(Slot::UBL, pos, SIDE_U, num));
                constraints.push(Constraint::new(Slot::UBL, pos, SIDE_B, num));
                // provides R
            }
            Slot::UFR => {
                constraints.push(Constraint::new(Slot::UBR, pos, SIDE_U, num));
                // provides F
                constraints.push(Constraint::new(Slot::UBR, pos, SIDE_R, num));
            }
            Slot::UFL => {
                constraints.push(Constraint::new(Slot::UBL, pos, SIDE_U, num));
                constraints.push(Constraint::new(Slot::UFR, pos, SIDE_F, num));
                constraints.push(Constraint::new(Slot::UFR, pos, SIDE_L, num));
            }
            Slot::DFL => {
                // provides D
                constraints.push(Constraint::new(Slot::UFR, pos, SIDE_F, num));
                constraints.push(Constraint::new(Slot::UBL, pos, SIDE_L, num));
            }
            Slot::DFR => {
                constraints.push(Constraint::new(Slot::DFL, pos, SIDE_D, num));
                constraints.push(Constraint::new(Slot::UFR, pos, SIDE_F, num));
                constraints.push(Constraint::new(Slot::UBR, pos, SIDE_R, num));
            }
            Slot::DBR => {
                constraints.push(Constraint::new(Slot::DFL, pos, SIDE_D, num));
                constraints.push(Constraint::new(Slot::UBL, pos, SIDE_B, num));
                constraints.push(Constraint::new(Slot::UBR, pos, SIDE_R, num));
            }
            Slot::DBL => {
                constraints.push(Constraint::new(Slot::DFL, pos, SIDE_D, num));
                constraints.push(Constraint::new(Slot::UBL, pos, SIDE_B, num));
                constraints.push(Constraint::new(Slot::UBL, pos, SIDE_L, num));
            }
        }
        for constraint in constraints {
            if !self.is_valid_color(&constraint) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

fn rotate_color(colors: &[usize; SIDES_COUNT], side: usize, rotation: usize) -> Option<usize> {
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
