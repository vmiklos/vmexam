/*
 * Copyright 2019 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
*/

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Takes an OSM way ID and turns it into a string that is readable and
//! e.g. OsmAnd can parse it as well.

use anyhow::Context as _;
use std::io::Write;
use std::sync::Arc;

/// Allows HTTP GET and POST requests.
pub trait Network: Send + Sync {
    /// If `data` is empty, that means HTTP GET, otherwise a HTTP POST.
    fn urlopen(&self, url: &str, data: &str) -> anyhow::Result<String>;

    /// Is stdout a tty?
    fn isatty(&self) -> bool;
}

pub use system::StdNetwork;

fn query_turbo(urllib: &dyn Network, query: &str) -> anyhow::Result<crate::serde::TurboResult> {
    let url = "http://overpass-api.de/api/interpreter";

    let buf = urllib.urlopen(url, query)?;

    let elements: crate::serde::TurboResult = match serde_json::from_str(&buf) {
        Ok(value) => value,
        Err(error) => {
            return Err(anyhow::anyhow!(
                "failed to parse JSON from overpass: {}",
                error
            ));
        }
    };

    Ok(elements)
}

fn query_nominatim(
    urllib: &dyn Network,
    query: &str,
) -> anyhow::Result<Vec<crate::serde::NominatimResult>> {
    let prefix = "http://nominatim.openstreetmap.org/search.php?";
    let encoded: String = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("q", query)
        .append_pair("format", "json")
        .finish();
    let url = format!("{}{}", prefix, encoded);

    let buf = urllib.urlopen(url.as_str(), "")?;

    let elements: Vec<crate::serde::NominatimResult> = match serde_json::from_str(&buf) {
        Ok(value) => value,
        Err(error) => {
            return Err(anyhow::anyhow!(
                "failed to parse JSON from nominatim: {}",
                error
            ));
        }
    };

    Ok(elements)
}

fn osmify(query: &str, urllib: &dyn Network) -> anyhow::Result<String> {
    let mut elements = query_nominatim(urllib, query)?;
    if elements.is_empty() {
        return Err(anyhow::anyhow!("no results from nominatim"));
    }

    if elements.len() > 1 {
        // There are multiple elements, prefer buildings if possible.
        let buildings: Vec<crate::serde::NominatimResult> = elements
            .iter()
            .filter(|i| i.class == "building")
            .cloned()
            .collect();

        if !buildings.is_empty() {
            elements = buildings;
        }
    }

    let element = &elements[0];
    let lat = &element.lat;
    let lon = &element.lon;
    let object_type = &element.osm_type;
    let object_id = element.osm_id;

    // Use overpass to get the properties of the object.
    let overpass_query = format!(
        r#"[out:json];
(
    {}({});
);
out body;"#,
        object_type, object_id
    );
    let turbo_result = query_turbo(urllib, &overpass_query)?;
    let elements = turbo_result.elements;
    if elements.is_empty() {
        return Err(anyhow::anyhow!("no results from overpass"));
    }

    let element = &elements[0];
    let tags = &element.tags;
    let city = &tags.city;
    let housenumber = &tags.housenumber;
    let postcode = &tags.postcode;
    let street = &tags.street;
    let addr = format!("{} {}, {} {}", postcode, city, street, housenumber);

    // Print the result.
    Ok(format!("{},{} ({})", lat, lon, addr))
}

fn spinner(
    rx: &std::sync::mpsc::Receiver<anyhow::Result<String>>,
    stream: &mut dyn Write,
    urllib: &Arc<dyn Network>,
) -> anyhow::Result<()> {
    let spin_characters = ['\\', '|', '/', '-'];
    let mut spin_index = 0;
    loop {
        match rx.try_recv() {
            Ok(result) => {
                if urllib.isatty() {
                    print!("\r");
                }
                std::io::stdout().flush()?;
                let result = result?;
                writeln!(stream, "{}", result)?;
                return Ok(());
            }
            Err(_) => {
                if urllib.isatty() {
                    print!("\r [{}] ", spin_characters[spin_index]);
                }
                std::io::stdout().flush()?;
                spin_index = (spin_index + 1) % spin_characters.len();
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}

fn worker(query: &str, urllib: &dyn Network, tx: &std::sync::mpsc::Sender<anyhow::Result<String>>) {
    let result = osmify(query, urllib);
    tx.send(result.context("failed to osmify")).unwrap()
}

/// Inner main() that is allowed to fail.
pub fn our_main(
    args: Vec<String>,
    stream: &mut dyn Write,
    urllib: &Arc<dyn Network>,
) -> anyhow::Result<()> {
    if args.len() > 1 {
        let (tx, rx) = std::sync::mpsc::channel();
        {
            let urllib = urllib.clone();
            std::thread::spawn(move || worker(&args[1], &*urllib, &tx));
        }
        spinner(&rx, stream, urllib)?;
    } else {
        writeln!(stream, "usage: addr-osmify <query>")?;
        writeln!(stream)?;
        writeln!(stream, "e.g. addr-osmify 'Mészáros utca 58/a, Budapest'")?;
    }

    Ok(())
}

/// Similar to plain main(), but with an interface that allows testing.
pub fn main(args: Vec<String>, stream: &mut dyn Write, urllib: &Arc<dyn Network>) -> i32 {
    match our_main(args, stream, urllib) {
        Ok(_) => 0,
        Err(err) => {
            stream.write_all(format!("{:?}\n", err).as_bytes()).unwrap();
            1
        }
    }
}

mod serde;
/// Real (not test) trait implementations.
pub mod system;
#[cfg(test)]
mod tests;
