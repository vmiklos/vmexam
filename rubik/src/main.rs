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

/// Names the colors of the cube facelets: up, right, face, down, left, back.
#[rustfmt::skip]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Color {
    U, R, F, D, L, B,
}

impl TryFrom<char> for Color {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Color::U),
            'R' => Ok(Color::R),
            'F' => Ok(Color::F),
            'D' => Ok(Color::D),
            'L' => Ok(Color::L),
            'B' => Ok(Color::B),
            _ => Err("Invalid color value".to_owned()),
        }
    }
}

/// The names of the facelet positions of the cube.
///
/// ```text
///             |************|
///             |*U1**U2**U3*|
///             |************|
///             |*U4**U5**U6*|
///             |************|
///             |*U7**U8**U9*|
///             |************|
/// ************|************|************|************|
/// *L1**L2**L3*|*F1**F2**F3*|*R1**R2**F3*|*B1**B2**B3*|
/// ************|************|************|************|
/// *L4**L5**L6*|*F4**F5**F6*|*R4**R5**R6*|*B4**B5**B6*|
/// ************|************|************|************|
/// *L7**L8**L9*|*F7**F8**F9*|*R7**R8**R9*|*B7**B8**B9*|
/// ************|************|************|************|
///             |************|
///             |*D1**D2**D3*|
///             |************|
///             |*D4**D5**D6*|
///             |************|
///             |*D7**D8**D9*|
///             |************|
/// ```
#[rustfmt::skip]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Facelet {
    U1, U2, U3, U4, _U5, U6, U7, U8, U9,
    R1, R2, R3, R4, _R5, R6, R7, R8, R9,
    F1, F2, F3, F4, _F5, F6, F7, F8, F9,
    D1, D2, D3, D4, _D5, D6, D7, D8, D9,
    L1, L2, L3, L4, _L5, L6, L7, L8, L9,
    B1, B2, B3, B4, _B5, B6, B7, B8, B9,
}

/// Map the corner positions to facelet positions.
const CORNER_FACELET: [[Facelet; 3]; 8] = [
    /*UBL=*/ [Facelet::U1, Facelet::L1, Facelet::B3],
    /*UBR=*/ [Facelet::U3, Facelet::B1, Facelet::R3],
    /*UFR=*/ [Facelet::U9, Facelet::R1, Facelet::F3],
    /*UFL=*/ [Facelet::U7, Facelet::F1, Facelet::L3],
    /*DFL=*/ [Facelet::D1, Facelet::L9, Facelet::F7],
    /*DFR=*/ [Facelet::D3, Facelet::F9, Facelet::R7],
    /*DBR=*/ [Facelet::D9, Facelet::R9, Facelet::B7],
    /*DBL=*/ [Facelet::D7, Facelet::B9, Facelet::L7],
];

/// Map the edge positions to facelet positions.
const EDGE_FACELET: [[Facelet; 2]; 12] = [
    /*BL=*/ [Facelet::B6, Facelet::L4],
    /*BR=*/ [Facelet::B4, Facelet::R6],
    /*FR=*/ [Facelet::F6, Facelet::R4],
    /*FL=*/ [Facelet::F4, Facelet::L6],
    /*UB=*/ [Facelet::U2, Facelet::B2],
    /*UR=*/ [Facelet::U6, Facelet::R2],
    /*UF=*/ [Facelet::U8, Facelet::F2],
    /*UL=*/ [Facelet::U4, Facelet::L2],
    /*DF=*/ [Facelet::D2, Facelet::F8],
    /*DR=*/ [Facelet::D6, Facelet::R8],
    /*DB=*/ [Facelet::D8, Facelet::B8],
    /*DL=*/ [Facelet::D4, Facelet::L8],
];

/// Map the corner positions to facelet colors.
const CORNER_COLOR: [[Color; 3]; 8] = [
    /*UBL=*/ [Color::U, Color::L, Color::B],
    /*UBR=*/ [Color::U, Color::B, Color::R],
    /*UFR=*/ [Color::U, Color::R, Color::F],
    /*UFL=*/ [Color::U, Color::F, Color::L],
    /*DFL=*/ [Color::D, Color::L, Color::F],
    /*DFR=*/ [Color::D, Color::F, Color::R],
    /*DBR=*/ [Color::D, Color::R, Color::B],
    /*DBL=*/ [Color::D, Color::B, Color::L],
];

/// Map the edge positions to facelet colors.
const EDGE_COLOR: [[Color; 2]; 12] = [
    /*BL=*/ [Color::B, Color::L],
    /*BR=*/ [Color::B, Color::R],
    /*FR=*/ [Color::F, Color::R],
    /*FL=*/ [Color::F, Color::L],
    /*UB=*/ [Color::U, Color::B],
    /*UR=*/ [Color::U, Color::R],
    /*UF=*/ [Color::U, Color::F],
    /*UL=*/ [Color::U, Color::L],
    /*DF=*/ [Color::D, Color::F],
    /*DR=*/ [Color::D, Color::R],
    /*DB=*/ [Color::D, Color::B],
    /*DL=*/ [Color::D, Color::L],
];

/// Cube on the facelet level.
struct FaceCube {
    f: [Color; 54],
}

#[rustfmt::skip]
const SOLVED_FACE_CUBE: FaceCube = FaceCube {
    f: [
        Color::U, Color::U, Color::U, Color::U, Color::U, Color::U, Color::U, Color::U, Color::U,
        Color::R, Color::R, Color::R, Color::R, Color::R, Color::R, Color::R, Color::R, Color::R,
        Color::F, Color::F, Color::F, Color::F, Color::F, Color::F, Color::F, Color::F, Color::F,
        Color::D, Color::D, Color::D, Color::D, Color::D, Color::D, Color::D, Color::D, Color::D,
        Color::L, Color::L, Color::L, Color::L, Color::L, Color::L, Color::L, Color::L, Color::L,
        Color::B, Color::B, Color::B, Color::B, Color::B, Color::B, Color::B, Color::B, Color::B,
    ],
};

impl TryFrom<&str> for FaceCube {
    type Error = String;
    fn try_from(cube_string: &str) -> Result<Self, Self::Error> {
        let mut f: [Color; 54] = SOLVED_FACE_CUBE.f;

        for (i, c) in cube_string.chars().enumerate() {
            f[i] = Color::try_from(c).unwrap();
        }

        Ok(FaceCube { f })
    }
}

impl TryFrom<&FaceCube> for cube::state::State {
    type Error = String;
    fn try_from(face_cube: &FaceCube) -> Result<Self, Self::Error> {
        let mut ori: usize = 0;
        let mut state = cube::state::SOLVED_STATE;
        let mut col1;
        let mut col2;
        for i in 0..8 {
            let i = cube::state::Corner::try_from(i).unwrap();
            // get the colors of the cubie at corner i, starting with U/D
            for index in 0..3 {
                ori = index;
                if face_cube.f[CORNER_FACELET[i as usize][ori] as usize] == Color::U
                    || face_cube.f[CORNER_FACELET[i as usize][ori] as usize] == Color::D
                {
                    break;
                }
            }
            col1 = face_cube.f[CORNER_FACELET[i as usize][(ori + 1) % 3] as usize];
            col2 = face_cube.f[CORNER_FACELET[i as usize][(ori + 2) % 3] as usize];
            for j in 0..8 {
                let j = cube::state::Corner::try_from(j).unwrap();
                if col1 == CORNER_COLOR[j as usize][1] && col2 == CORNER_COLOR[j as usize][2] {
                    // in cornerposition i we have cornercubie j
                    state.cp[i as usize] = j;
                    state.co[i as usize] = ori as u8 % 3;
                    break;
                }
            }
        }
        for i in 0..12 {
            let i = cube::state::Edge::try_from(i).unwrap();
            for j in 0..12 {
                let j = cube::state::Edge::try_from(j).unwrap();
                if face_cube.f[EDGE_FACELET[i as usize][0] as usize] == EDGE_COLOR[j as usize][0]
                    && face_cube.f[EDGE_FACELET[i as usize][1] as usize]
                        == EDGE_COLOR[j as usize][1]
                {
                    state.ep[i as usize] = j;
                    state.eo[i as usize] = 0;
                    break;
                }
                if face_cube.f[EDGE_FACELET[i as usize][0] as usize] == EDGE_COLOR[j as usize][1]
                    && face_cube.f[EDGE_FACELET[i as usize][1] as usize]
                        == EDGE_COLOR[j as usize][0]
                {
                    state.ep[i as usize] = j;
                    state.eo[i as usize] = 1;
                    break;
                }
            }
        }
        Ok(state)
    }
}

/// Shuffles a solved cube, to help excecising. Picks 24 random steps to randomize the starting
/// state.
#[derive(clap::Args)]
struct Shuffle {}

/// Solves a state of the cube.
#[derive(clap::Args)]
struct Solve {
    #[arg(short, long)]
    faces: String,
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
    // Example shuffle:
    // F L' B R' U R U B' L2 R' F2 U2 L' F2 D F U R' D R U' L' R2 D2
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

fn solve(args: &Solve) -> anyhow::Result<()> {
    let face_cube = FaceCube::try_from(args.faces.as_str()).unwrap();
    let state = cube::state::State::try_from(&face_cube).unwrap();
    let (move_table, pruning_table) = two_phase::fs::read_table()?;
    let max: u8 = 23;
    let timeout: Option<f32> = None;
    let mut solver = two_phase::Solver::new(&move_table, &pruning_table, max, timeout);
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
