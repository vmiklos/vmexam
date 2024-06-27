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

#[derive(serde::Deserialize)]
struct Activity {
    name: String,
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

fn create_json(args: &Arguments) -> anyhow::Result<()> {
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
    println!("Running: gpsbabel {}", gpsbabel_args.join(" "));
    checked_run("gpsbabel", &gpsbabel_args)
}

fn extract_activity_name(args: &Arguments) -> anyhow::Result<String> {
    let mut meta_json = std::path::PathBuf::from(&args.fit);
    meta_json.set_extension("meta.json");
    let file = std::fs::File::open(meta_json)?;
    let activity: Activity = serde_json::from_reader(&file)?;
    Ok(activity.name)
}

fn read_json(args: &Arguments) -> anyhow::Result<serde_json::Value> {
    let file = std::fs::File::open(&args.json)?;
    Ok(serde_json::from_reader(file)?)
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    // Convert to JSON.
    create_json(&args)?;

    // Try to exract activity name.
    let activity_name = match extract_activity_name(&args) {
        Ok(value) => value,
        Err(_) => "".to_string(),
    };

    if activity_name.is_empty() {
        return Ok(());
    }

    // Mutate the JSON to show the activity name.
    println!("Injecting description into the JSON");
    let mut json = read_json(&args)?;
    let features = json.as_object_mut().unwrap().get_mut("features").unwrap();
    let feature = &mut features.as_array_mut().unwrap()[0];
    let properties = feature
        .as_object_mut()
        .unwrap()
        .get_mut("properties")
        .unwrap();
    let description = serde_json::Value::from(format!("<b>Name</b> {activity_name}"));
    properties
        .as_object_mut()
        .unwrap()
        .insert("description".into(), description);
    serde_json::to_writer(std::fs::File::create(&args.json)?, &json)?;
    Ok(())
}
