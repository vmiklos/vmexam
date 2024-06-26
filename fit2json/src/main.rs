/*
 * Copyright 2024 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Wrapper around gpsbabel to produce geojson from Strava fit files.

use anyhow::Context as _;
use clap::Parser as _;

#[derive(clap::Parser)]
struct Arguments {
    /// The input FIT file.
    fit: String,

    /// The output JSON file.
    json: String,
}

fn checked_run(first: &str, rest: &[&str]) -> anyhow::Result<()> {
    let exit_status = std::process::Command::new(first).args(rest).status()?;
    match exit_status.code().context("code() failed")? {
        0 => Ok(()),
        code => Err(anyhow::anyhow!(
            "failed to execute {first} {rest:?}: exit status is {code}"
        )),
    }
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    let gpsbabel_args = [
        "-i",
        "garmin_fit",
        "-f",
        &args.fit,
        "-o",
        "geojson",
        "-F",
        &args.json,
    ];
    println!("gpsbabel {}", gpsbabel_args.join(" "));
    checked_run("gpsbabel", &gpsbabel_args)?;

    Ok(())
}
