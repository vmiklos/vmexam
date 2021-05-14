/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

extern crate atty;
extern crate reqwest;
extern crate serde_json;
extern crate url;

use std::io::Write;

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct OsmifyError {
    details: String,
}

impl std::fmt::Display for OsmifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for OsmifyError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub trait Urllib: Send + Sync {
    fn urlopen(&self, url: &str, data: &str) -> BoxResult<String>;
}

fn query_turbo(urllib: &dyn Urllib, query: &str) -> BoxResult<String> {
    let url = "http://overpass-api.de/api/interpreter";

    let buf = urllib.urlopen(url, query)?;
    Ok(buf)
}

fn query_nominatim(urllib: &dyn Urllib, query: &str) -> BoxResult<String> {
    let prefix = "http://nominatim.openstreetmap.org/search.php?";
    let encoded: String = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("q", query)
        .append_pair("format", "json")
        .finish();
    let url = format!("{}{}", prefix, encoded);

    let buf = urllib.urlopen(url.as_str(), "")?;

    Ok(buf)
}

fn osmify(query: &str, urllib: &dyn Urllib) -> BoxResult<String> {
    let nominatim = query_nominatim(urllib, query)?;
    let json: serde_json::Value = match serde_json::from_str(&nominatim) {
        Ok(value) => value,
        Err(error) => {
            return Err(Box::new(OsmifyError {
                details: format!("Failed to parse JSON from nominatim: {:?}", error),
            }));
        }
    };
    let mut elements = json.as_array().ok_or("option::NoneError")?.clone();
    if elements.is_empty() {
        return Err(Box::new(OsmifyError {
            details: "No results from nominatim".to_string(),
        }));
    }

    if elements.len() > 1 {
        // There are multiple elements, prefer buildings if possible.
        let buildings: Vec<serde_json::Value> = elements
            .iter()
            .filter(|i| {
                let i = match i.as_object() {
                    Some(value) => value,
                    None => return false,
                };
                let class = match i.get("class") {
                    Some(value) => value.as_str(),
                    None => return false,
                };
                match class {
                    Some(value) => value == "building",
                    None => false,
                }
            })
            .cloned()
            .collect();

        if !buildings.is_empty() {
            elements = buildings;
        }
    }

    let element = elements[0].as_object().ok_or("option::NoneError")?;
    let lat = element["lat"]
        .as_str()
        .ok_or("no lat in json from nominatim")?;
    let lon = element["lon"]
        .as_str()
        .ok_or("no lon in json from nominatim")?;
    let object_type = element["osm_type"]
        .as_str()
        .ok_or("no osm_type in json from nominatim")?;
    let object_id = element["osm_id"]
        .as_u64()
        .ok_or("no osm_id in json from nominatim")?;

    // Use overpass to get the properties of the object.
    let overpass_query = format!(
        r#"[out:json];
(
    {}({});
);
out body;"#,
        object_type, object_id
    );
    let turbo = query_turbo(urllib, &overpass_query)?;
    let json: serde_json::Value = match serde_json::from_str(&turbo) {
        Ok(value) => value,
        Err(error) => {
            return Err(Box::new(OsmifyError {
                details: format!("Failed to parse JSON from overpass: {:?}", error),
            }));
        }
    };
    let json = json.as_object().ok_or("option::NoneError")?;
    let elements = &json["elements"].as_array().ok_or("option::NoneError")?;
    if elements.is_empty() {
        return Err(Box::new(OsmifyError {
            details: "No results from overpass".to_string(),
        }));
    }

    let element = &elements[0];
    let tags = element["tags"].as_object().ok_or("option::NoneError")?;
    let city = tags["addr:city"].as_str().ok_or("option::NoneError")?;
    let housenumber = tags["addr:housenumber"]
        .as_str()
        .ok_or("option::NoneError")?;
    let postcode = tags["addr:postcode"].as_str().ok_or("option::NoneError")?;
    let street = tags["addr:street"].as_str().ok_or("option::NoneError")?;
    let addr = format!("{} {}, {} {}", postcode, city, street, housenumber);

    // Print the result.
    Ok(format!("geo:{},{} ({})", lat, lon, addr))
}

fn spinner(
    rx: &std::sync::mpsc::Receiver<Result<String, String>>,
    stream: &mut dyn Write,
) -> BoxResult<()> {
    let spin_characters = vec!['\\', '|', '/', '-'];
    let mut spin_index = 0;
    loop {
        match rx.try_recv() {
            Ok(result) => {
                if atty::is(atty::Stream::Stdout) {
                    print!("\r");
                }
                std::io::stdout().flush()?;
                let result = result?;
                writeln!(stream, "{}", result)?;
                return Ok(());
            }
            Err(_) => {
                if atty::is(atty::Stream::Stdout) {
                    print!("\r [{}] ", spin_characters[spin_index]);
                }
                std::io::stdout().flush()?;
                spin_index = (spin_index + 1) % spin_characters.len();
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}

pub fn main(args: Vec<String>, stream: &mut dyn Write, urllib: Box<dyn Urllib>) -> BoxResult<()> {
    if args.len() > 1 {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let result = osmify(&args[1], &*urllib);
            match result {
                Ok(value) => tx.send(Ok(value)),
                Err(error) => tx.send(Err(format!("Failed to osmify: {:?}", error))),
            }
        });
        spinner(&rx, stream)?;
    } else {
        writeln!(stream, "usage: addr-osmify <query>")?;
        writeln!(stream)?;
        writeln!(stream, "e.g. addr-osmify 'Mészáros utca 58/a, Budapest'")?;
    }

    Ok(())
}
