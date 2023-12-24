/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

//! Commandline interface to ics2txt.

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! An ICS printer for mutt with detailed time info.

fn main() {
    let args: Vec<String> = std::env::args().collect();
    std::process::exit(ics2txt::main(args, &mut std::io::stdout()))
}
