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

fn get_slot(
    model: &csp::cube::Model,
    slot: csp::cube::Slot,
    slot_name: &str,
) -> anyhow::Result<Vec<String>> {
    Ok([
        model.get_cube_index(slot).to_string(),
        slot_name.to_string(),
        model.get_color_string(slot, csp::cube::Side::U)?,
        model.get_color_string(slot, csp::cube::Side::F)?,
    ]
    .to_vec())
}

fn print_markdown(table: &[Vec<String>]) {
    // Calc column widths.
    let mut col_widths: Vec<usize> = Vec::new();
    for col_index in 0..table[0].len() {
        let mut width = 0;
        for row in table {
            if row[col_index].len() > width {
                width = row[col_index].len();
            }
        }
        col_widths.push(width);
    }

    // Print with header bottom line & padding.
    for (row_index, row) in table.iter().enumerate() {
        for (cell_index, cell) in row.iter().enumerate() {
            print!("{:width$}", cell, width = col_widths[cell_index]);
            if cell_index < col_widths.len() - 1 {
                print!(" ");
            }
        }
        println!();
        if row_index == 0 {
            for (cell_index, _) in row.iter().enumerate() {
                print!("{:-<width$}", "", width = col_widths[cell_index]);
                if cell_index < col_widths.len() - 1 {
                    print!(" ");
                }
            }
            println!();
        }
    }
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

    let mut table: Vec<Vec<String>> = vec![vec![
        "Cube".to_string(),
        "Corner".to_string(),
        "U".to_string(),
        "F".to_string(),
    ]];
    table.push(get_slot(&model, csp::cube::Slot::DFL, "DFL")?);
    table.push(get_slot(&model, csp::cube::Slot::DFR, "DFR")?);
    table.push(get_slot(&model, csp::cube::Slot::DBR, "DBR")?);
    table.push(get_slot(&model, csp::cube::Slot::DBL, "DBL")?);
    table.push(get_slot(&model, csp::cube::Slot::UBL, "UBL")?);
    table.push(get_slot(&model, csp::cube::Slot::UBR, "UBR")?);
    table.push(get_slot(&model, csp::cube::Slot::UFR, "UFR")?);
    table.push(get_slot(&model, csp::cube::Slot::UFL, "UFL")?);

    print_markdown(&table);

    Ok(())
}
