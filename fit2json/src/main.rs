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
use base64::Engine as _;
use clap::Parser as _;
use rand::RngCore as _;

#[derive(clap::Parser)]
struct Arguments {
    /// The input FIT file.
    fit: String,

    /// The output JSON file, a random urlsafe name is generated if omitted.
    json: Option<String>,
}

#[derive(serde::Deserialize)]
struct Activity {
    name: String,
    distance: f64,
    total_elevation_gain: f64,
    moving_time: f64,
    average_speed: f64,
    max_speed: f64,
    elapsed_time: f64,
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

fn create_json(args: &Arguments, json_path: &str) -> anyhow::Result<()> {
    let gpsbabel_args = [
        "-i",
        "garmin_fit",
        "-f",
        &args.fit,
        "-o",
        "geojson",
        "-F",
        json_path,
    ];
    checked_run("gpsbabel", &gpsbabel_args)
}

fn format_duration(num_seconds: i64) -> String {
    let seconds = num_seconds % 60;
    let num_minutes = num_seconds / 60;
    let minutes = num_minutes % 60;
    let hours = num_minutes / 60;
    format!("{hours}:{minutes:0>2}:{seconds:0>2}")
}

/// Returns stats in general and name in particular.
fn extract_stats(args: &Arguments) -> anyhow::Result<(Vec<(String, String)>, String)> {
    let mut stats: Vec<(String, String)> = Vec::new();
    let mut meta_json = std::path::PathBuf::from(&args.fit);
    meta_json.set_extension("meta.json");
    let file = std::fs::File::open(meta_json)?;
    let activity: Activity = serde_json::from_reader(&file)?;
    stats.push(("Name".into(), activity.name.to_string()));
    stats.push((
        "Distance".into(),
        format!("{:.2} km", activity.distance / 1000_f64),
    ));
    stats.push((
        "Elevation gain".into(),
        format!("{} m", activity.total_elevation_gain),
    ));
    stats.push((
        "Average speed".into(),
        format!("{:.2} km/h", activity.average_speed * 3.6_f64),
    ));
    stats.push((
        "Moving time".into(),
        format_duration(activity.moving_time as i64),
    ));
    stats.push((
        "Max speed".into(),
        format!("{:.2} km/h", activity.max_speed * 3.6_f64),
    ));
    stats.push((
        "Elapsed time".into(),
        format_duration(activity.elapsed_time as i64),
    ));
    Ok((stats, activity.name.to_string()))
}

fn read_json(json_path: &str) -> anyhow::Result<serde_json::Value> {
    let file = std::fs::File::open(json_path)?;
    Ok(serde_json::from_reader(file)?)
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    let json_path = match args.json {
        Some(ref value) => value.to_string(),
        None => {
            let mut bytes = [0; 17];
            rand::thread_rng().fill_bytes(&mut bytes);
            let b64 = base64::prelude::BASE64_STANDARD.encode(bytes);
            let mut urlsafe: String = b64
                .chars()
                .map(|i| match i {
                    '+' => '-',
                    '/' => '_',
                    _ => i,
                })
                .collect();
            if let Some(value) = urlsafe.strip_suffix('=') {
                urlsafe = value.to_string();
            }
            format!("{urlsafe}.json")
        }
    };

    // Convert to JSON.
    create_json(&args, &json_path)?;

    // Read the JSON to potentially mutate it.
    let mut json = read_json(&json_path)?;
    let features = json.as_object_mut().unwrap().get_mut("features").unwrap();
    let feature = &mut features.as_array_mut().unwrap()[0];
    let properties = feature
        .as_object_mut()
        .unwrap()
        .get_mut("properties")
        .unwrap();
    let mut table: Vec<(String, String)> = Vec::new();

    // Try to inject the activity name & stats.
    if let Ok((mut stats, name)) = extract_stats(&args) {
        table.append(&mut stats);

        let name = serde_json::Value::from(name);
        properties
            .as_object_mut()
            .unwrap()
            .insert("name".into(), name);
    }

    // Write the potentially mutated JSON.
    let mut description: Vec<String> = Vec::new();
    description.push("<table>".into());
    for row in table {
        description.push(format!("<tr><td><b>{}</b> {}</td></tr>", row.0, row.1));
    }
    description.push("</table>".into());
    let description = serde_json::Value::from(description.join(""));
    properties
        .as_object_mut()
        .unwrap()
        .insert("description".into(), description);
    serde_json::to_writer(std::fs::File::create(&json_path)?, &json)?;
    if args.json.is_none() {
        println!("{}", json_path);
    }
    Ok(())
}
