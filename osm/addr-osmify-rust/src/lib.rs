/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
*/

#![deny(warnings)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

//! Takes an OSM way ID and turns it into a string that is readable and
//! e.g. OsmAnd can parse it as well.

use std::io::Write;
use std::sync::Arc;

/// A Result which allows any error that implements Error.
pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct OsmifyError {
    details: String,
}

impl std::fmt::Display for OsmifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for OsmifyError {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Allows HTTP GET and POST requests.
pub trait Urllib: Send + Sync {
    /// If `data` is empty, that means HTTP GET, otherwise a HTTP POST.
    fn urlopen(&self, url: &str, data: &str) -> BoxResult<String>;
}

/// TurboTags contains various tags about one Overpass element.
#[derive(serde::Deserialize)]
struct TurboTags {
    #[serde(rename(deserialize = "addr:city"))]
    city: String,
    #[serde(rename(deserialize = "addr:housenumber"))]
    housenumber: String,
    #[serde(rename(deserialize = "addr:postcode"))]
    postcode: String,
    #[serde(rename(deserialize = "addr:street"))]
    street: String,
}

/// TurboElement represents one result from Overpass.
#[derive(serde::Deserialize)]
struct TurboElement {
    tags: TurboTags,
}

/// TurboResult is the result from Overpass.
#[derive(serde::Deserialize)]
struct TurboResult {
    elements: Vec<TurboElement>,
}

fn query_turbo(urllib: &dyn Urllib, query: &str) -> BoxResult<TurboResult> {
    let url = "http://overpass-api.de/api/interpreter";

    let buf = urllib.urlopen(url, query)?;

    let elements: TurboResult = match serde_json::from_str(&buf) {
        Ok(value) => value,
        Err(error) => {
            return Err(Box::new(OsmifyError {
                details: format!("failed to parse JSON from overpass: {}", error.to_string()),
            }));
        }
    };

    Ok(elements)
}

/// NominatimResult represents one element in the result array from Nominatim.
#[derive(Clone, serde::Deserialize)]
struct NominatimResult {
    class: String,
    lat: String,
    lon: String,
    osm_type: String,
    osm_id: u64,
}

fn query_nominatim(urllib: &dyn Urllib, query: &str) -> BoxResult<Vec<NominatimResult>> {
    let prefix = "http://nominatim.openstreetmap.org/search.php?";
    let encoded: String = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("q", query)
        .append_pair("format", "json")
        .finish();
    let url = format!("{}{}", prefix, encoded);

    let buf = urllib.urlopen(url.as_str(), "")?;

    let elements: Vec<NominatimResult> = match serde_json::from_str(&buf) {
        Ok(value) => value,
        Err(error) => {
            return Err(Box::new(OsmifyError {
                details: format!("failed to parse JSON from nominatim: {}", error.to_string()),
            }));
        }
    };

    Ok(elements)
}

fn osmify(query: &str, urllib: &dyn Urllib) -> BoxResult<String> {
    let mut elements = query_nominatim(urllib, query)?;
    if elements.is_empty() {
        return Err(Box::new(OsmifyError {
            details: "no results from nominatim".to_string(),
        }));
    }

    if elements.len() > 1 {
        // There are multiple elements, prefer buildings if possible.
        let buildings: Vec<NominatimResult> = elements
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
        return Err(Box::new(OsmifyError {
            details: "no results from overpass".to_string(),
        }));
    }

    let element = &elements[0];
    let tags = &element.tags;
    let city = &tags.city;
    let housenumber = &tags.housenumber;
    let postcode = &tags.postcode;
    let street = &tags.street;
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

/// Similar to plain main(), but with an interface that allows testing.
pub fn main(args: Vec<String>, stream: &mut dyn Write, urllib: &Arc<dyn Urllib>) -> BoxResult<()> {
    if args.len() > 1 {
        let (tx, rx) = std::sync::mpsc::channel();
        let urllib = urllib.clone();
        std::thread::spawn(move || {
            let result = osmify(&args[1], &*urllib);
            match result {
                Ok(value) => tx.send(Ok(value)),
                Err(error) => tx.send(Err(format!("failed to osmify: {}", error.to_string()))),
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

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    struct URLRoute {
        url: String,
        data_path: String,
        result_path: String,
    }

    struct MockUrllib {
        routes: Vec<URLRoute>,
    }

    impl Urllib for MockUrllib {
        fn urlopen(&self, url: &str, data: &str) -> BoxResult<String> {
            for route in self.routes.iter() {
                if route.url != url {
                    continue;
                }

                if !route.data_path.is_empty() {
                    // Can't use assert_eq!() here, failure would result in a hang.
                    let contents = match std::fs::read_to_string(&route.data_path) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(Box::new(OsmifyError {
                                details: format!(
                                    "failed read {}: {}",
                                    route.data_path,
                                    error.to_string()
                                ),
                            }))
                        }
                    };
                    if data != contents {
                        return Err(Box::new(OsmifyError {
                            details: format!("unexpected data: '{}' != '{}'", data, contents),
                        }));
                    }
                }

                let contents = match std::fs::read_to_string(&route.result_path) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(Box::new(OsmifyError {
                            details: format!(
                                "failed read {}: {}",
                                route.result_path,
                                error.to_string()
                            ),
                        }))
                    }
                };
                return Ok(contents);
            }

            return Err(Box::new(OsmifyError {
                details: format!("unexpected url: {}", url),
            }));
        }
    }

    #[test]
    fn test_happy() -> BoxResult<()> {
        let args: Vec<String> = vec!["".to_string(), "Mészáros utca 58/a, Budapest".to_string()];

        let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

        let routes = vec![
            URLRoute {
                url: "http://nominatim.openstreetmap.org/search.php?q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest&format=json".to_string(),
                data_path: "".to_string(),
                result_path: "mock/nominatim-happy.json".to_string()
            },
            URLRoute {
                url: "http://overpass-api.de/api/interpreter".to_string(),
                data_path: "mock/overpass-happy.expected-data".to_string(),
                result_path: "mock/overpass-happy.json".to_string()
            }
        ];
        let urllib: Arc<dyn Urllib> = Arc::new(MockUrllib { routes });
        main(args, &mut buf, &urllib)?;

        let buf_vec = buf.into_inner();
        let buf_string = match std::str::from_utf8(&buf_vec) {
            Ok(v) => v,
            Err(e) => panic!("invalid UTF-8 sequence: {}", e),
        };
        assert_eq!(
            buf_string,
            "geo:47.490592,19.030662 (1016 Budapest, Mészáros utca 58/a)\n"
        );

        Ok(())
    }

    #[test]
    fn test_nominatim_json() -> BoxResult<()> {
        let args: Vec<String> = vec!["".to_string(), "Mészáros utca 58/a, Budapest".to_string()];

        let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

        let routes = vec![
            URLRoute {
                url: "http://nominatim.openstreetmap.org/search.php?q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest&format=json".to_string(),
                data_path: "".to_string(),
                result_path: "mock/nominatim-bad.json".to_string()
            },
        ];
        let urllib: Arc<dyn Urllib> = Arc::new(MockUrllib { routes });
        let error = match main(args, &mut buf, &urllib) {
            Ok(_) => panic!("unexpected success"),
            Err(e) => e,
        };

        assert_eq!(
            error.to_string(),
            "failed to osmify: failed to parse JSON from nominatim: EOF while parsing an object at line 2 column 0",
        );

        Ok(())
    }

    #[test]
    fn test_nominatim_no_result() -> BoxResult<()> {
        let args: Vec<String> = vec!["".to_string(), "Mészáros utca 58/a, Budapestt".to_string()];

        let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

        let routes = vec![
            URLRoute {
                url: "http://nominatim.openstreetmap.org/search.php?q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapestt&format=json".to_string(),
                data_path: "".to_string(),
                result_path: "mock/nominatim-no-result.json".to_string()
            },
        ];
        let urllib: Arc<dyn Urllib> = Arc::new(MockUrllib { routes });
        let error = match main(args, &mut buf, &urllib) {
            Ok(_) => panic!("unexpected success"),
            Err(e) => e,
        };

        assert_eq!(
            error.to_string(),
            "failed to osmify: no results from nominatim",
        );

        Ok(())
    }

    #[test]
    fn test_prefer_buildings() -> BoxResult<()> {
        let args: Vec<String> = vec![
            "".to_string(),
            "Karinthy Frigyes út 18, Budapest".to_string(),
        ];

        let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

        let routes = vec![
            URLRoute {
                url: "http://nominatim.openstreetmap.org/search.php?q=Karinthy+Frigyes+%C3%BAt+18%2C+Budapest&format=json".to_string(),
                data_path: "".to_string(),
                result_path: "mock/nominatim-prefer-buildings.json".to_string()
            },
            URLRoute {
                url: "http://overpass-api.de/api/interpreter".to_string(),
                data_path: "mock/overpass-prefer-buildings.expected-data".to_string(),
                result_path: "mock/overpass-prefer-buildings.json".to_string()
            }
        ];
        let urllib: Arc<dyn Urllib> = Arc::new(MockUrllib { routes });
        main(args, &mut buf, &urllib)?;

        let buf_vec = buf.into_inner();
        let buf_string = match std::str::from_utf8(&buf_vec) {
            Ok(v) => v,
            Err(e) => panic!("invalid UTF-8 sequence: {}", e),
        };
        assert_eq!(
            buf_string,
            "geo:47.47690895,19.0512550758533 (1111 Budapest, Karinthy Frigyes út 18)\n"
        );

        Ok(())
    }

    #[test]
    fn test_overpass_json() -> BoxResult<()> {
        let args: Vec<String> = vec!["".to_string(), "Mészáros utca 58/a, Budapest".to_string()];

        let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

        let routes = vec![
            URLRoute {
                url: "http://nominatim.openstreetmap.org/search.php?q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest&format=json".to_string(),
                data_path: "".to_string(),
                result_path: "mock/nominatim-happy.json".to_string()
            },
            URLRoute {
                url: "http://overpass-api.de/api/interpreter".to_string(),
                data_path: "mock/overpass-happy.expected-data".to_string(),
                result_path: "mock/overpass-bad.json".to_string()
            }
        ];
        let urllib: Arc<dyn Urllib> = Arc::new(MockUrllib { routes });
        let error = match main(args, &mut buf, &urllib) {
            Ok(_) => panic!("unexpected success"),
            Err(e) => e,
        };

        assert_eq!(
            error.to_string(),
            "failed to osmify: failed to parse JSON from overpass: EOF while parsing a value at line 3 column 0",
        );

        Ok(())
    }

    #[test]
    fn test_overpass_noresult() -> BoxResult<()> {
        let args: Vec<String> = vec!["".to_string(), "Mészáros utca 58/a, Budapest".to_string()];

        let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

        let routes = vec![
            URLRoute {
                url: "http://nominatim.openstreetmap.org/search.php?q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest&format=json".to_string(),
                data_path: "".to_string(),
                result_path: "mock/nominatim-happy.json".to_string()
            },
            URLRoute {
                url: "http://overpass-api.de/api/interpreter".to_string(),
                data_path: "mock/overpass-happy.expected-data".to_string(),
                result_path: "mock/overpass-noresult.json".to_string()
            }
        ];
        let urllib: Arc<dyn Urllib> = Arc::new(MockUrllib { routes });
        let error = match main(args, &mut buf, &urllib) {
            Ok(_) => panic!("unexpected success"),
            Err(e) => e,
        };

        assert_eq!(
            error.to_string(),
            "failed to osmify: no results from overpass",
        );

        Ok(())
    }

    #[test]
    fn test_noargs() -> BoxResult<()> {
        let args: Vec<String> = vec!["".to_string()];
        let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
        let routes = vec![];
        let urllib: std::sync::Arc<dyn Urllib> = std::sync::Arc::new(MockUrllib { routes });
        main(args, &mut buf, &urllib)?;

        let buf_vec = buf.into_inner();
        let buf_string = match std::str::from_utf8(&buf_vec) {
            Ok(v) => v,
            Err(e) => panic!("invalid UTF-8 sequence: {}", e),
        };
        assert!(
            buf_string.starts_with("usage: "),
            "buf_string is '{}'",
            buf_string
        );

        Ok(())
    }

    #[test]
    #[allow(deprecated)]
    fn osmify_error_description() {
        use std::error::Error;

        let error = OsmifyError {
            details: "test".to_string(),
        };
        assert_eq!(error.description(), "test".to_string());
    }
}
