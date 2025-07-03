/*
 * Copyright 2022 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
*/

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Tests the addr_osmify library crate.

use super::*;

/// How to generate mock overpass output files:
/// cat src/fixtures/hello.overpassql | curl -d @- -X POST http://overpass-api.de/api/interpreter
struct URLRoute {
    url: String,
    data_path: String,
    result_path: String,
}

struct TestNetwork {
    routes: Vec<URLRoute>,
    isatty: bool,
}

impl Network for TestNetwork {
    fn urlopen(&self, url: &str, data: &str) -> anyhow::Result<String> {
        for route in self.routes.iter() {
            if route.url != url {
                continue;
            }

            if !route.data_path.is_empty() {
                let expected = std::fs::read_to_string(&route.data_path)?;
                assert_eq!(data, expected);
            }

            if route.result_path.is_empty() {
                return Err(anyhow::anyhow!("empty result_path for url '{}'", url));
            }

            let contents = std::fs::read_to_string(&route.result_path)?;
            return Ok(contents);
        }

        Err(anyhow::anyhow!("unexpected url: {}", url))
    }

    fn isatty(&self) -> bool {
        self.isatty
    }
}

#[test]
fn test_happy() {
    let args: Vec<String> = vec!["".to_string(), "Mészáros utca 58/a, Budapest".to_string()];

    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    let routes = vec![
        URLRoute {
            url: "http://nominatim.openstreetmap.org/search.php?q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest&format=json".to_string(),
            data_path: "".to_string(),
            result_path: "src/fixtures/nominatim-happy.json".to_string()
        },
        URLRoute {
            url: "http://overpass-api.de/api/interpreter".to_string(),
            data_path: "src/fixtures/overpass-happy.overpassql".to_string(),
            result_path: "src/fixtures/overpass-happy.json".to_string()
        }
    ];
    let urllib: Arc<dyn Network> = Arc::new(TestNetwork {
        routes,
        isatty: true,
    });
    assert_eq!(main(args, &mut buf, &urllib), 0);

    let buf_vec = buf.into_inner();
    let buf_string = String::from_utf8(buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "47.490592,19.030662 (1016 Budapest, Mészáros utca 58/a)\n"
    );
}

#[test]
fn test_nominatim_json() {
    let args: Vec<String> = vec!["".to_string(), "Mészáros utca 58/a, Budapest".to_string()];

    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    let routes = vec![
        URLRoute {
            url: "http://nominatim.openstreetmap.org/search.php?q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest&format=json".to_string(),
            data_path: "".to_string(),
            result_path: "src/fixtures/nominatim-bad.json".to_string()
        },
    ];
    let urllib: Arc<dyn Network> = Arc::new(TestNetwork {
        routes,
        isatty: false,
    });
    assert_eq!(main(args, &mut buf, &urllib), 1);

    let buf_vec = buf.into_inner();
    let buf_string = String::from_utf8(buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "failed to osmify\n\nCaused by:\n    failed to parse JSON from nominatim: EOF while parsing an object at line 2 column 0\n",
    );
}

#[test]
fn test_nominatim_no_result() {
    let args: Vec<String> = vec!["".to_string(), "Mészáros utca 58/a, Budapestt".to_string()];

    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    let routes = vec![
        URLRoute {
            url: "http://nominatim.openstreetmap.org/search.php?q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapestt&format=json".to_string(),
            data_path: "".to_string(),
            result_path: "src/fixtures/nominatim-no-result.json".to_string()
        },
    ];
    let urllib: Arc<dyn Network> = Arc::new(TestNetwork {
        routes,
        isatty: false,
    });
    assert_eq!(main(args, &mut buf, &urllib), 1);

    let buf_vec = buf.into_inner();
    let buf_string = std::str::from_utf8(&buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "failed to osmify\n\nCaused by:\n    no results from nominatim\n",
    );
}

#[test]
fn test_prefer_buildings() {
    let args: Vec<String> = vec![
        "".to_string(),
        "Karinthy Frigyes út 18, Budapest".to_string(),
    ];

    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    let routes = vec![
        URLRoute {
            url: "http://nominatim.openstreetmap.org/search.php?q=Karinthy+Frigyes+%C3%BAt+18%2C+Budapest&format=json".to_string(),
            data_path: "".to_string(),
            result_path: "src/fixtures/nominatim-prefer-buildings.json".to_string()
        },
        URLRoute {
            url: "http://overpass-api.de/api/interpreter".to_string(),
            data_path: "src/fixtures/overpass-prefer-buildings.overpassql".to_string(),
            result_path: "src/fixtures/overpass-prefer-buildings.json".to_string()
        }
    ];
    let urllib: Arc<dyn Network> = Arc::new(TestNetwork {
        routes,
        isatty: false,
    });
    assert_eq!(main(args, &mut buf, &urllib), 0);

    let buf_vec = buf.into_inner();
    let buf_string = String::from_utf8(buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "47.47690895,19.0512550758533 (1111 Budapest, Karinthy Frigyes út 18)\n"
    );
}

/// Test what happens when we try to prefer buildings, but that fails.
#[test]
fn test_prefer_buildings_fail() {
    let args: Vec<String> = vec![
        "".to_string(),
        "Karinthy Frigyes út 18, Budapest".to_string(),
    ];

    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    let routes = vec![
        URLRoute {
            url: "http://nominatim.openstreetmap.org/search.php?q=Karinthy+Frigyes+%C3%BAt+18%2C+Budapest&format=json".to_string(),
            data_path: "".to_string(),
            result_path: "src/fixtures/nominatim-prefer-buildings-fail.json".to_string()
        },
        URLRoute {
            url: "http://overpass-api.de/api/interpreter".to_string(),
            data_path: "src/fixtures/overpass-prefer-buildings-fail.overpassql".to_string(),
            result_path: "src/fixtures/overpass-prefer-buildings-fail.json".to_string()
        }
    ];
    let urllib: Arc<dyn Network> = Arc::new(TestNetwork {
        routes,
        isatty: false,
    });

    let ret = main(args, &mut buf, &urllib);

    let buf_vec = buf.into_inner();
    let buf_string = String::from_utf8(buf_vec).unwrap();
    assert_eq!(ret, 1);
    assert_eq!(
        buf_string,
        "failed to osmify\n\nCaused by:\n    failed to parse JSON from overpass: missing field `addr:city` at line 19 column 3\n"
    );
}

#[test]
fn test_overpass_json() {
    let args: Vec<String> = vec!["".to_string(), "Mészáros utca 58/a, Budapest".to_string()];

    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    let routes = vec![
        URLRoute {
            url: "http://nominatim.openstreetmap.org/search.php?q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest&format=json".to_string(),
            data_path: "".to_string(),
            result_path: "src/fixtures/nominatim-happy.json".to_string()
        },
        URLRoute {
            url: "http://overpass-api.de/api/interpreter".to_string(),
            data_path: "src/fixtures/overpass-happy.overpassql".to_string(),
            result_path: "src/fixtures/overpass-bad.json".to_string()
        }
    ];
    let urllib: Arc<dyn Network> = Arc::new(TestNetwork {
        routes,
        isatty: false,
    });
    assert_eq!(main(args, &mut buf, &urllib), 1);

    let buf_vec = buf.into_inner();
    let buf_string = std::str::from_utf8(&buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "failed to osmify\n\nCaused by:\n    failed to parse JSON from overpass: EOF while parsing a value at line 3 column 0\n",
    );
}

#[test]
fn test_overpass_noresult() {
    let args: Vec<String> = vec!["".to_string(), "Mészáros utca 58/a, Budapest".to_string()];

    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    let routes = vec![
        URLRoute {
            url: "http://nominatim.openstreetmap.org/search.php?q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest&format=json".to_string(),
            data_path: "".to_string(),
            result_path: "src/fixtures/nominatim-happy.json".to_string()
        },
        URLRoute {
            url: "http://overpass-api.de/api/interpreter".to_string(),
            data_path: "src/fixtures/overpass-happy.overpassql".to_string(),
            result_path: "src/fixtures/overpass-noresult.json".to_string()
        }
    ];
    let urllib: Arc<dyn Network> = Arc::new(TestNetwork {
        routes,
        isatty: false,
    });
    assert_eq!(main(args, &mut buf, &urllib), 1);

    let buf_vec = buf.into_inner();
    let buf_string = std::str::from_utf8(&buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "failed to osmify\n\nCaused by:\n    no results from overpass\n",
    );
}

#[test]
fn test_noargs() {
    let args: Vec<String> = vec!["".to_string()];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
    let routes = vec![];
    let urllib: std::sync::Arc<dyn Network> = std::sync::Arc::new(TestNetwork {
        routes,
        isatty: false,
    });
    assert_eq!(main(args, &mut buf, &urllib), 0);

    let buf_vec = buf.into_inner();
    let buf_string = std::str::from_utf8(&buf_vec).unwrap();
    assert!(buf_string.starts_with("usage: "));
}

/// Checks if the test network impl catches missing mocks.
#[test]
fn test_network() {
    let routes = vec![URLRoute {
        url: "http://www.example1.com".to_string(),
        data_path: "".to_string(),
        result_path: "".to_string(),
    }];

    // Empty result_path.
    let urllib: std::sync::Arc<dyn Network> = std::sync::Arc::new(TestNetwork {
        routes,
        isatty: false,
    });

    let ret = urllib.urlopen("http://www.example1.com", "");

    assert!(ret.is_err());

    // Not routed URL.
    let ret = urllib.urlopen("http://www.example2.com", "");

    assert!(ret.is_err());
}
