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
        if let Some(response) = self.responses.get(url) {
            return Ok(NetworkResponse {
                status_code: response.status_code,
                headers: response.headers.clone(),
                body: response.body.clone(),
            });
        }
        Err(anyhow::anyhow!("Unexpected GET request to {}", url))
    }

    fn post(&self, url: &str, _body: &str) -> anyhow::Result<NetworkResponse> {
        if let Some(response) = self.responses.get(url) {
            return Ok(NetworkResponse {
                status_code: response.status_code,
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
            status_code: 200,
            headers: HashMap::new(),
            body: token_body,
        },
    );
    responses.insert(
        "https://www.strava.com/api/v3/athlete/activities?page=1&per_page=200".to_string(),
        NetworkResponse {
            status_code: 200,
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
    let mut responses = HashMap::new();
    responses.insert(
        "https://www.strava.com/oauth/token".to_string(),
        NetworkResponse {
            status_code: 500,
            headers: HashMap::new(),
            body: b"".to_vec(),
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
    let config_content = std::fs::read_to_string("src/fixtures/strava-mirrorrc").unwrap();
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
    assert!(err.contains("status is not success: 500"));
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
            status_code: 200,
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
            status_code: 200,
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

    // When mirroring activities:
    let args = vec!["strava-mirror".to_string()];
    let ret = run(args, &ctx);

    // Then make sure there is a failure:
    assert!(ret.is_err());
    let err = ret.unwrap_err().to_string();
    assert!(err.contains("JWT has expired"));
}
