/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Given a 2x2 table with numbers, select cells, so that the sum of the selected cells will equal to
//! the provided sum.

//! (Real-life math exercise for 2nd grade students in primary school.)

fn main() {
    let mut problem = puzzle_solver::Puzzle::new();
    let a1 = problem.new_var_with_candidates(&[0, 5]);
    let a2 = problem.new_var_with_candidates(&[0, 4]);
    let b1 = problem.new_var_with_candidates(&[0, 8]);
    let b2 = problem.new_var_with_candidates(&[0, 7]);
    problem.equals(a1 + a2 + b1 + b2, 16);
    let solutions = problem.solve_all();
    for s in solutions {
        println!("{} {}\n{} {}", s[a1], s[a2], s[b1], s[b2]);
    }
}
