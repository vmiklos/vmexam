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

    fn get_hostname(&self) -> anyhow::Result<String> {
        let hostname = gethostname::gethostname();
        let host = hostname.to_str().context("to_str() failed")?;
        Ok(host.to_string())
    }

    fn get_current_dir(&self) -> anyhow::Result<String> {
        let current_dir = std::env::current_dir()?;
        let dir = current_dir.to_str().context("to_str() failed")?;
        Ok(dir.to_string())
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
