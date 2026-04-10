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
use isahc::RequestExt as _;
use std::collections::HashMap;
use std::rc::Rc;

/// Command-line arguments.
#[derive(clap::Parser)]
pub struct Args {
    /// Be quiet.
    #[arg(short, long)]
    pub quiet: bool,
}

/// Real network implementation, using isahc.
struct RealNetwork {}

impl strava_mirror::Network for RealNetwork {
    fn get(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> anyhow::Result<isahc::Response<isahc::Body>> {
        let mut request = isahc::Request::get(url);
        for (key, value) in headers {
            request = request.header(key, value);
        }
        let response = request.body(())?.send()?;
        Ok(response)
    }

    fn post(&self, url: &str, body: &str) -> anyhow::Result<isahc::Response<isahc::Body>> {
        let response = isahc::post(url, body)?;
        Ok(response)
    }
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
    let network = Rc::new(RealNetwork {});
    let ctx = strava_mirror::Context { fs, network };

    strava_mirror::run(std::env::args().collect(), &ctx)
}
