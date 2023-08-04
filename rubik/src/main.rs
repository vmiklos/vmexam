/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Cmdline tool, related to <https://en.wikipedia.org/wiki/Rubik%27s_Cube>.

use anyhow::Context as _;
use clap::Parser as _;
use rand::Rng as _;
use std::io::BufRead as _;
use std::io::Write as _;

/// Shuffles a solved cube, to help excecising. Picks 24 random steps to randomize the starting
/// state.
#[derive(clap::Args)]
struct Shuffle {}

/// Solves a state of the cube.
#[derive(clap::Args)]
struct Solve {
    #[arg(short, long)]
    colors: Option<String>,
}

#[derive(clap::Subcommand)]
enum Commands {
    Shuffle(Shuffle),
    Solve(Solve),
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
                1 => "F",
                2 => "B",
                3 => "R",
                4 => "L",
                5 => "U",
                6 => "D",
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
            1 => " ",
            2 => "'",
            3 => "2",
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

fn flushed_print(question: &str) -> anyhow::Result<()> {
    print!("{question} ");
    Ok(std::io::stdout().flush()?)
}

fn ask_string(question: &str) -> anyhow::Result<String> {
    flushed_print(question)?;
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().context("no first line")?;
    Ok(line?)
}

fn colors_to_faces(colors: &Option<String>) -> anyhow::Result<String> {
    let colors = match colors {
        Some(value) => value.to_string(),
        None => {
            let lines = vec![
                ask_string("blue  :")?,
                ask_string("yellow:")?,
                ask_string("red   :")?,
                ask_string("green :")?,
                ask_string("white :")?,
                ask_string("orange:")?,
            ];
            lines.join("")
        }
    };
    let mut faces: Vec<u8> = Vec::new();
    for color in colors.chars() {
        let face = match color {
            'B' | 'b' => 'U',
            'Y' | 'y' => 'R',
            'R' | 'r' => 'F',
            'G' | 'g' => 'D',
            'W' | 'w' => 'L',
            'O' | 'o' => 'B',
            _ => {
                return Err(anyhow::anyhow!("invalid color: {}", color));
            }
        } as u8;
        faces.push(face);
    }

    Ok(String::from_utf8(faces)?)
}

fn solve(args: &Solve) -> anyhow::Result<()> {
    let faces = colors_to_faces(&args.colors)?;
    let face_cube = kewb::FaceCube::try_from(faces.as_str()).unwrap();
    let state = kewb::State::try_from(&face_cube).unwrap();
    let (move_table, pruning_table) = kewb::fs::read_table()?;
    let max: u8 = 23;
    let timeout: Option<f32> = None;
    let mut solver = kewb::Solver::new(&move_table, &pruning_table, max, timeout);
    let solution = solver.solve(state).context("no solution")?;
    println!("{solution}");
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Shuffle(_) => shuffle(),
        Commands::Solve(args) => solve(args),
    }
}
