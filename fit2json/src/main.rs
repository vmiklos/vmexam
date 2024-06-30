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

fn extract_key_values_from_html(html: &str) -> anyhow::Result<Vec<String>> {
    let mut lines = html.split('\n');
    lines.next();
    lines.next_back();
    Ok(lines.map(|i| i.to_string()).collect())
}

fn extract_stats_html(args: &Arguments) -> anyhow::Result<String> {
    let tempfile = tempfile::Builder::new().suffix(".kml").tempfile()?;
    let kml_path = tempfile
        .path()
        .to_str()
        .context("to_str() failed")?
        .to_string();
    let gpsbabel_args = [
        "-i",
        "garmin_fit",
        "-f",
        &args.fit,
        "-o",
        // Metric units.
        "kml,units=m",
        "-F",
        &kml_path,
    ];
    checked_run("gpsbabel", &gpsbabel_args)?;
    let xml = std::fs::read_to_string(tempfile.path())?;
    let package = sxd_document::parser::parse(&xml)?;
    let document = package.as_document();
    let factory = sxd_xpath::Factory::new();
    let xpath = factory
        .build("/k:kml/k:Document/k:Folder/k:Folder/k:description/text()")
        .context("could not compile XPath")?;
    let xpath = xpath.context("No XPath was compiled")?;
    let mut context = sxd_xpath::Context::new();
    context.set_namespace("k", "http://www.opengis.net/kml/2.2");
    let value = xpath.evaluate(&context, document.root())?;
    let sxd_xpath::Value::Nodeset(nodeset) = value else {
        return Err(anyhow::anyhow!("XPath result is not a nodeset"));
    };
    let mut html: String = "".into();
    for node in nodeset.document_order() {
        let value = node.string_value();
        if value.starts_with('<') {
            html = value;
            break;
        }
    }
    Ok(html)
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
    let mut table: Vec<String> = Vec::new();

    // Try to inject the activity name.
    if let Ok(activity_name) = extract_activity_name(&args) {
        table.push(format!("<tr><td><b>Name</b> {activity_name}</td></tr>"));
    }

    // Try to inject other stats, provided by gpsbabel.
    let html = extract_stats_html(&args)?;
    let mut stats = extract_key_values_from_html(&html)?;
    table.append(&mut stats);

    // Write the potentially mutated JSON.
    let mut description: Vec<String> = Vec::new();
    description.push("<table>".into());
    for row in table {
        description.push(row);
    }
    description.push("</table>".into());
    let description = serde_json::Value::from(description.join(""));
    properties
        .as_object_mut()
        .unwrap()
        .insert("description".into(), description);
    serde_json::to_writer(std::fs::File::create(&json_path)?, &json)?;
    Ok(())
}
