/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Sudoku solver, implementing the algorithm from
//! <https://medium.com/@ev.zafeiratos/sudoku-solver-with-python-a-methodical-approach-for-algorithm-optimization-part-1-b2c99887167f>.

use anyhow::Context as _;
use clap::Parser as _;

#[derive(clap::Parser)]
struct Arguments {
    /// Path to problem file: 9 numbers in 9 lines.
    problem_path: String,
}

const MODEL_WIDTH: usize = 9;
const MODEL_HEIGHT: usize = 9;
type ModelRow = [usize; MODEL_WIDTH];
type Model = [ModelRow; MODEL_HEIGHT];

fn create_model(problem: &str) -> anyhow::Result<Model> {
    let mut model: Model = [[0; MODEL_WIDTH]; MODEL_HEIGHT];
    let fro_rows = problem.split('\n');
    for (row_index, fro_row) in fro_rows.enumerate() {
        if row_index >= MODEL_HEIGHT {
            break;
        }
        let mut row: ModelRow = [0; MODEL_WIDTH];
        for (cell_index, cell) in fro_row.chars().enumerate() {
            let value: usize = cell.to_digit(10).context("to_digit() failed")?.try_into()?;
            row[cell_index] = value;
        }
        model[row_index] = row;
    }
    Ok(model)
}

fn find_empty(model: &Model) -> Option<(usize, usize)> {
    for (row_index, row) in model.iter().enumerate() {
        for (cell_index, cell) in row.iter().enumerate() {
            if *cell == 0 {
                return Some((row_index, cell_index));
            }
        }
    }
    None
}

fn is_valid(model: &Model, num: usize, pos: (usize, usize)) -> bool {
    let (row, cell) = pos;
    for cell in model[row] {
        if cell == num {
            return false;
        }
    }
    for row in model {
        if row[cell] == num {
            return false;
        }
    }
    let row_block_start = (row / 3) * 3;
    let cell_block_start = (cell / 3) * 3;
    for row in model.iter().skip(row_block_start).take(3) {
        for cell in row.iter().skip(cell_block_start).take(3) {
            if *cell == num {
                return false;
            }
        }
    }
    true
}

fn solve(model: &mut Model) -> bool {
    let empty = if let Some(value) = find_empty(model) {
        value
    } else {
        return true;
    };
    let (row, cell) = empty;
    for num in 1..10 {
        if is_valid(model, num, empty) {
            model[row][cell] = num;
            if solve(model) {
                return true;
            }
            model[row][cell] = 0;
        }
    }
    false
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    let problem = std::fs::read_to_string(args.problem_path)?;
    let mut model = create_model(&problem)?;
    assert!(solve(&mut model));
    for row in model {
        for cell in row {
            print!("{cell}");
        }
        println!();
    }

    Ok(())
}
