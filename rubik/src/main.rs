/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Cmdline tool, related to <https://en.wikipedia.org/wiki/Rubik%27s_Cube>.

use clap::Parser as _;
use rand::Rng as _;

/// Shuffles a solved cube, to help excecising. Picks 24 random steps to randomize the starting
/// state.
#[derive(clap::Args)]
struct Shuffle {}

#[derive(clap::Subcommand)]
enum Commands {
    Shuffle(Shuffle),
}

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn shuffle() -> anyhow::Result<()> {
    let mut prev_side = "".to_string();
    for step in 1..25 {
        let mut side;
        loop {
            // Randomly pick one side of the cube.
            side = match rand::thread_rng().gen_range(1..7) {
                1 => "s",
                2 => "t",
                3 => "k",
                4 => "n",
                5 => "f",
                6 => "l",
                _ => {
                    unreachable!();
                }
            }
            .to_string();
            if side != prev_side {
                break;
            }
            // Side would be the same as the previous, try again.
        }
        prev_side = side.to_string();
        // Randomly pick a direction.
        let direction = match rand::thread_rng().gen_range(1..4) {
            1 => "e",
            2 => "i",
            3 => "u",
            _ => {
                unreachable!();
            }
        };
        print!("{side}{direction} ");
        if step % 4 == 0 {
            print!(" ");
        }
        if step % 12 == 0 {
            println!();
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Shuffle(_) => shuffle(),
    }
}
