/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Tests the strava_mirror library crate.

use super::*;

struct TestNetwork {
    responses: HashMap<String, NetworkResponse>,
}

impl Network for TestNetwork {
    fn get(&self, url: &str, headers: &HashMap<String, String>) -> anyhow::Result<NetworkResponse> {
        if url.contains("nominatim.openstreetmap.org") {
            assert_eq!(headers.get("Accept-Language").unwrap(), "en-US");
        }
        // For now we have no case when we want to simulate a GET failing.
        println!("TestNetwork::get: url is '{}'", url);
        let response = self.responses.get(url).unwrap();
        return Ok(NetworkResponse {
            headers: response.headers.clone(),
            body: response.body.clone(),
        });
    }
}

struct TestProcess {
    /// Joined cmdline, output.
    command_outputs: std::cell::RefCell<std::collections::VecDeque<(String, String)>>,
}

impl TestProcess {
    fn new(command_outputs: &[(&str, &str)]) -> Self {
        let command_outputs = command_outputs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        TestProcess {
            command_outputs: std::cell::RefCell::new(command_outputs),
        }
    }
}

impl Process for TestProcess {
    fn command_output(&self, command: &str, args: &[&str]) -> anyhow::Result<String> {
        assert_eq!(command, "gpsbabel");
        let cmdline = args.join(" ");
        println!(
            "debug, TestProcess::command_output: cmdline is '{}'",
            cmdline
        );
        let mut command_outputs = self.command_outputs.borrow_mut();
        let command_output = command_outputs.pop_front().unwrap();
        assert_eq!(command_output.0, cmdline);
        Ok(command_output.1)
    }
}

struct TestTime {
    now: time::OffsetDateTime,
    sleep_called: std::cell::Cell<bool>,
}

impl Default for TestTime {
    fn default() -> Self {
        Self {
            now: time::macros::datetime!(2026-04-12 12:00:00 UTC),
            sleep_called: std::cell::Cell::new(false),
        }
    }
}

impl Time for TestTime {
    fn now(&self) -> time::OffsetDateTime {
        self.now
    }

    fn to_local_offset(&self, timestamp: i64) -> anyhow::Result<time::OffsetDateTime> {
        Ok(time::OffsetDateTime::from_unix_timestamp(timestamp)?)
    }

    fn sleep(&self, duration: std::time::Duration) {
        if duration.as_secs() > 0 {
            self.sleep_called.set(true);
        }
    }
}

fn setup_config(fs: &vfs::VfsPath) {
    let config_dir = fs.join(".config").unwrap();
    config_dir.create_dir_all().unwrap();
    let config_content = std::fs::read_to_string("src/fixtures/strava-mirrorrc").unwrap();
    config_dir
        .join("strava-mirrorrc")
        .unwrap()
        .create_file()
        .unwrap()
        .write_all(config_content.as_bytes())
        .unwrap();
}

#[test]
fn test_no_activities() {
    // Given no activities:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let mut responses = HashMap::new();
    let token_body = std::fs::read("src/fixtures/token.json").unwrap();
    responses.insert(
        "https://www.strava.com/oauth/token".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: token_body,
        },
    );
    responses.insert(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: b"{\"models\":[]}".to_vec(),
        },
    );
    let network = Rc::new(TestNetwork { responses });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };
    setup_config(&fs);

    // When mirroring activities:
    let args = vec!["strava-mirror".to_string()];
    let ret = run(args, &ctx);

    // Then make sure there is no failure:
    assert!(ret.is_ok());
}

#[test]
fn test_jwt_to_cookie_error() {
    // Given a config with an invalid JWT:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let mut responses = HashMap::new();
    let token_body = std::fs::read("src/fixtures/token.json").unwrap();
    responses.insert(
        "https://www.strava.com/oauth/token".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: token_body,
        },
    );
    let network = Rc::new(TestNetwork { responses });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };
    let config_dir = fs.join(".config").unwrap();
    config_dir.create_dir_all().unwrap();
    let config_content = r#"jwt = "invalid""#;
    config_dir
        .join("strava-mirrorrc")
        .unwrap()
        .create_file()
        .unwrap()
        .write_all(config_content.as_bytes())
        .unwrap();

    // When mirroring activities:
    let args = vec!["strava-mirror".to_string()];
    let ret = run(args, &ctx);

    // Then make sure there is a failure:
    assert!(ret.is_err());
    let err = ret.unwrap_err().to_string();
    assert!(err.contains("JWT doesn't have 3 parts"));
}

#[test]
fn test_jwt_to_cookie_expired() {
    // Given a config with an expired JWT:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let mut responses = HashMap::new();
    let token_body = std::fs::read("src/fixtures/token.json").unwrap();
    responses.insert(
        "https://www.strava.com/oauth/token".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: token_body,
        },
    );
    let network = Rc::new(TestNetwork { responses });
    // Config's JWT expires on 2026-05-07, so set "now" to 2026-05-09.
    let now = time::macros::datetime!(2026-05-09 12:00:00 UTC);
    let time = Rc::new(TestTime {
        now,
        sleep_called: std::cell::Cell::new(false),
    });
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };
    setup_config(&fs);

    // When mirroring activities:
    let args = vec!["strava-mirror".to_string()];
    let ret = run(args, &ctx);

    // Then make sure there is a failure:
    assert!(ret.is_err());
    let err = ret.unwrap_err().to_string();
    assert!(err.contains("JWT has expired"));
}

#[test]
fn test_mirror_activity() {
    // Given a single activity configured:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let mut responses = HashMap::new();
    let token_body = std::fs::read("src/fixtures/token.json").unwrap();
    responses.insert(
        "https://www.strava.com/oauth/token".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: token_body,
        },
    );
    let activities_body = std::fs::read("src/fixtures/activities-1.json").unwrap();
    responses.insert(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: activities_body,
        },
    );
    let mut data_headers = HashMap::new();
    data_headers.insert(
        "content-disposition".to_string(),
        "attachment; filename=\"activity.fit\"".to_string(),
    );
    responses.insert(
        "https://www.strava.com/activities/1/export_original".to_string(),
        NetworkResponse {
            headers: data_headers,
            body: b"fitdata".to_vec(),
        },
    );
    let network = Rc::new(TestNetwork { responses });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };
    setup_config(&fs);

    // When mirroring activities:
    let args = vec!["strava-mirror".to_string()];
    run(args, &ctx).unwrap();

    // Then make sure the 2 expeced files are created:
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    assert!(activities_dir.exists().unwrap());
    let base_name = "2025-04-09T07-44-48Z_1";
    assert!(
        activities_dir
            .join(format!("{}.meta.json", base_name))
            .unwrap()
            .exists()
            .unwrap()
    );
    assert!(
        activities_dir
            .join(format!("{}.fit", base_name))
            .unwrap()
            .exists()
            .unwrap()
    );
}

#[test]
fn test_list_activities_after() {
    // Given one activity mirrored already:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();
    let timestamp_str_1 = "2025-04-09T07-44-48Z";
    let base_name_1 = format!("{}_1", timestamp_str_1);
    let meta_path_1 = activities_dir
        .join(format!("{}.meta.json", base_name_1))
        .unwrap();
    let activity1_content = r#"{"id": 1, "start_time": "2025-04-09T07:44:48Z", "sport_type": "Ride", "moving_time_raw": 3600, "elapsed_time_raw": 4000, "distance_raw": 1000.0, "elevation_gain_raw": 100.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(activity1_content.as_bytes())
        .unwrap();
    activities_dir
        .join(format!("{}.fit", base_name_1))
        .unwrap()
        .create_file()
        .unwrap();
    let mut responses = HashMap::new();
    let token_body = std::fs::read("src/fixtures/token.json").unwrap();
    responses.insert(
        "https://www.strava.com/oauth/token".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: token_body,
        },
    );
    let activities_url =
        format!("https://www.strava.com/athlete/training_activities?new_activity_only=false",);
    let activities_body = std::fs::read("src/fixtures/activities-2.json").unwrap();
    responses.insert(
        activities_url,
        NetworkResponse {
            headers: HashMap::new(),
            body: activities_body,
        },
    );
    responses.insert(
        "https://www.strava.com/api/v3/activities/2".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: b"{\"id\": 2, \"name\": \"myactivity2\", \"moving_time_raw\": 3600, \"elapsed_time_raw\": 4000, \"distance_raw\": 1000.0, \"elevation_gain_raw\": 100.0}".to_vec(),
        },
    );
    let mut data_headers = HashMap::new();
    data_headers.insert(
        "content-disposition".to_string(),
        "attachment; filename=\"activity2.fit\"".to_string(),
    );
    responses.insert(
        "https://www.strava.com/activities/2/export_original".to_string(),
        NetworkResponse {
            headers: data_headers,
            body: b"fitdata2".to_vec(),
        },
    );
    let network = Rc::new(TestNetwork { responses });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };
    setup_config(&fs);

    // When doing incremental mirroring to get the second activity:
    let args = vec!["strava-mirror".to_string()];
    run(args, &ctx).unwrap();

    // Then make sure at the end we have the second activity mirrroed, too:
    let timestamp_str_2 = "2025-04-10T07-44-48Z";
    let base_name_2 = format!("{}_2", timestamp_str_2);
    assert!(
        activities_dir
            .join(format!("{}.meta.json", base_name_2))
            .unwrap()
            .exists()
            .unwrap()
    );
    assert!(
        activities_dir
            .join(format!("{}.fit", base_name_2))
            .unwrap()
            .exists()
            .unwrap()
    );
}

#[test]
fn test_get_mirrored_activities_ignore_file() {
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs.join(".local/share/strava-mirror/activities").unwrap();
    activities_dir.create_dir_all().unwrap();
    // Create a plain file under activities/, which should be ignored.
    activities_dir
        .join("ignore-me")
        .unwrap()
        .create_file()
        .unwrap();
    // Create a year directory and a valid meta file to ensure we still process other things.
    let year_dir = activities_dir.join("2025").unwrap();
    year_dir.create_dir_all().unwrap();
    // Create a file with an underscore but an invalid timestamp format.
    year_dir
        .join("invalid-format_1.meta.json")
        .unwrap()
        .create_file()
        .unwrap();
    let timestamp_str = "2025-04-09T07-44-48Z";
    let base_name = format!("{}_1", timestamp_str);
    let meta_path = year_dir.join(format!("{}.meta.json", base_name)).unwrap();
    meta_path.create_file().unwrap().write_all(b"{}").unwrap();

    let mirrored_activities = get_mirrored_activities(&activities_dir).unwrap();

    assert_eq!(mirrored_activities.len(), 1);
}

#[test]
fn test_mirror_activity_only_data() {
    // Given an activity where the meta is already mirrored:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();
    let timestamp_str = "2025-04-09T07-44-48Z";
    let base_name = format!("{}_1", timestamp_str);
    let meta_path = activities_dir
        .join(format!("{}.meta.json", base_name))
        .unwrap();
    meta_path.create_file().unwrap().write_all(b"{}").unwrap();

    let mut responses = HashMap::new();
    let token_body = std::fs::read("src/fixtures/token.json").unwrap();
    responses.insert(
        "https://www.strava.com/oauth/token".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: token_body,
        },
    );
    let activities_body = std::fs::read("src/fixtures/activities-1.json").unwrap();
    responses.insert(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: activities_body,
        },
    );
    // Notice that api/v3/activities/1 is NOT in the responses, so if we try to download it, we fail.
    let mut data_headers = HashMap::new();
    data_headers.insert(
        "content-disposition".to_string(),
        "attachment; filename=\"activity.fit\"".to_string(),
    );
    responses.insert(
        "https://www.strava.com/activities/1/export_original".to_string(),
        NetworkResponse {
            headers: data_headers,
            body: b"fitdata".to_vec(),
        },
    );
    let network = Rc::new(TestNetwork { responses });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };
    setup_config(&fs);

    // When mirroring activities:
    let args = vec!["strava-mirror".to_string()];
    run(args, &ctx).unwrap();

    // Then make sure the data file is created:
    assert!(
        activities_dir
            .join(format!("{}.fit", base_name))
            .unwrap()
            .exists()
            .unwrap()
    );
}

#[test]
fn test_mirror_activity_already_mirrored() {
    // Given an activity where both meta and data are already mirrored:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();
    let timestamp_str = "2025-04-09T07-44-48Z";
    let base_name = format!("{}_1", timestamp_str);
    let meta_path = activities_dir
        .join(format!("{}.meta.json", base_name))
        .unwrap();
    meta_path.create_file().unwrap().write_all(b"{}").unwrap();
    activities_dir
        .join(format!("{}.fit", base_name))
        .unwrap()
        .create_file()
        .unwrap();

    let mut responses = HashMap::new();
    let token_body = std::fs::read("src/fixtures/token.json").unwrap();
    responses.insert(
        "https://www.strava.com/oauth/token".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: token_body,
        },
    );
    let activities_url =
        format!("https://www.strava.com/athlete/training_activities?new_activity_only=false",);
    let activities_body = std::fs::read("src/fixtures/activities-1.json").unwrap();
    responses.insert(
        activities_url,
        NetworkResponse {
            headers: HashMap::new(),
            body: activities_body,
        },
    );
    // Notice that neither api/v3/activities/1 nor export_original is in the responses.
    let network = Rc::new(TestNetwork { responses });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };
    setup_config(&fs);

    // When mirroring activities:
    let args = vec!["strava-mirror".to_string()];
    run(args, &ctx).unwrap();

    // Then nothing should be downloaded (verified by lack of unexpected network requests).
}

#[test]
fn test_query_countries() {
    // Given an activity with location data and a nominatim cache:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();
    let timestamp_str = "2025-04-09T07-44-48Z";
    let base_name = format!("{}_1", timestamp_str);
    let meta_path = activities_dir
        .join(format!("{}.meta.json", base_name))
        .unwrap();
    // 47.0, 19.0 is in Hungary.
    meta_path
        .create_file()
        .unwrap()
        .write_all(b"{\"id\": 1, \"start_time\": \"2025-04-09T07:44:48Z\", \"start_latlng\": [47.0, 19.0], \"sport_type\": \"Ride\", \"moving_time_raw\": 3600, \"elapsed_time_raw\": 4000, \"distance_raw\": 1000.0, \"elevation_gain_raw\": 100.0}")
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
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };
    setup_config(&fs);

    // When querying countries:
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "countries".to_string(),
    ];
    run(args, &ctx).unwrap();

    // Then make sure the cache is created:
    let cache_path = fs
        .join(".local/share/strava-mirror/countries-cache.json")
        .unwrap();
    assert!(cache_path.exists().unwrap());
}

#[test]
fn test_get_sleep_duration() {
    let mut headers = HashMap::new();
    let now = time::macros::datetime!(2026-05-03 10:05:30 UTC);

    // No headers
    assert!(get_sleep_duration(&headers, now).is_err());

    // Limit provided but usage missing
    headers.insert("x-ratelimit-limit".to_string(), "100,1000".to_string());
    assert!(get_sleep_duration(&headers, now).is_err());
    headers.clear();

    // Headers provided but only one value (malformed)
    headers.insert("x-ratelimit-limit".to_string(), "100".to_string());
    headers.insert("x-ratelimit-usage".to_string(), "50".to_string());
    assert!(get_sleep_duration(&headers, now).is_err());
    headers.clear();

    // 15-min limit is not a number
    headers.insert("x-ratelimit-limit".to_string(), "foo,1000".to_string());
    headers.insert("x-ratelimit-usage".to_string(), "100,200".to_string());
    assert!(get_sleep_duration(&headers, now).is_err());
    headers.clear();

    // 15-min usage is not a number
    headers.insert("x-ratelimit-limit".to_string(), "100,1000".to_string());
    headers.insert("x-ratelimit-usage".to_string(), "foo,200".to_string());
    assert!(get_sleep_duration(&headers, now).is_err(),);
    headers.clear();

    // Daily limit is not a number
    headers.insert("x-ratelimit-limit".to_string(), "100,foo".to_string());
    headers.insert("x-ratelimit-usage".to_string(), "50,100".to_string());
    assert!(get_sleep_duration(&headers, now).is_err());
    headers.clear();

    // Daily usage is not a number
    headers.insert("x-ratelimit-limit".to_string(), "100,1000".to_string());
    headers.insert("x-ratelimit-usage".to_string(), "50,foo".to_string());
    assert!(get_sleep_duration(&headers, now).is_err());
    headers.clear();

    // 15-min limit reached (with 10s buffer)
    headers.insert("x-ratelimit-limit".to_string(), "100,1000".to_string());
    headers.insert("x-ratelimit-usage".to_string(), "100,200".to_string());
    // (15 - (5 % 15)) * 60 - 30 + 10 = 10 * 60 - 30 + 10 = 580
    assert_eq!(
        get_sleep_duration(&headers, now).unwrap(),
        std::time::Duration::from_secs(580)
    );

    // x-readratelimit-limit is prioritized
    headers.insert("x-readratelimit-limit".to_string(), "50,500".to_string());
    headers.insert("x-readratelimit-usage".to_string(), "50,100".to_string());
    // (15 - (5 % 15)) * 60 - 30 + 10 = 580
    assert_eq!(
        get_sleep_duration(&headers, now).unwrap(),
        std::time::Duration::from_secs(580)
    );
    headers.clear();

    // Daily limit reached (with 10s buffer)
    headers.insert("x-ratelimit-limit".to_string(), "100,1000".to_string());
    headers.insert("x-ratelimit-usage".to_string(), "50,1000".to_string());
    // next midnight is 2026-05-04 00:00:00.
    // From 10:05:30 to 00:00:00 is (24 - 10)h - 5m - 30s = 13h 54m 30s = 50070s
    // 50070 + 10 = 50080
    assert_eq!(
        get_sleep_duration(&headers, now).unwrap(),
        std::time::Duration::from_secs(50080)
    );

    // No limits reached
    headers.insert("x-ratelimit-usage".to_string(), "50,200".to_string());
    assert_eq!(
        get_sleep_duration(&headers, now).unwrap(),
        std::time::Duration::from_secs(0)
    );
}

#[test]
fn test_query_countries_summary() {
    // Given two activities in different countries and an existing cache:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();
    let timestamp_str = "2025-04-09T07-44-48Z";

    // Activity 1 in Hungary
    let meta_path1 = activities_dir
        .join(format!("{}_1.meta.json", timestamp_str))
        .unwrap();
    meta_path1
        .create_file()
        .unwrap()
        .write_all(b"{\"id\": 1, \"start_time\": \"2025-04-09T07:44:48Z\", \"sport_type\": \"Ride\", \"moving_time_raw\": 3600, \"elapsed_time_raw\": 4000, \"distance_raw\": 1000.0, \"elevation_gain_raw\": 100.0}")
        .unwrap();

    // Activity 2 in Austria (48.0, 16.0)
    let meta_path2 = activities_dir
        .join(format!("{}_2.meta.json", timestamp_str))
        .unwrap();
    meta_path2
        .create_file()
        .unwrap()
        .write_all(b"{\"id\": 2, \"start_time\": \"2025-04-09T07:44:48Z\", \"sport_type\": \"Ride\", \"moving_time_raw\": 3600, \"elapsed_time_raw\": 4000, \"distance_raw\": 1000.0, \"elevation_gain_raw\": 100.0}")
        .unwrap();
    let data_path2 = activities_dir
        .join(format!("{}_2.fit", timestamp_str))
        .unwrap();
    data_path2.create_file().unwrap();

    // Pre-existing cache for Hungary
    let cache_path = fs
        .join(".local/share/strava-mirror/countries-cache.json")
        .unwrap();
    cache_path.parent().create_dir_all().unwrap();
    cache_path
        .create_file()
        .unwrap()
        .write_all(b"{\"2025-04-09T07-44-48Z_1\": \"Hungary\"}")
        .unwrap();

    let mut responses = HashMap::new();
    // Only Austria needs to be fetched, Hungary is in cache.
    responses.insert(
        "https://nominatim.openstreetmap.org/reverse?lat=48&lon=16&format=json".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: b"{\"address\": {\"country\": \"Austria\"}}".to_vec(),
        },
    );
    let network = Rc::new(TestNetwork { responses });
    let home_dir = home::home_dir().unwrap();
    let home_path = home_dir.to_string_lossy();
    let cmdline = format!("-i garmin_fit -f {}/.local/share/strava-mirror/activities/2025/2025-04-09T07-44-48Z_2.fit -o geojson -F -", home_path);
    let geojson = r#"{"features": [{"geometry": {"coordinates": [[16.0, 48.0, 1.2]]}}]}"#;
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

    // When querying countries summary:
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "countries".to_string(),
        "--summary".to_string(),
    ];
    run(args, &ctx).unwrap();

    // Then make sure the cache is updated with Austria:
    let mut cache_content = String::new();
    cache_path
        .open_file()
        .unwrap()
        .read_to_string(&mut cache_content)
        .unwrap();
    assert!(cache_content.contains("Austria"));
}

#[test]
fn test_run_unknown_query() {
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
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
    setup_config(&fs);

    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "unknown".to_string(),
    ];
    let ret = run(args, &ctx);
    assert!(ret.is_err());
    assert_eq!(ret.unwrap_err().to_string(), "unknown query: unknown");
}

#[test]
fn test_get_countries_no_activities() {
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
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

    let countries = get_countries(&ctx).unwrap();
    assert!(countries.is_empty());
}

#[test]
fn test_get_activity_country_special_cases() {
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs.join(".local/share/strava-mirror/activities").unwrap();
    let year_dir = activities_dir.join("2025").unwrap();
    year_dir.create_dir_all().unwrap();
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

    // 1. File without .meta.json suffix: get_local_activities ignores it.
    let fit_path = year_dir.join("activity.fit").unwrap();
    fit_path.create_file().unwrap();
    let ret = get_local_activities(&ctx).unwrap();
    assert!(ret.is_empty());
}

#[test]
fn test_get_countries_ignore_file() {
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs.join(".local/share/strava-mirror/activities").unwrap();
    activities_dir.create_dir_all().unwrap();
    // Create a plain file under activities/, which should be ignored by get_countries.
    activities_dir
        .join("ignore-me")
        .unwrap()
        .create_file()
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

    let countries = get_countries(&ctx).unwrap();
    assert!(countries.is_empty());
}

#[test]
fn test_run_quiet() {
    // Given no activities and quiet mode:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let mut responses = HashMap::new();
    let token_body = std::fs::read("src/fixtures/token.json").unwrap();
    responses.insert(
        "https://www.strava.com/oauth/token".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: token_body,
        },
    );
    responses.insert(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: b"{\"models\":[]}".to_vec(),
        },
    );
    let network = Rc::new(TestNetwork { responses });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };
    setup_config(&fs);

    // When mirroring activities with --quiet:
    let args = vec!["strava-mirror".to_string(), "--quiet".to_string()];
    let ret = run(args, &ctx);

    // Then make sure there is no failure:
    assert!(ret.is_ok());
}

#[test]
fn test_query_countries_html() {
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
        .write_all(b"{\"id\": 1, \"name\": \"AT\", \"start_time\": \"2025-01-01T00:00:00Z\", \"start_latlng\": [48.0, 16.0], \"sport_type\": \"Ride\", \"moving_time_raw\": 3600, \"elapsed_time_raw\": 4000, \"distance_raw\": 1000.0, \"elevation_gain_raw\": 100.0}")
        .unwrap();

    // 2. Activities in Hungary (most activities, should come first)
    let meta_path_hu1 = activities_dir
        .join("2025-02-01T00-00-00Z_2.meta.json")
        .unwrap();
    meta_path_hu1
        .create_file()
        .unwrap()
        .write_all(b"{\"id\": 2, \"name\": \"HU1\", \"start_time\": \"2025-02-01T00:00:00Z\", \"start_latlng\": [47.0, 19.0], \"sport_type\": \"Ride\", \"moving_time_raw\": 3600, \"elapsed_time_raw\": 4000, \"distance_raw\": 1000.0, \"elevation_gain_raw\": 100.0}")
        .unwrap();
    let meta_path_hu2 = activities_dir
        .join("2025-02-02T00-00-00Z_3.meta.json")
        .unwrap();
    meta_path_hu2
        .create_file()
        .unwrap()
        .write_all(b"{\"id\": 3, \"name\": \"HU2\", \"start_time\": \"2025-02-02T00:00:00Z\", \"start_latlng\": [47.1, 19.1], \"sport_type\": \"Ride\", \"moving_time_raw\": 3600, \"elapsed_time_raw\": 4000, \"distance_raw\": 1000.0, \"elevation_gain_raw\": 100.0}")
        .unwrap();

    // 3. Activity in Germany (same count as AT, should come after AT by name)
    let meta_path_de = activities_dir
        .join("2025-03-01T00-00-00Z_4.meta.json")
        .unwrap();
    meta_path_de
        .create_file()
        .unwrap()
        .write_all(b"{\"id\": 4, \"name\": \"DE\", \"start_time\": \"2025-03-01T00:00:00Z\", \"start_latlng\": [52.0, 13.0], \"sport_type\": \"Ride\", \"moving_time_raw\": 3600, \"elapsed_time_raw\": 4000, \"distance_raw\": 1000.0, \"elevation_gain_raw\": 100.0}")
        .unwrap();

    let mut responses = HashMap::new();
    responses.insert(
        "https://nominatim.openstreetmap.org/reverse?lat=48&lon=16&format=json".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: b"{\"address\": {\"country\": \"Austria\"}}".to_vec(),
        },
    );
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
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };
    setup_config(&fs);

    // When querying countries as HTML:
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "countries".to_string(),
        "--html".to_string(),
    ];
    run(args, &ctx).unwrap();
}

#[test]
fn test_rate_limit_sleep() {
    // Given the 15-min rate limit is reached:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let mut responses = HashMap::new();
    let token_body = std::fs::read("src/fixtures/token.json").unwrap();
    responses.insert(
        "https://www.strava.com/oauth/token".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: token_body,
        },
    );
    let mut rate_limit_headers = HashMap::new();
    rate_limit_headers.insert("x-ratelimit-limit".to_string(), "100,1000".to_string());
    rate_limit_headers.insert("x-ratelimit-usage".to_string(), "100,200".to_string());
    responses.insert(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false".to_string(),
        NetworkResponse {
            headers: rate_limit_headers,
            body: b"{\"models\":[]}".to_vec(),
        },
    );
    let network = Rc::new(TestNetwork { responses });
    let time = Rc::new(TestTime {
        now: time::macros::datetime!(2026-05-03 10:05:30 UTC),
        sleep_called: std::cell::Cell::new(false),
    });
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time: time.clone(),
    };
    setup_config(&fs);

    // When mirroring activities:
    let args = vec!["strava-mirror".to_string()];
    run(args, &ctx).unwrap();

    // Then make sure sleep was called:
    assert!(time.sleep_called.get());
}

#[test]
fn test_run_full_history() {
    // Given one activity mirrored already:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();
    let timestamp_str_1 = "2025-04-09T07-44-48Z";
    let base_name_1 = format!("{}_1", timestamp_str_1);
    let meta_path_1 = activities_dir
        .join(format!("{}.meta.json", base_name_1))
        .unwrap();
    let activity1_content = r#"{"id": 1, "start_time": "2025-04-09T07:44:48Z", "sport_type": "Ride", "moving_time_raw": 3600, "elapsed_time_raw": 4000, "distance_raw": 1000.0, "elevation_gain_raw": 100.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(activity1_content.as_bytes())
        .unwrap();
    activities_dir
        .join(format!("{}.fit", base_name_1))
        .unwrap()
        .create_file()
        .unwrap();
    let mut responses = HashMap::new();
    let token_body = std::fs::read("src/fixtures/token.json").unwrap();
    responses.insert(
        "https://www.strava.com/oauth/token".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: token_body,
        },
    );
    // Note: NO &after= parameter in the URL.
    let activities_url =
        "https://www.strava.com/athlete/training_activities?new_activity_only=false";
    responses.insert(
        activities_url.to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: b"{\"models\":[]}".to_vec(),
        },
    );
    let network = Rc::new(TestNetwork { responses });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };
    setup_config(&fs);

    // When mirroring activities with --full-history:
    let args = vec!["strava-mirror".to_string(), "--full-history".to_string()];
    run(args, &ctx).unwrap();

    // Then no panic occurs (meaning the URL matched the one without &after=).
}

#[test]
fn test_mirror_activity_full_history_change() {
    // Given an activity mirrored already, but with a different name:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();
    let timestamp_str_1 = "2025-04-09T07-44-48Z";
    let base_name_1 = format!("{}_1", timestamp_str_1);
    let meta_path_1 = activities_dir
        .join(format!("{}.meta.json", base_name_1))
        .unwrap();
    // Local name is "old name".
    let activity1_content = r#"{"id": 1, "name": "old name", "start_time": "2025-04-09T07:44:48Z", "sport_type": "Ride", "moving_time_raw": 3600, "elapsed_time_raw": 4000, "distance_raw": 1000.0, "elevation_gain_raw": 100.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(activity1_content.as_bytes())
        .unwrap();

    let mut responses = HashMap::new();
    let token_body = std::fs::read("src/fixtures/token.json").unwrap();
    responses.insert(
        "https://www.strava.com/oauth/token".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: token_body,
        },
    );
    // Summary name is "new name".
    responses.insert(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: b"{\"models\":[{\"name\": \"new name\", \"id\": 1, \"start_time\": \"2025-04-09T07:44:48Z\", \"sport_type\": \"Ride\", \"moving_time_raw\": 3600, \"elapsed_time_raw\": 4000, \"distance_raw\": 1000.0, \"elevation_gain_raw\": 100.0}]}"
                .to_vec(),
        },
    );
    // This is the re-download request:
    responses.insert(
        "https://www.strava.com/api/v3/activities/1".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: b"{\"name\": \"new name\"}".to_vec(),
        },
    );
    let mut data_headers = HashMap::new();
    data_headers.insert(
        "content-disposition".to_string(),
        "attachment; filename=\"activity.fit\"".to_string(),
    );
    responses.insert(
        "https://www.strava.com/activities/1/export_original".to_string(),
        NetworkResponse {
            headers: data_headers,
            body: b"fitdata".to_vec(),
        },
    );

    let network = Rc::new(TestNetwork { responses });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };
    setup_config(&fs);

    // When mirroring activities with --full-history:
    let args = vec!["strava-mirror".to_string(), "--full-history".to_string()];
    run(args, &ctx).unwrap();

    // Then the local file is updated:
    let mut updated_content = String::new();
    meta_path_1
        .open_file()
        .unwrap()
        .read_to_string(&mut updated_content)
        .unwrap();
    assert!(updated_content.contains("new name"));
}

#[test]
fn test_query_custom() {
    // Given an activity mirrored already:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();
    let timestamp_str_1 = "2025-04-09T07-44-48Z";
    let base_name_1 = format!("{}_1", timestamp_str_1);
    let meta_path_1 = activities_dir
        .join(format!("{}.meta.json", base_name_1))
        .unwrap();
    let activity1_content = r#"{"id": 1, "name": "myactivity", "start_time": "2025-04-09T07:44:48Z", "start_latlng": [47.0, 19.0], "sport_type": "Ride", "moving_time_raw": 3600, "elapsed_time_raw": 4000, "distance_raw": 1000.0, "elevation_gain_raw": 100.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(activity1_content.as_bytes())
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

    // When querying custom:
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "custom".to_string(),
    ];
    run(args, &ctx).unwrap();

    // Then no failure occurs and output was printed (stdout is not captured here, but run() returns Ok).
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
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "top-walks-by-time".to_string(),
    ];
    run(args, &ctx).unwrap();

    // Then no failure occurs.
}

#[test]
fn test_query_top_walks_by_distance() {
    // Given three activities (2 walks, 1 ride):
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();

    // 1. Long walk (by time, short by distance)
    let meta_path_1 = activities_dir
        .join("2025-01-01T10-00-00Z_1.meta.json")
        .unwrap();
    let content_1 = r#"{"id": 1, "name": "long time walk", "start_time": "2025-01-01T10:00:00Z", "sport_type": "Walk", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 5000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(content_1.as_bytes())
        .unwrap();

    // 2. Short walk (by time, long by distance)
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
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "top-walks-by-distance".to_string(),
    ];
    run(args, &ctx).unwrap();

    // Then no failure occurs.
}

#[test]
fn test_query_top_walks_by_elevation() {
    // Given three activities (2 walks, 1 ride):
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();

    // 1. Walk with high elevation
    let meta_path_1 = activities_dir
        .join("2025-01-01T10-00-00Z_1.meta.json")
        .unwrap();
    let content_1 = r#"{"id": 1, "name": "mountain walk", "start_time": "2025-01-01T10:00:00Z", "sport_type": "Walk", "moving_time_raw": 5000, "elapsed_time_raw": 5400, "distance_raw": 5000.0, "elevation_gain_raw": 1000.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(content_1.as_bytes())
        .unwrap();

    // 2. Walk with low elevation
    let meta_path_2 = activities_dir
        .join("2025-01-02T10-00-00Z_2.meta.json")
        .unwrap();
    let content_2 = r#"{"id": 2, "name": "flat walk", "start_time": "2025-01-02T10:00:00Z", "sport_type": "Walk", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 10000.0, "elevation_gain_raw": 10.0}"#;
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
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "top-walks-by-elevation".to_string(),
    ];
    run(args, &ctx).unwrap();

    // Then no failure occurs.
}

#[test]
fn test_query_top_rides_by_time() {
    // Given three activities (1 walk, 2 rides):
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();

    // 1. Long ride
    let meta_path_1 = activities_dir
        .join("2025-01-01T10-00-00Z_1.meta.json")
        .unwrap();
    let content_1 = r#"{"id": 1, "name": "long ride", "start_time": "2025-01-01T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 50000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(content_1.as_bytes())
        .unwrap();

    // 2. Short ride
    let meta_path_2 = activities_dir
        .join("2025-01-02T10-00-00Z_2.meta.json")
        .unwrap();
    let content_2 = r#"{"id": 2, "name": "short ride", "start_time": "2025-01-02T10:00:00Z", "sport_type": "Ride", "moving_time_raw": 5000, "elapsed_time_raw": 5400, "distance_raw": 25000.0, "elevation_gain_raw": 200.0}"#;
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
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "top-rides-by-time".to_string(),
    ];
    run(args, &ctx).unwrap();

    // Then no failure occurs.
}

#[test]
fn test_query_top_rides_by_distance() {
    // Given three activities (1 walk, 2 rides):
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
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "top-rides-by-distance".to_string(),
    ];
    run(args, &ctx).unwrap();

    // Then no failure occurs.
}

#[test]
fn test_query_top_rides_by_elevation() {
    // Given three activities (1 walk, 2 rides):
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
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "top-rides-by-elevation".to_string(),
    ];
    run(args, &ctx).unwrap();

    // Then no failure occurs.
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
    let content_5 = r#"{"id": 5, "name": "hungarian walk", "start_time": "2025-01-01T10:00:00Z", "start_latlng": [47.0, 19.0], "sport_type": "Walk", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 10000.0, "elevation_gain_raw": 500.0}"#;
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
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "longest-rides-by-year".to_string(),
    ];
    run(args, &ctx).unwrap();

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
    let content_1 = r#"{"id": 1, "name": "hungarian walk", "start_time": "2025-01-01T10:00:00Z", "start_latlng": [47.0, 19.0], "sport_type": "Walk", "moving_time_raw": 10000, "elapsed_time_raw": 10400, "distance_raw": 10000.0, "elevation_gain_raw": 500.0}"#;
    meta_path_1
        .create_file()
        .unwrap()
        .write_all(content_1.as_bytes())
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
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        process: Rc::new(TestProcess::new(&[])),
        time,
    };
    setup_config(&fs);

    // When querying all:
    let args = vec![
        "strava-mirror".to_string(),
        "--query".to_string(),
        "all".to_string(),
    ];
    run(args, &ctx).unwrap();

    // Then no failure occurs.
}

#[test]
fn test_format_duration() {
    assert_eq!(format_duration(3600), "1:00:00");
    assert_eq!(format_duration(16864), "4:41:04");
    assert_eq!(format_duration(59), "0:00:59");
}

#[test]
fn test_format_distance() {
    assert_eq!(format_distance(15962.8), "15.96 km");
    assert_eq!(format_distance(1000.0), "1.00 km");
    assert_eq!(format_distance(50.0), "0.05 km");
}

#[test]
fn test_format_elevation() {
    assert_eq!(format_elevation(1038.1), "1038 m");
    assert_eq!(format_elevation(100.9), "101 m");
    assert_eq!(format_elevation(0.0), "0 m");
}

#[test]
fn test_should_redownload_meta() {
    let now = time::macros::datetime!(2025-04-09 07:44:48 UTC);
    let mut metadata = ActivityMetadata {
        id: 1,
        name: Some("old name".to_string()),
        start_time: now,
        sport_type: "Ride".to_string(),
        moving_time_raw: 3600,
        elapsed_time_raw: 4000,
        distance_raw: 1000.0,
        elevation_gain_raw: 100.0,
    };
    let mut summary = ActivitiesItemResponse {
        id: 1,
        name: "old name".to_string(),
        start_time: now,
        sport_type: "Ride".to_string(),
        moving_time_raw: 3600,
        elapsed_time_raw: 4000,
        distance_raw: 1000.0,
        elevation_gain_raw: 100.0,
    };

    // No change
    assert!(!should_redownload_meta(&metadata, &summary));

    // Name change
    summary.name = "new name".to_string();
    assert!(should_redownload_meta(&metadata, &summary));
    summary.name = "old name".to_string();

    // Sport type change
    summary.sport_type = "Run".to_string();
    assert!(should_redownload_meta(&metadata, &summary));

    // Metadata name is None (e.g. from a partial local file)
    metadata.name = None;
    assert!(should_redownload_meta(&metadata, &summary));
}
