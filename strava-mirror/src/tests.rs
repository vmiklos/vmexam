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

pub(crate) struct TestNetwork {
    pub(crate) responses: HashMap<String, NetworkResponse>,
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

pub(crate) struct TestProcess {
    /// Maps a joined cmdline to its output.
    outputs: HashMap<String, String>,
}

impl TestProcess {
    pub(crate) fn new(outputs: &[(&str, &str)]) -> Self {
        let outputs = outputs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        TestProcess { outputs }
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
        self.outputs
            .get(&cmdline)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("no mock output for: {}", cmdline))
    }
}

pub(crate) struct TestTime {
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

pub(crate) fn setup_config(fs: &vfs::VfsPath) {
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

/// Builds the gpsbabel cmdline get_activity_lat_lon() runs for a given activity base name.
pub(crate) fn gpsbabel_cmdline(base_name: &str) -> String {
    let home_dir = home::home_dir().unwrap();
    format!(
        "-i garmin_fit -f {}/.local/share/strava-mirror/activities/2025/{}.fit -o geojson -F -",
        home_dir.to_str().unwrap(),
        base_name
    )
}

#[test]
fn test_no_activities() {
    // Given no activities:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let mut responses = HashMap::new();
    responses.insert(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false&page=1"
            .to_string(),
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
    let responses = HashMap::new();
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
    let responses = HashMap::new();
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
    let activities_1_body = std::fs::read("src/fixtures/activities-1.json").unwrap();
    responses.insert(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false&page=1"
            .to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: activities_1_body,
        },
    );
    let activities_0_body = std::fs::read("src/fixtures/activities-0.json").unwrap();
    responses.insert(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false&page=2"
            .to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: activities_0_body,
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
    let activity1_content = r#"{"id": 1, "name": "activity1", "start_time": "2025-04-09T07:44:48Z", "sport_type": "Ride", "moving_time_raw": 3600, "elapsed_time_raw": 4000, "distance_raw": 1000.0, "elevation_gain_raw": 100.0}"#;
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
    let activities_url = format!(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false&page=1",
    );
    let activities_body = std::fs::read("src/fixtures/activities-2.json").unwrap();
    responses.insert(
        activities_url,
        NetworkResponse {
            headers: HashMap::new(),
            body: activities_body,
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
    let meta_content = r#"{"id": 1, "name": "myactivity", "start_time": "2025-04-09T07:44:48Z", "sport_type": "Ride", "moving_time_raw": 3600, "elapsed_time_raw": 4000, "distance_raw": 1000.0, "elevation_gain_raw": 100.0}"#;
    meta_path
        .create_file()
        .unwrap()
        .write_all(meta_content.as_bytes())
        .unwrap();

    let mut responses = HashMap::new();
    let activities_1_body = std::fs::read("src/fixtures/activities-1.json").unwrap();
    responses.insert(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false&page=1"
            .to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: activities_1_body,
        },
    );
    let activities_0_body = std::fs::read("src/fixtures/activities-0.json").unwrap();
    responses.insert(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false&page=2"
            .to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: activities_0_body,
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
fn test_mirror_activity_skip_data() {
    // Given an activity where the data is already mirrored, but the meta is not:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let activities_dir = fs
        .join(".local/share/strava-mirror/activities/2025")
        .unwrap();
    activities_dir.create_dir_all().unwrap();
    let timestamp_str = "2025-04-09T07-44-48Z";
    let base_name = format!("{}_1", timestamp_str);
    let data_path = activities_dir.join(format!("{}.fit", base_name)).unwrap();
    data_path
        .create_file()
        .unwrap()
        .write_all(b"fitdata")
        .unwrap();

    let mut responses = HashMap::new();
    let activities_1_body = std::fs::read("src/fixtures/activities-1.json").unwrap();
    responses.insert(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false&page=1"
            .to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: activities_1_body,
        },
    );
    let activities_0_body = std::fs::read("src/fixtures/activities-0.json").unwrap();
    responses.insert(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false&page=2"
            .to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: activities_0_body,
        },
    );
    // Notice that export_original is NOT in the responses, so if we try to download the data
    // again, we fail.
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

    // Then make sure the meta file is created (and the data download was skipped):
    assert!(
        activities_dir
            .join(format!("{}.meta.json", base_name))
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
    let meta_content = r#"{"id": 1, "name": "myactivity", "start_time": "2025-04-09T07:44:48Z", "sport_type": "Ride", "moving_time_raw": 3600, "elapsed_time_raw": 4000, "distance_raw": 1000.0, "elevation_gain_raw": 100.0}"#;
    meta_path
        .create_file()
        .unwrap()
        .write_all(meta_content.as_bytes())
        .unwrap();
    activities_dir
        .join(format!("{}.fit", base_name))
        .unwrap()
        .create_file()
        .unwrap();

    let mut responses = HashMap::new();
    let activities_url = format!(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false&page=1",
    );
    let activities_body = std::fs::read("src/fixtures/activities-1.json").unwrap();
    responses.insert(
        activities_url,
        NetworkResponse {
            headers: HashMap::new(),
            body: activities_body,
        },
    );
    // Notice that export_original is not in the responses.
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

    // 2. Missing .fit file: get_activity_lat_lon() fails, so the activity is skipped.
    let metadata = ActivityMetadata {
        id: 1,
        name: "no fit".to_string(),
        start_time: time::macros::datetime!(2025-01-01 10:00:00 UTC),
        sport_type: "Ride".to_string(),
        moving_time_raw: 3600,
        elapsed_time_raw: 4000,
        distance_raw: 1000.0,
        elevation_gain_raw: 100.0,
    };
    let mut cache = HashMap::new();
    let ret = get_activity_country(
        &ctx,
        "2025/2025-01-01T10-00-00Z_1.meta.json",
        metadata,
        &mut cache,
    )
    .unwrap();
    assert!(ret.is_none());
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
    responses.insert(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false&page=1"
            .to_string(),
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
fn test_rate_limit_sleep() {
    // Given a rate limit configured:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let mut responses = HashMap::new();
    let headers = HashMap::new();
    responses.insert(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false&page=1"
            .to_string(),
        NetworkResponse {
            headers,
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
    let activity1_content = r#"{"id": 1, "name": "activity1", "start_time": "2025-04-09T07:44:48Z", "sport_type": "Ride", "moving_time_raw": 3600, "elapsed_time_raw": 4000, "distance_raw": 1000.0, "elevation_gain_raw": 100.0}"#;
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
    // Note: NO &after= parameter in the URL.
    let activities_url =
        "https://www.strava.com/athlete/training_activities?new_activity_only=false&page=1";
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
    // Summary name is "new name".
    responses.insert(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false&page=1".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: b"{\"models\":[{\"name\": \"new name\", \"id\": 1, \"start_time\": \"2025-04-09T07:44:48Z\", \"sport_type\": \"Ride\", \"moving_time_raw\": 3600, \"elapsed_time_raw\": 4000, \"distance_raw\": 1000.0, \"elevation_gain_raw\": 100.0}]}"
                .to_vec(),
        },
    );
    let activities_0_body = std::fs::read("src/fixtures/activities-0.json").unwrap();
    responses.insert(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false&page=2"
            .to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: activities_0_body,
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
fn test_should_redownload_meta() {
    let now = time::macros::datetime!(2025-04-09 07:44:48 UTC);
    let metadata = ActivityMetadata {
        id: 1,
        name: "old name".to_string(),
        start_time: now,
        sport_type: "Ride".to_string(),
        moving_time_raw: 3600,
        elapsed_time_raw: 4000,
        distance_raw: 1000.0,
        elevation_gain_raw: 100.0,
    };
    let mut summary = ActivityMetadata {
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
}
