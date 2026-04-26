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
    fn get(
        &self,
        url: &str,
        _headers: &HashMap<String, String>,
    ) -> anyhow::Result<NetworkResponse> {
        // For now we have no case when we want to simulate a GET failing.
        let response = self.responses.get(url).unwrap();
        return Ok(NetworkResponse {
            headers: response.headers.clone(),
            body: response.body.clone(),
        });
    }

    fn post(&self, url: &str, _body: &str) -> anyhow::Result<NetworkResponse> {
        if let Some(response) = self.responses.get(url) {
            return Ok(NetworkResponse {
                headers: response.headers.clone(),
                body: response.body.clone(),
            });
        }
        Err(anyhow::anyhow!("Unexpected POST request to {}", url))
    }
}

struct TestTime {
    now: time::OffsetDateTime,
}

impl Default for TestTime {
    fn default() -> Self {
        Self {
            now: time::macros::datetime!(2026-04-12 12:00:00 UTC),
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

    fn sleep(&self, _duration: std::time::Duration) {}
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
        "https://www.strava.com/api/v3/athlete/activities?page=1&per_page=200".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: b"[]".to_vec(),
        },
    );
    let network = Rc::new(TestNetwork { responses });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
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
fn test_get_access_token_error() {
    // Given the oauth token request fails:
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let responses = HashMap::new();
    let network = Rc::new(TestNetwork { responses });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        time,
    };
    setup_config(&fs);

    // When mirroring activities:
    let args = vec!["strava-mirror".to_string()];
    let ret = run(args, &ctx);

    // Then make sure there is a failure:
    assert!(ret.is_err());
    let err = ret.unwrap_err().to_string();
    assert_eq!(
        err,
        "Unexpected POST request to https://www.strava.com/oauth/token"
    );
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
        time,
    };
    let config_dir = fs.join(".config").unwrap();
    config_dir.create_dir_all().unwrap();
    let config_content = r#"client_id = "42"
client_secret = "s"
refresh_token = "r"
jwt = "invalid""#;
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
    let time = Rc::new(TestTime { now });
    let ctx = Context {
        fs: fs.clone(),
        network,
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
        "https://www.strava.com/api/v3/athlete/activities?page=1&per_page=200".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: activities_body,
        },
    );
    responses.insert(
        "https://www.strava.com/api/v3/athlete/activities?page=2&per_page=200".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: b"[]".to_vec(),
        },
    );
    let activity_meta_body = std::fs::read("src/fixtures/activity1.json").unwrap();
    responses.insert(
        "https://www.strava.com/api/v3/activities/1".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: activity_meta_body,
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
    let activity1_content = r#"{"id": 1, "start_date": "2025-04-09T07:44:48Z"}"#;
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
    let after_ts = time::macros::datetime!(2025-04-09 07:44:48 UTC).unix_timestamp();
    let activities_url = format!(
        "https://www.strava.com/api/v3/athlete/activities?page=1&per_page=200&after={}",
        after_ts
    );
    let activities_body = std::fs::read("src/fixtures/activities-2.json").unwrap();
    responses.insert(
        activities_url,
        NetworkResponse {
            headers: HashMap::new(),
            body: activities_body,
        },
    );
    let activities_url_p2 = format!(
        "https://www.strava.com/api/v3/athlete/activities?page=2&per_page=200&after={}",
        after_ts
    );
    responses.insert(
        activities_url_p2,
        NetworkResponse {
            headers: HashMap::new(),
            body: b"[]".to_vec(),
        },
    );
    responses.insert(
        "https://www.strava.com/api/v3/activities/2".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: b"{\"id\": 2, \"name\": \"myactivity2\"}".to_vec(),
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
        "https://www.strava.com/api/v3/athlete/activities?page=1&per_page=200".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: activities_body,
        },
    );
    responses.insert(
        "https://www.strava.com/api/v3/athlete/activities?page=2&per_page=200".to_string(),
        NetworkResponse {
            headers: HashMap::new(),
            body: b"[]".to_vec(),
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
    let after_ts = 1744184688;
    let activities_url = format!(
        "https://www.strava.com/api/v3/athlete/activities?page=1&per_page=200&after={}",
        after_ts
    );
    let activities_body = std::fs::read("src/fixtures/activities-1.json").unwrap();
    responses.insert(
        activities_url,
        NetworkResponse {
            headers: HashMap::new(),
            body: activities_body,
        },
    );
    let activities_url_p2 = format!(
        "https://www.strava.com/api/v3/athlete/activities?page=2&per_page=200&after={}",
        after_ts
    );
    responses.insert(
        activities_url_p2,
        NetworkResponse {
            headers: HashMap::new(),
            body: b"[]".to_vec(),
        },
    );
    // Notice that neither api/v3/activities/1 nor export_original is in the responses.
    let network = Rc::new(TestNetwork { responses });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
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
        .write_all(b"{\"start_latlng\": [47.0, 19.0]}")
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
        .join(".local/share/strava-mirror/nominatim-cache.json")
        .unwrap();
    assert!(cache_path.exists().unwrap());
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
        .write_all(b"{\"start_latlng\": [47.0, 19.0]}")
        .unwrap();

    // Activity 2 in Austria (48.0, 16.0)
    let meta_path2 = activities_dir
        .join(format!("{}_2.meta.json", timestamp_str))
        .unwrap();
    meta_path2
        .create_file()
        .unwrap()
        .write_all(b"{\"start_latlng\": [48.0, 16.0]}")
        .unwrap();

    // Pre-existing cache for Hungary
    let cache_path = fs
        .join(".local/share/strava-mirror/nominatim-cache.json")
        .unwrap();
    cache_path.parent().create_dir_all().unwrap();
    cache_path
        .create_file()
        .unwrap()
        .write_all(b"{\"lat=47&lon=19\": \"Hungary\"}")
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
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
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
        time,
    };

    let countries = get_countries(&ctx).unwrap();
    assert!(countries.is_empty());
}

#[test]
fn test_get_activity_country_special_cases() {
    let fs = vfs::VfsPath::new(vfs::MemoryFS::new());
    let year_dir = fs.join("2025").unwrap();
    year_dir.create_dir_all().unwrap();
    let network = Rc::new(TestNetwork {
        responses: HashMap::new(),
    });
    let time = Rc::new(TestTime::default());
    let ctx = Context {
        fs: fs.clone(),
        network,
        time,
    };
    let mut cache = HashMap::new();

    // 1. File without .meta.json suffix
    let fit_path = year_dir.join("activity.fit").unwrap();
    fit_path.create_file().unwrap();
    let ret = get_activity_country(&ctx, &fit_path, &mut cache).unwrap();
    assert!(ret.is_none());

    // 2. .meta.json without start_latlng
    let meta_no_latlng = year_dir.join("no_latlng.meta.json").unwrap();
    meta_no_latlng
        .create_file()
        .unwrap()
        .write_all(b"{}")
        .unwrap();
    let ret = get_activity_country(&ctx, &meta_no_latlng, &mut cache).unwrap();
    assert!(ret.is_none());

    // 3. .meta.json with empty start_latlng
    let meta_empty_latlng = year_dir.join("empty_latlng.meta.json").unwrap();
    meta_empty_latlng
        .create_file()
        .unwrap()
        .write_all(b"{\"start_latlng\": []}")
        .unwrap();
    let ret = get_activity_country(&ctx, &meta_empty_latlng, &mut cache).unwrap();
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
        time,
    };

    let countries = get_countries(&ctx).unwrap();
    assert!(countries.is_empty());
}
