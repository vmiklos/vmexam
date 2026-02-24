/*
 * Copyright 2022 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to pushping.

use anyhow::Context as _;
use isahc::RequestExt as _;
use std::rc::Rc;

struct RealNetwork {}

impl pushping::Network for RealNetwork {
    fn post(&self, url: String, data: String) -> anyhow::Result<()> {
        isahc::Request::post(url).body(data)?.send()?;
        Ok(())
    }
}

struct RealProcess {}

impl pushping::Process for RealProcess {
    fn command_status(&self, command: &str, args: &[&str]) -> anyhow::Result<i32> {
        let exit_status = std::process::Command::new(command)
            .args(args)
            .status()
            .context("failed to execute the command as a child process")?;
        let exit_code = exit_status.code().context("code() failed")?;
        Ok(exit_code)
    }
}

struct RealTime {}

impl pushping::Time for RealTime {
    fn now(&self) -> time::OffsetDateTime {
        time::OffsetDateTime::now_local().expect("now_local() failed")
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let ctx = pushping::Context::new(
        vfs::PhysicalFS::new("/").into(),
        Rc::new(RealNetwork {}),
        Rc::new(RealProcess {}),
        Rc::new(RealTime {}),
    );
    let exit_code = pushping::run(args, &ctx)?;
    std::process::exit(exit_code);
}
