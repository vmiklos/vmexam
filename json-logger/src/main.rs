/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to json-logger.

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let ctx = json_logger::Context::new(
        vfs::PhysicalFS::new("/").into(),
        std::rc::Rc::new(json_logger::PhysicalTime::new()),
    );
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    json_logger::run(args, &ctx, &mut stdin, &mut stdout)
}
