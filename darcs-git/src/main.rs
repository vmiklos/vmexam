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
use std::io::Read as _;

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
        let mut stdin = std::io::stdin();
        let fd = libc::STDIN_FILENO;
        let mut settings = termios::Termios::from_fd(fd)?;

        // Set raw mode.
        let old_settings = settings;
        settings.c_lflag &= !(termios::ICANON | libc::ECHO);
        termios::tcsetattr(fd, termios::TCSANOW, &settings)?;

        // Read a character.
        let mut buffer = [0; 1];
        stdin.read_exact(&mut buffer)?;

        // Restore old mode.
        termios::tcsetattr(fd, termios::TCSANOW, &old_settings)?;

        Ok(String::from_utf8(buffer.to_vec())?)
    }
}

fn main() -> anyhow::Result<()> {
    let ctx = StdContext {};
    darcs_git::main(&ctx)
}
