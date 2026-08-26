/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

use std::collections::HashMap;
use std::rc::Rc;

use crate::Context;
use crate::NetworkResponse;
use crate::run;
use crate::tests::TestNetwork;
use crate::tests::TestProcess;
use crate::tests::TestTime;
use crate::tests::gpsbabel_cmdline;
use crate::tests::setup_config;

fn parse_html(buf: std::io::Cursor<Vec<u8>>) -> scraper::Html {
    let html = String::from_utf8(buf.into_inner()).unwrap();
    scraper::Html::parse_document(&html)
}

fn query_selectors(
    document: &scraper::Html,
    row_selector: &str,
    cell_selector: &str,
) -> Vec<String> {
    let row_sel = scraper::Selector::parse(row_selector).unwrap();
    let cell_sel = scraper::Selector::parse(cell_selector).unwrap();
    document
        .select(&row_sel)
        .map(|row| {
            row.select(&cell_sel)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .to_string()
        })
        .collect()
}

#[test]
fn test_format_duration() {
    assert_eq!(super::format_duration(3600), "1:00:00");
    assert_eq!(super::format_duration(16864), "4:41:04");
    assert_eq!(super::format_duration(59), "0:00:59");
}

#[test]
fn test_format_distance() {
    assert_eq!(super::format_distance(15962.8), "15.96 km");
    assert_eq!(super::format_distance(1000.0), "1.00 km");
    assert_eq!(super::format_distance(50.0), "0.05 km");
}

#[test]
fn test_format_elevation() {
    assert_eq!(super::format_elevation(1038.1), "1038 m");
    assert_eq!(super::format_elevation(100.9), "101 m");
    assert_eq!(super::format_elevation(0.0), "0 m");
}

#[test]
fn test_query_countries() {
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();

    // 1. Activity in Austria (comes before Germany and Hungary by name)
    let meta_path_at = activities_dir
        .join("2025-01-01T00-00-00Z_1.meta.json")
        .unwrap();
    meta_path_at
        .create_file()
        .unwrap()
        .write_all(b"{\"id\": 1, \"name\": \"AT\", \"start_time\": \"2025-01-01T00:00:00Z\", \"sport_type\": \"Ride\", \"moving_time_raw\": 3600, \"elapsed_time_raw\": 4000, \"distance_raw\": 1000.0, \"elevation_gain_raw\": 100.0}")
        .unwrap();

    // 2. Activities in Hungary (most activities, should come first)
    let meta_path_hu1 = activities_dir
        .join("2025-02-01T00-00-00Z_2.meta.json")
        .unwrap();
    meta_path_hu1
        .create_file()
        .unwrap()
        .write_all(b"{\"id\": 2, \"name\": \"HU1\", \"start_time\": \"2025-02-01T00:00:00Z\", \"sport_type\": \"Ride\", \"moving_time_raw\": 3600, \"elapsed_time_raw\": 4000, \"distance_raw\": 1000.0, \"elevation_gain_raw\": 100.0}")
        .unwrap();
    let meta_path_hu2 = activities_dir
        .join("2025-02-02T00-00-00Z_3.meta.json")
        .unwrap();
    meta_path_hu2
        .create_file()
        .unwrap()
        .write_all(b"{\"id\": 3, \"name\": \"HU2\", \"start_time\": \"2025-02-02T00:00:00Z\", \"sport_type\": \"Ride\", \"moving_time_raw\": 3600, \"elapsed_time_raw\": 4000, \"distance_raw\": 1000.0, \"elevation_gain_raw\": 100.0}")
        .unwrap();

    // 3. Activity in Germany (same count as AT, should come after AT by name)
    let meta_path_de = activities_dir
        .join("2025-03-01T00-00-00Z_4.meta.json")
        .unwrap();
    meta_path_de
        .create_file()
        .unwrap()
        .write_all(b"{\"id\": 4, \"name\": \"DE\", \"start_time\": \"2025-03-01T00:00:00Z\", \"sport_type\": \"Ride\", \"moving_time_raw\": 3600, \"elapsed_time_raw\": 4000, \"distance_raw\": 1000.0, \"elevation_gain_raw\": 100.0}")
        .unwrap();

    // Each activity has a matching .fit file, gpsbabel provides its coordinates below.
    for base_name in [
        "2025-01-01T00-00-00Z_1",
        "2025-02-01T00-00-00Z_2",
        "2025-02-02T00-00-00Z_3",
        "2025-03-01T00-00-00Z_4",
    ] {
        activities_dir
            .join(format!("{}.fit", base_name))
            .unwrap()
            .create_file()
            .unwrap();
    }

    let mut responses = HashMap::new();
    // Austria is served from cache, so no Nominatim request needed for it.
    responses.insert(
        "https://nominatim.openstreetmap.org/reverse?lat=47&lon=19&format=json".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: b"{\"address\": {\"country\": \"Hungary\"}}".to_vec(),
        },
    );
    responses.insert(
        "https://nominatim.openstreetmap.org/reverse?lat=47.1&lon=19.1&format=json".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: b"{\"address\": {\"country\": \"Hungary\"}}".to_vec(),
        },
    );
    responses.insert(
        "https://nominatim.openstreetmap.org/reverse?lat=52&lon=13&format=json".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: b"{\"address\": {\"country\": \"Germany\"}}".to_vec(),
        },
    );
    let network = Rc::new(TestNetwork { responses });
    // GeoJSON coordinates are [longitude, latitude, elevation].
    // Only need gpsbabel for non-cached activities (HU1, HU2, DE).
    let command_outputs = [
        (
            gpsbabel_cmdline("2025-02-01T00-00-00Z_2"),
            r#"{"features": [{"geometry": {"coordinates": [[19.0, 47.0, 149.4]]}}]}"#.to_string(),
        ),
        (
            gpsbabel_cmdline("2025-02-02T00-00-00Z_3"),
            r#"{"features": [{"geometry": {"coordinates": [[19.1, 47.1, 149.4]]}}]}"#.to_string(),
        ),
        (
            gpsbabel_cmdline("2025-03-01T00-00-00Z_4"),
            r#"{"features": [{"geometry": {"coordinates": [[13.0, 52.0, 149.4]]}}]}"#.to_string(),
        ),
    ];
    let command_outputs: Vec<(&str, &str)> = command_outputs
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    let process = Rc::new(TestProcess::new(&command_outputs));
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process,
        time,
    };
    setup_config(&fs);

    // Pre-populate cache for the Austria activity, so the cache-hit path is exercised.
    let cache_path = fs
        .join(".local/share/strava-mirror/countries-cache.json")
        .unwrap();
    cache_path.parent().create_dir_all().unwrap();
    cache_path
        .create_file()
        .unwrap()
        .write_all(b"{\"2025-01-01T00-00-00Z_1\": \"Austria\"}")
        .unwrap();

    // When querying countries:
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "countries".to_string(),
    ];
    run(args, &mut buf, &ctx).unwrap();
}

#[test]
fn test_query_top_walks_by_time() {
    // Given three activities (2 walks, 1 ride):
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();

    // 1. Long walk
    let meta_path_1 = activities_dir
        .join("2025-01-01T10-00-00Z_1.meta.json")
        .unwrap();
    let content_1 = r#"{"id": 1, "name": "long walk", "start_time": "2025-01-01T10:00:00Z", "sport_type": "Walk", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 10000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(content_1.as_bytes())
        .unwrap();

    // 2. Short walk
    let meta_path_2 = activities_dir
        .join("2025-01-02T10-00-00Z_2.meta.json")
        .unwrap();
    let content_2 = r#"{"id": 2, "name": "short walk", "start_time": "2025-01-02T10:00:00Z", "sport_type": "Walk", "moving_time_raw": 5000, "elapsed_time_raw": 5400, "distance_raw": 5000.0, "elevation_gain_raw": 200.0}"#;
    meta_path_2
        .create_file()
        .unwrap()
        .write_all(content_2.as_bytes())
        .unwrap();

    // 3. Long ride (should be ignored)
    let meta_path_3 = activities_dir
        .join("2025-01-03T10-00-00Z_3.meta.json")
        .unwrap();
    let content_3 = r#"{"id": 3, "name": "long ride", "start_time": "2025-01-03T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 20000, "elapsed_time_raw": 20400, "distance_raw": 50000.0, "elevation_gain_raw": 1000.0}"#;
    meta_path_3
        .create_file()
        .unwrap()
        .write_all(content_3.as_bytes())
        .unwrap();

    let network = Rc::new(TestNetwork {
        responses: HashMap::new(),
    });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };

    // When querying top walks by time:
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "top-walks-by-time".to_string(),
    ];
    run(args, &mut buf, &ctx).unwrap();

    // Then the result has a table with 2 non-header rows: long walk first, short walk second.
    let document = parse_html(buf);
    let names = query_selectors(&document, "tbody tr", "td:nth-child(3) a");
    assert_eq!(names, ["long walk", "short walk"]);
}

#[test]
fn test_query_top_walks_by_distance() {
    // Given two activities (2 walks):
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();

    // 1. Short walk (by distance)
    let meta_path_1 = activities_dir
        .join("2025-01-01T10-00-00Z_1.meta.json")
        .unwrap();
    let content_1 = r#"{"id": 1, "name": "short distance walk", "start_time": "2025-01-01T10:00:00Z", "sport_type": "Walk", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 5000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(content_1.as_bytes())
        .unwrap();

    // 2. Long walk (by distance)
    let meta_path_2 = activities_dir
        .join("2025-01-02T10-00-00Z_2.meta.json")
        .unwrap();
    let content_2 = r#"{"id": 2, "name": "long distance walk", "start_time": "2025-01-02T10:00:00Z", "sport_type": "Walk", "moving_time_raw": 5000, "elapsed_time_raw": 5400, "distance_raw": 10000.0, "elevation_gain_raw": 200.0}"#;
    meta_path_2
        .create_file()
        .unwrap()
        .write_all(content_2.as_bytes())
        .unwrap();

    let network = Rc::new(TestNetwork {
        responses: HashMap::new(),
    });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };

    // When querying top walks by distance:
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "top-walks-by-distance".to_string(),
    ];
    run(args, &mut buf, &ctx).unwrap();

    // Then the result has a table with 2 non-header rows: long distance first, short distance second.
    let document = parse_html(buf);
    let names = query_selectors(&document, "tbody tr", "td:nth-child(3) a");
    assert_eq!(names, ["long distance walk", "short distance walk"]);
}

#[test]
fn test_query_top_walks_by_elevation() {
    // Given two activities (2 walks):
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();

    // 1. Walk with low elevation
    let meta_path_1 = activities_dir
        .join("2025-01-01T10-00-00Z_1.meta.json")
        .unwrap();
    let content_1 = r#"{"id": 1, "name": "flat walk", "start_time": "2025-01-01T10:00:00Z", "sport_type": "Walk", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 10000.0, "elevation_gain_raw": 10.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(content_1.as_bytes())
        .unwrap();

    // 2. Walk with high elevation
    let meta_path_2 = activities_dir
        .join("2025-01-02T10-00-00Z_2.meta.json")
        .unwrap();
    let content_2 = r#"{"id": 2, "name": "mountain walk", "start_time": "2025-01-02T10:00:00Z", "sport_type": "Walk", "moving_time_raw": 5000, "elapsed_time_raw": 5400, "distance_raw": 5000.0, "elevation_gain_raw": 1000.0}"#;
    meta_path_2
        .create_file()
        .unwrap()
        .write_all(content_2.as_bytes())
        .unwrap();

    let network = Rc::new(TestNetwork {
        responses: HashMap::new(),
    });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };

    // When querying top walks by elevation:
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "top-walks-by-elevation".to_string(),
    ];
    run(args, &mut buf, &ctx).unwrap();

    // Then the result has a table with 2 non-header rows: mountain first, flat second.
    let document = parse_html(buf);
    let names = query_selectors(&document, "tbody tr", "td:nth-child(3) a");
    assert_eq!(names, ["mountain walk", "flat walk"]);
}

#[test]
fn test_query_top_rides_by_time() {
    // Given two activities (2 rides):
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();

    // 1. Short ride
    let meta_path_1 = activities_dir
        .join("2025-01-01T10-00-00Z_1.meta.json")
        .unwrap();
    let content_1 = r#"{"id": 1, "name": "short ride", "start_time": "2025-01-01T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 5000, "elapsed_time_raw": 5400, "distance_raw": 25000.0, "elevation_gain_raw": 200.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(content_1.as_bytes())
        .unwrap();

    // 2. Long ride
    let meta_path_2 = activities_dir
        .join("2025-01-02T10-00-00Z_2.meta.json")
        .unwrap();
    let content_2 = r#"{"id": 2, "name": "long ride", "start_time": "2025-01-02T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 50000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_2
        .create_file()
        .unwrap()
        .write_all(content_2.as_bytes())
        .unwrap();

    let network = Rc::new(TestNetwork {
        responses: HashMap::new(),
    });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };

    // When querying top rides by time:
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "top-rides-by-time".to_string(),
    ];
    run(args, &mut buf, &ctx).unwrap();

    // Then the result has a table with 2 non-header rows: long ride first, short ride second.
    let document = parse_html(buf);
    let names = query_selectors(&document, "tbody tr", "td:nth-child(3) a");
    assert_eq!(names, ["long ride", "short ride"]);
}

#[test]
fn test_query_top_rides_by_distance() {
    // Given two activities (2 rides):
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();

    // 1. Long time ride (short distance)
    let meta_path_1 = activities_dir
        .join("2025-01-01T10-00-00Z_1.meta.json")
        .unwrap();
    let content_1 = r#"{"id": 1, "name": "long time ride", "start_time": "2025-01-01T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 20000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(content_1.as_bytes())
        .unwrap();

    // 2. Short time ride (long distance)
    let meta_path_2 = activities_dir
        .join("2025-01-02T10-00-00Z_2.meta.json")
        .unwrap();
    let content_2 = r#"{"id": 2, "name": "long distance ride", "start_time": "2025-01-02T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 5000, "elapsed_time_raw": 5400, "distance_raw": 50000.0, "elevation_gain_raw": 200.0}"#;
    meta_path_2
        .create_file()
        .unwrap()
        .write_all(content_2.as_bytes())
        .unwrap();

    let network = Rc::new(TestNetwork {
        responses: HashMap::new(),
    });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };

    // When querying top rides by distance:
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "top-rides-by-distance".to_string(),
    ];
    run(args, &mut buf, &ctx).unwrap();

    // Then the result has a table with 2 non-header rows: long distance first, short distance second.
    let document = parse_html(buf);
    let names = query_selectors(&document, "tbody tr", "td:nth-child(3) a");
    assert_eq!(names, ["long distance ride", "long time ride"]);
}

#[test]
fn test_query_top_rides_by_elevation() {
    // Given two activities (2 rides):
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();

    // 1. Long ride (short elevation)
    let meta_path_1 = activities_dir
        .join("2025-01-01T10-00-00Z_1.meta.json")
        .unwrap();
    let content_1 = r#"{"id": 1, "name": "long ride", "start_time": "2025-01-01T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 50000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(content_1.as_bytes())
        .unwrap();

    // 2. Short ride (high elevation)
    let meta_path_2 = activities_dir
        .join("2025-01-02T10-00-00Z_2.meta.json")
        .unwrap();
    let content_2 = r#"{"id": 2, "name": "mountain ride", "start_time": "2025-01-02T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 5000, "elapsed_time_raw": 5400, "distance_raw": 25000.0, "elevation_gain_raw": 2000.0}"#;
    meta_path_2
        .create_file()
        .unwrap()
        .write_all(content_2.as_bytes())
        .unwrap();

    let network = Rc::new(TestNetwork {
        responses: HashMap::new(),
    });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };

    // When querying top rides by elevation:
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "top-rides-by-elevation".to_string(),
    ];
    run(args, &mut buf, &ctx).unwrap();

    // Then the result has a table with 2 non-header rows: mountain first, flat second.
    let document = parse_html(buf);
    let names = query_selectors(&document, "tbody tr", "td:nth-child(3) a");
    assert_eq!(names, ["mountain ride", "long ride"]);
}

#[test]
fn test_query_longest_rides_by_year() {
    // Given two rides in one year and one ride in another year:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir_2024 = fs
        .join(".local/share/strava-mirror/activities/2024")
        .unwrap();
    activities_dir_2024.create_dir_all().unwrap();
    let activities_dir_2025 = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir_2025.create_dir_all().unwrap();

    // 1. Short ride in 2024.
    let meta_path_1 = activities_dir_2024
        .join("2024-01-01T10-00-00Z_1.meta.json")
        .unwrap();
    let content_1 = r#"{"id": 1, "name": "short 2024 ride", "start_time": "2024-01-01T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 5000, "elapsed_time_raw": 5400, "distance_raw": 25000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(content_1.as_bytes())
        .unwrap();

    // 2. Short ride in 2025.
    let meta_path_2 = activities_dir_2025
        .join("2025-01-01T10-00-00Z_2.meta.json")
        .unwrap();
    let content_2 = r#"{"id": 2, "name": "short 2025 ride", "start_time": "2025-01-01T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 5000, "elapsed_time_raw": 5400, "distance_raw": 30000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_2
        .create_file()
        .unwrap()
        .write_all(content_2.as_bytes())
        .unwrap();

    // 3. Long ride in 2025, updates the year's best.
    let meta_path_3 = activities_dir_2025
        .join("2025-01-02T10-00-00Z_3.meta.json")
        .unwrap();
    let content_3 = r#"{"id": 3, "name": "long 2025 ride", "start_time": "2025-01-02T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 60000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_3
        .create_file()
        .unwrap()
        .write_all(content_3.as_bytes())
        .unwrap();
    // 4. Shortest ride in 2025, doesn't update the year's best.
    let meta_path_4 = activities_dir_2025
        .join("2025-01-03T10-00-00Z_4.meta.json")
        .unwrap();
    let content_4 = r#"{"id": 4, "name": "shortest 2025 ride", "start_time": "2025-01-03T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 20000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_4
        .create_file()
        .unwrap()
        .write_all(content_4.as_bytes())
        .unwrap();

    // 5. A non-Ride.
    let meta_path_5 = activities_dir_2025
        .join("2025-01-01T10-00-00Z_5.meta.json")
        .unwrap();
    let content_5 = r#"{"id": 5, "name": "hungarian walk", "start_time": "2025-01-01T10:00:00Z", "sport_type": "Walk", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 10000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_5
        .create_file()
        .unwrap()
        .write_all(content_5.as_bytes())
        .unwrap();

    let network = Rc::new(TestNetwork {
        responses: HashMap::new(),
    });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };

    // When querying the longest ride by year:
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "longest-rides-by-year".to_string(),
    ];
    run(args, &mut buf, &ctx).unwrap();

    // Then the result has a table with 2 non-header rows: 2025 first, then 2024.
    let document = parse_html(buf);
    let names = query_selectors(&document, "tbody tr", "td:nth-child(3) a");
    assert_eq!(names, ["long 2025 ride", "short 2024 ride"]);
}

#[test]
fn test_query_total_distance_by_year() {
    // Given two rides in one year and one ride in another year:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir_2024 = fs
        .join(".local/share/strava-mirror/activities/2024")
        .unwrap();
    activities_dir_2024.create_dir_all().unwrap();
    let activities_dir_2025 = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir_2025.create_dir_all().unwrap();

    // 1. Ride in 2024.
    let meta_path_1 = activities_dir_2024
        .join("2024-01-01T10-00-00Z_1.meta.json")
        .unwrap();
    let content_1 = r#"{"id": 1, "name": "2024 ride", "start_time": "2024-01-01T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 5000, "elapsed_time_raw": 5400, "distance_raw": 25000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(content_1.as_bytes())
        .unwrap();

    // 2. First ride in 2025.
    let meta_path_2 = activities_dir_2025
        .join("2025-01-01T10-00-00Z_2.meta.json")
        .unwrap();
    let content_2 = r#"{"id": 2, "name": "2025 ride 1", "start_time": "2025-01-01T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 5000, "elapsed_time_raw": 5400, "distance_raw": 30000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_2
        .create_file()
        .unwrap()
        .write_all(content_2.as_bytes())
        .unwrap();

    // 3. Second ride in 2025, adds to the year's total.
    let meta_path_3 = activities_dir_2025
        .join("2025-01-02T10-00-00Z_3.meta.json")
        .unwrap();
    let content_3 = r#"{"id": 3, "name": "2025 ride 2", "start_time": "2025-01-02T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 60000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_3
        .create_file()
        .unwrap()
        .write_all(content_3.as_bytes())
        .unwrap();

    // 4. A Walk in 2025, also contributes to the year's total.
    let meta_path_4 = activities_dir_2025
        .join("2025-01-01T10-00-00Z_4.meta.json")
        .unwrap();
    let content_4 = r#"{"id": 4, "name": "hungarian walk", "start_time": "2025-01-01T10:00:00Z", "sport_type": "Walk", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 10000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_4
        .create_file()
        .unwrap()
        .write_all(content_4.as_bytes())
        .unwrap();

    let network = Rc::new(TestNetwork {
        responses: HashMap::new(),
    });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };

    // When querying the total distance by year:
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "total-distance-by-year".to_string(),
    ];
    run(args, &mut buf, &ctx).unwrap();

    // Then the result has a table with 2 non-header rows: 2025 first, then 2024.
    let document = parse_html(buf);
    let names = query_selectors(&document, "tbody tr", "td:first-child");
    assert_eq!(names, ["2025", "2024"]);
}

#[test]
fn test_query_activity_count_by_year() {
    // Given two rides in one year and one ride in another year:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir_2024 = fs
        .join(".local/share/strava-mirror/activities/2024")
        .unwrap();
    activities_dir_2024.create_dir_all().unwrap();
    let activities_dir_2025 = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir_2025.create_dir_all().unwrap();

    // 1. Ride in 2024.
    let meta_path_1 = activities_dir_2024
        .join("2024-01-01T10-00-00Z_1.meta.json")
        .unwrap();
    let content_1 = r#"{"id": 1, "name": "2024 ride", "start_time": "2024-01-01T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 5000, "elapsed_time_raw": 5400, "distance_raw": 25000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(content_1.as_bytes())
        .unwrap();

    // 2. Ride in 2025.
    let meta_path_2 = activities_dir_2025
        .join("2025-01-01T10-00-00Z_2.meta.json")
        .unwrap();
    let content_2 = r#"{"id": 2, "name": "2025 ride", "start_time": "2025-01-01T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 5000, "elapsed_time_raw": 5400, "distance_raw": 30000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_2
        .create_file()
        .unwrap()
        .write_all(content_2.as_bytes())
        .unwrap();

    // 3. Walk in 2025, also counted.
    let meta_path_3 = activities_dir_2025
        .join("2025-01-02T10-00-00Z_3.meta.json")
        .unwrap();
    let content_3 = r#"{"id": 3, "name": "2025 walk", "start_time": "2025-01-02T10:00:00Z", "sport_type": "Walk", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 10000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_3
        .create_file()
        .unwrap()
        .write_all(content_3.as_bytes())
        .unwrap();

    let network = Rc::new(TestNetwork {
        responses: HashMap::new(),
    });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };

    // When querying the activity count by year:
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "activity-count-by-year".to_string(),
    ];
    run(args, &mut buf, &ctx).unwrap();

    // Then the result has a table with 2 non-header rows: 2025 first, then 2024.
    let document = parse_html(buf);
    let names = query_selectors(&document, "tbody tr", "td:first-child");
    assert_eq!(names, ["2025", "2024"]);
}

#[test]
fn test_query_activity_type_breakdown() {
    // Given a ride and a walk:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();

    let meta_path_1 = activities_dir
        .join("2025-01-01T10-00-00Z_1.meta.json")
        .unwrap();
    let content_1 = r#"{"id": 1, "name": "ride", "start_time": "2025-01-01T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 5000, "elapsed_time_raw": 5400, "distance_raw": 25000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(content_1.as_bytes())
        .unwrap();

    let meta_path_2 = activities_dir
        .join("2025-01-02T10-00-00Z_2.meta.json")
        .unwrap();
    let content_2 = r#"{"id": 2, "name": "walk", "start_time": "2025-01-02T10:00:00Z", "sport_type": "Walk", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 10000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_2
        .create_file()
        .unwrap()
        .write_all(content_2.as_bytes())
        .unwrap();

    // 3. Later ride, becomes the latest Ride.
    let meta_path_3 = activities_dir
        .join("2025-06-01T10-00-00Z_3.meta.json")
        .unwrap();
    let content_3 = r#"{"id": 3, "name": "later ride", "start_time": "2025-06-01T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 3000, "elapsed_time_raw": 3400, "distance_raw": 15000.0, "elevation_gain_raw": 200.0}"#;
    meta_path_3
        .create_file()
        .unwrap()
        .write_all(content_3.as_bytes())
        .unwrap();

    let network = Rc::new(TestNetwork {
        responses: HashMap::new(),
    });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };

    // When querying the activity type breakdown:
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "activity-type-breakdown".to_string(),
    ];
    run(args, &mut buf, &ctx).unwrap();

    // Then no failure occurs.
}

#[test]
fn test_query_all() {
    // Given some activities:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();

    // 1. Walk in Hungary
    let meta_path_1 = activities_dir
        .join("2025-01-01T10-00-00Z_1.meta.json")
        .unwrap();
    let content_1 = r#"{"id": 1, "name": "hungarian walk", "start_time": "2025-01-01T10:00:00Z", "sport_type": "Walk", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 10000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(content_1.as_bytes())
        .unwrap();
    activities_dir
        .join("2025-01-01T10-00-00Z_1.fit")
        .unwrap()
        .create_file()
        .unwrap();

    let mut responses = HashMap::new();
    responses.insert(
        "https://nominatim.openstreetmap.org/reverse?lat=47&lon=19&format=json".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: b"{\"address\": {\"country\": \"Hungary\"}}".to_vec(),
        },
    );
    let network = Rc::new(TestNetwork { responses });
    let cmdline = gpsbabel_cmdline("2025-01-01T10-00-00Z_1");
    // GeoJSON coordinates are [longitude, latitude, elevation], so this is lat=47, lon=19.
    let geojson = r#"{"features": [{"geometry": {"coordinates": [[19.0, 47.0, 149.4]]}}]}"#;
    let command_outputs = [(cmdline.as_str(), geojson)];
    let process = Rc::new(TestProcess::new(&command_outputs));
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process,
        time,
    };
    setup_config(&fs);

    // When querying all:
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "all".to_string(),
    ];
    run(args, &mut buf, &ctx).unwrap();

    // Then no failure occurs.
}
