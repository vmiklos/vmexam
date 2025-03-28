/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Provides the 'cube' cmdline tool.

use clap::Parser as _;

#[derive(clap::Parser)]
struct Arguments {
    /// Path to problem file: 9 numbers in 9 lines.
    problem_path: String,
}

fn print_slot(model: &csp::cube::Model, slot: i8, slot_name: &str) {
    println!(
        "{}: use cube {}, then U is {}, F is {}",
        slot_name,
        model.get_cube_index(slot),
        model.get_color_string(slot, csp::cube::SIDE_U),
        model.get_color_string(slot, csp::cube::SIDE_F)
    );
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    let problem = std::fs::read_to_string(args.problem_path)?;
    let mut model = csp::cube::Model::new(&problem);
    let ret = model.solve();
    if !ret {
        println!("found no solutions");
        return Ok(());
    }

    println!("found a solution:");
    print_slot(&model, csp::cube::SLOT_DFL, "dfl");
    print_slot(&model, csp::cube::SLOT_DFR, "dfr");
    print_slot(&model, csp::cube::SLOT_DBR, "dbr");
    print_slot(&model, csp::cube::SLOT_DBL, "dbl");
    print_slot(&model, csp::cube::SLOT_UBL, "ubl");
    print_slot(&model, csp::cube::SLOT_UBR, "ubr");
    print_slot(&model, csp::cube::SLOT_UFR, "ufr");
    print_slot(&model, csp::cube::SLOT_UFL, "ufl");

    Ok(())
}
