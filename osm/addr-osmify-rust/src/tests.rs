/*
 * Copyright 2022 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
*/

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Tests the addr_osmify library crate.

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
    assert_eq!(main(args, &mut buf, &urllib), 0);

    let buf_vec = buf.into_inner();
    let buf_string = match std::str::from_utf8(&buf_vec) {
        Ok(v) => v,
        Err(e) => panic!("invalid UTF-8 sequence: {}", e),
    };
    assert_eq!(
        buf_string,
        "47.490592,19.030662 (1016 Budapest, Mészáros utca 58/a)\n"
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
    assert_eq!(main(args, &mut buf, &urllib), 1);

    let buf_vec = buf.into_inner();
    let buf_string = std::str::from_utf8(&buf_vec).unwrap();
    assert_eq!(
        buf_string,
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
    assert_eq!(main(args, &mut buf, &urllib), 1);

    let buf_vec = buf.into_inner();
    let buf_string = std::str::from_utf8(&buf_vec).unwrap();
    assert_eq!(buf_string, "failed to osmify: no results from nominatim",);

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
    assert_eq!(main(args, &mut buf, &urllib), 0);

    let buf_vec = buf.into_inner();
    let buf_string = match std::str::from_utf8(&buf_vec) {
        Ok(v) => v,
        Err(e) => panic!("invalid UTF-8 sequence: {}", e),
    };
    assert_eq!(
        buf_string,
        "47.47690895,19.0512550758533 (1111 Budapest, Karinthy Frigyes út 18)\n"
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
    assert_eq!(main(args, &mut buf, &urllib), 1);

    let buf_vec = buf.into_inner();
    let buf_string = std::str::from_utf8(&buf_vec).unwrap();
    assert_eq!(
        buf_string,
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
    assert_eq!(main(args, &mut buf, &urllib), 1);

    let buf_vec = buf.into_inner();
    let buf_string = std::str::from_utf8(&buf_vec).unwrap();
    assert_eq!(buf_string, "failed to osmify: no results from overpass",);

    Ok(())
}

#[test]
fn test_noargs() -> BoxResult<()> {
    let args: Vec<String> = vec!["".to_string()];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
    let routes = vec![];
    let urllib: std::sync::Arc<dyn Urllib> = std::sync::Arc::new(MockUrllib { routes });
    assert_eq!(main(args, &mut buf, &urllib), 0);

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
