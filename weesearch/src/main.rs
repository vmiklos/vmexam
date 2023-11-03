/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

//! Commandline interface to weesearch.

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

fn main() {
    let args: Vec<String> = std::env::args().collect();
    std::process::exit(weesearch::main(args, &mut std::io::stdout()))
}
