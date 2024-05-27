/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Simple [CSP](https://en.wikipedia.org/wiki/Constraint_satisfaction_problem) solver sample.

use clap::Parser as _;

#[derive(clap::Parser)]
struct Arguments {
    #[arg(short, long)]
    sum: i32,
    a1: i32,
    a2: i32,
    b1: i32,
    b2: i32,
}

fn main() {
    let args = Arguments::parse();

    let mut problem = puzzle_solver::Puzzle::new();
    let a1 = problem.new_var_with_candidates(&[0, args.a1]);
    let a2 = problem.new_var_with_candidates(&[0, args.a2]);
    let b1 = problem.new_var_with_candidates(&[0, args.b1]);
    let b2 = problem.new_var_with_candidates(&[0, args.b2]);
    problem.equals(a1 + a2 + b1 + b2, args.sum);
    let solutions = problem.solve_all();
    for s in solutions {
        println!("{} {}\n{} {}", s[a1], s[a2], s[b1], s[b2]);
    }
}
