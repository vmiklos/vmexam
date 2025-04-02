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

fn print_slot(
    model: &csp::cube::Model,
    slot: csp::cube::Slot,
    slot_name: &str,
) -> anyhow::Result<()> {
    println!(
        "cube is {}, corner is {}, U is {}, F is {}",
        model.get_cube_index(slot),
        slot_name,
        model.get_color_string(slot, csp::cube::Side::U)?,
        model.get_color_string(slot, csp::cube::Side::F)?
    );

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    let problem = std::fs::read_to_string(args.problem_path)?;
    let mut model = csp::cube::Model::new(&problem);
    let ret = model.solve()?;
    if !ret {
        println!("found no solutions");
        return Ok(());
    }

    print_slot(&model, csp::cube::Slot::DFL, "dfl")?;
    print_slot(&model, csp::cube::Slot::DFR, "dfr")?;
    print_slot(&model, csp::cube::Slot::DBR, "dbr")?;
    print_slot(&model, csp::cube::Slot::DBL, "dbl")?;
    print_slot(&model, csp::cube::Slot::UBL, "ubl")?;
    print_slot(&model, csp::cube::Slot::UBR, "ubr")?;
    print_slot(&model, csp::cube::Slot::UFR, "ufr")?;
    print_slot(&model, csp::cube::Slot::UFL, "ufl")?;

    Ok(())
}
