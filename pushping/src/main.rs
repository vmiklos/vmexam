/*
 * Copyright 2022 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to pushping.

use isahc::RequestExt as _;
use std::rc::Rc;

struct RealNetwork {}

impl pushping::Network for RealNetwork {
    fn post(&self, url: String, data: String) -> anyhow::Result<()> {
        isahc::Request::post(url).body(data)?.send()?;
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let ctx = pushping::Context::new(
        vfs::PhysicalFS::new("/").into(),
        Rc::new(RealNetwork {}),
    );
    let exit_code = pushping::run(args, &ctx)?;
    std::process::exit(exit_code);
}
