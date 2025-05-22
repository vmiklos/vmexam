/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to darcs-git.

use anyhow::Context as _;
use std::io::BufRead as _;

/// Context implementation, backed by library calls.
struct StdContext {}

impl darcs_git::Context for StdContext {
    fn command_status(&self, command: &str, args: &[&str]) -> anyhow::Result<i32> {
        let exit_status = std::process::Command::new(command).args(args).status()?;
        exit_status.code().context("code() failed")
    }

    fn command_output(&self, command: &str, args: &[&str]) -> anyhow::Result<String> {
        let output = std::process::Command::new(command).args(args).output()?;
        String::from_utf8(output.stdout).context("from_utf8() failed")
    }

    fn env_args(&self) -> Vec<String> {
        std::env::args().collect()
    }

    fn print(&self, string: &str) {
        print!("{string}");
    }

    fn readln(&self) -> anyhow::Result<String> {
        let stdin = std::io::stdin();
        let line = stdin.lock().lines().next().context("no first line")?;
        Ok(line?)
    }

    fn readch(&self) -> anyhow::Result<String> {
        let term = console::Term::stdout();
        Ok(term.read_char()?.to_string())
    }
}

fn main() -> anyhow::Result<()> {
    let ctx = StdContext {};
    darcs_git::main(&ctx)
}
