/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to strava_mirror.

use anyhow::Context as _;
use clap::Parser as _;

/// Command-line arguments.
#[derive(clap::Parser)]
pub struct Args {
    /// Be quiet.
    #[arg(short, long)]
    pub quiet: bool,
}

/// Sets up logging so it has local time timestamp as a prefix.
fn setup_logging(level: log::LevelFilter) -> anyhow::Result<()> {
    let mut builder = simplelog::ConfigBuilder::new();
    builder.set_time_format_custom(simplelog::format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second]"
    ));
    if builder.set_time_offset_to_local().is_err() {
        return Err(anyhow::anyhow!("offset to local failed"));
    }
    let config = builder.build();
    simplelog::CombinedLogger::init(vec![simplelog::TermLogger::new(
        level,
        config,
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Never,
    )])?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let log_level = if args.quiet {
        log::LevelFilter::Error
    } else {
        log::LevelFilter::Info
    };
    setup_logging(log_level)?;

    let home = home::home_dir().context("home_dir() failed")?;
    let fs: vfs::VfsPath = vfs::PhysicalFS::new(home).into();
    let ctx = strava_mirror::Context { fs };

    strava_mirror::run(Vec::new(), &ctx)
}
