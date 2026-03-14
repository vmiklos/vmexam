/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Logs the value of one key of the JSON passed in on stdin.

use anyhow::Context as _;
use clap::Parser as _;
use std::io::Write as _;
use std::rc::Rc;

/// Abstracts away the current time.
pub trait Time {
    /// Returns the current local time.
    fn now(&self) -> time::OffsetDateTime;
}

/// Abstracts away the physical filesystem.
pub struct Context {
    /// File system.
    pub fs: vfs::VfsPath,
    /// Time.
    pub time: Rc<dyn Time>,
}

impl Context {
    /// Creates a new Context.
    pub fn new(fs: vfs::VfsPath, time: Rc<dyn Time>) -> Self {
        Context { fs, time }
    }
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The name of the key in the input JSON whose string value should be logged.
    #[arg(short, long)]
    key: String,

    /// The directory where log files will be stored.
    #[arg(short, long)]
    log_dir: String,

    /// Print '{}' on stdout if the logging is done without errors.
    #[arg(short, long)]
    print_empty: bool,
}

/// Main logic of json-logger.
pub fn run(
    args: Vec<String>,
    ctx: &Context,
    stdin: &mut dyn std::io::Read,
    stdout: &mut dyn std::io::Write,
) -> anyhow::Result<()> {
    // Decide what is the log path.
    let args = Args::parse_from(args);
    let cwd = std::env::current_dir()?;
    let cwd_str = cwd.to_string_lossy();
    let filename = cwd_str.replace("/", "-");

    // Open stdin as a stream.
    let stream = serde_json::Deserializer::from_reader(stdin).into_iter::<serde_json::Value>();

    // Find out the timestamp prefix.
    let now = ctx.time.now();
    let format = time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")?;
    let timestamp = now.format(&format)?;

    // Log the specified key from the JSON.
    for item in stream {
        let parsed = item?;

        let log_value = parsed
            .get(&args.key)
            .context("key not found in input JSON")?;
        let log_str = log_value
            .as_str()
            .context("value for the key is not a string")?;
        let log_path = ctx.fs.join(&args.log_dir)?.join(&filename)?;
        if !log_path.exists()? {
            log_path.create_file()?;
        }
        let mut file = log_path.append_file()?;

        writeln!(file, "[{}] {}", timestamp, log_str)?;
    }

    if args.print_empty {
        stdout.write_all(b"{}\n")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests;
