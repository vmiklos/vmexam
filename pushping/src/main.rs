/*
 * Copyright 2022 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to pushping.

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let ctx = pushping::Context::new(vfs::PhysicalFS::new("/").into());
    let exit_code = pushping::run(args, &ctx)?;
    std::process::exit(exit_code);
}
