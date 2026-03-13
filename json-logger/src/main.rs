/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Logs the value of one key of the JSON passed in on stdin.

use clap::Parser as _;
use std::io::Write as _;

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

fn main() -> anyhow::Result<()> {
    // Decide what is the log path.
    let args = Args::parse();
    let cwd = std::env::current_dir()?;
    let cwd_str = cwd.to_string_lossy();
    let filename = cwd_str.replace("/", "-");
    let mut log_path = std::path::PathBuf::from(&args.log_dir);
    log_path.push(filename);

    // Open stdin as a stream.
    let stdin = std::io::stdin();
    let stream = serde_json::Deserializer::from_reader(stdin).into_iter::<serde_json::Value>();

    // Find out the timestamp prefix.
    let now = time::OffsetDateTime::now_local()?;
    let format = time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")?;
    let timestamp = now.format(&format)?;

    // Log the specified key from the JSON.
    for input_result in stream {
        let input = match input_result {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to parse JSON from stdin: {}", e);
                continue;
            }
        };

        if let Some(log_value) = input.get(&args.key) {
            if let Some(log_str) = log_value.as_str() {
                let mut file = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&log_path)?;

                writeln!(file, "[{}] {}", timestamp, log_str)?;
            } else {
                eprintln!("Value for key '{}' is not a string.", args.key);
            }
        } else {
            eprintln!("Key '{}' not found in input JSON.", args.key);
        }
    }

    if args.print_empty {
        std::io::stdout().write_all(b"{}\n")?;
    }

    Ok(())
}
