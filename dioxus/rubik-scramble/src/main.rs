/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to rubik-scramble.

use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

const TABLE: &[u8] = include_bytes!("../bin/table.bin");

fn make_scramble() -> String {
    // f2l-solved for now
    let table = kewb::fs::decode_table(TABLE).unwrap();
    let mut solver = kewb::Solver::new(&table, 25, None);
    let mut states = Vec::new();
    let state = kewb::generators::generate_state_f2l_solved();
    let scramble = kewb::scramble::scramble_from_state(state, &mut solver).unwrap();

    states.push(state);
    solver.clear();

    let stringified = scramble
        .iter()
        .map(|m| m.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    stringified
}

/// The root component.
pub fn app() -> Element {
    let scramble = use_signal(|| make_scramble());
    rsx! {
        div { "{scramble}" }
    }
}
