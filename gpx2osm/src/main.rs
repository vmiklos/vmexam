/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Converts OSMTracker GPX files to JOSM OSM files.

use anyhow::Context as _;
use clap::Parser as _;
use serde::Serialize as _;

/// A single waypoint from the GPX input.
#[derive(serde::Deserialize)]
struct Waypoint {
    #[serde(rename = "@lat")]
    lat: String,
    #[serde(rename = "@lon")]
    lon: String,
    name: String,
}

/// The root of the GPX input; only waypoints are of interest.
#[derive(serde::Deserialize)]
struct Gpx {
    #[serde(rename = "wpt", default)]
    waypoints: Vec<Waypoint>,
}

/// A key-value tag of an OSM node.
#[derive(serde::Serialize)]
struct Tag {
    #[serde(rename = "@k")]
    k: String,
    #[serde(rename = "@v")]
    v: String,
}

/// An OSM node in the output.
#[derive(serde::Serialize)]
struct Node {
    #[serde(rename = "@id")]
    id: i64,
    #[serde(rename = "@visible")]
    visible: bool,
    #[serde(rename = "@lat")]
    lat: String,
    #[serde(rename = "@lon")]
    lon: String,
    tag: Tag,
}

/// The root of the OSM output.
#[derive(serde::Serialize)]
#[serde(rename = "osm")]
struct Osm {
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "node")]
    nodes: Vec<Node>,
}

/// Converts OSMTracker GPX files to JOSM OSM files.
#[derive(clap::Parser)]
struct Arguments {
    /// Path of the input GPX file.
    input: std::path::PathBuf,

    /// Path of the output OSM file.
    output: std::path::PathBuf,
}

/// Turns the parsed GPX waypoints into an OSM document.
fn gpx2osm(gpx: &Gpx) -> anyhow::Result<String> {
    let nodes = gpx
        .waypoints
        .iter()
        .enumerate()
        .map(|(index, waypoint)| {
            // JOSM uses negative ids for not-yet-uploaded nodes.
            let id = -(i64::try_from(index)? + 1);
            Ok(Node {
                id,
                visible: true,
                lat: waypoint.lat.clone(),
                lon: waypoint.lon.clone(),
                tag: Tag {
                    k: "addr:housenumber".to_string(),
                    v: waypoint.name.clone(),
                },
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    let osm = Osm {
        version: "0.6".to_string(),
        nodes,
    };

    // The serde serializer does not emit a header, so write it here.
    let mut output = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    output.push('\n');
    let mut serializer = quick_xml::se::Serializer::new(&mut output);
    serializer.indent('\t', 1);
    osm.serialize(serializer)?;
    output.push('\n');
    Ok(output)
}

fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();

    let input = std::fs::read_to_string(&arguments.input).context("failed to read input file")?;
    let gpx: Gpx = quick_xml::de::from_str(&input).context("failed to parse GPX input")?;

    let output = gpx2osm(&gpx)?;
    std::fs::write(&arguments.output, output).context("failed to write output file")?;

    Ok(())
}
