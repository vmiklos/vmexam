/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Mirrors your Strava activities.

use anyhow::Context as _;
use isahc::ReadResponseExt as _;
use isahc::RequestExt as _;

/// Contents of the config file.
#[derive(serde::Deserialize)]
struct Config {
    client_id: String,
    client_secret: String,
    refresh_token: String,
}

/// Reads the config file.
fn read_config(home: &std::path::Path) -> anyhow::Result<Config> {
    let config_path = home.join(".config").join("strava-mirrorrc");
    let config_content = std::fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

/// Response for a /oauth/token request.
#[derive(serde::Deserialize)]
struct TokenResponse {
    access_token: String,
}

/// Gets an access token from a refresh token.
fn get_access_token(config: &Config) -> anyhow::Result<String> {
    let url = "https://www.strava.com/oauth/token";
    let params = [
        ("client_id", &config.client_id),
        ("client_secret", &config.client_secret),
        ("refresh_token", &config.refresh_token),
        ("grant_type", &"refresh_token".to_string()),
    ];

    let mut response = isahc::post(url, serde_urlencoded::to_string(params)?)?;
    let status = response.status();
    if !status.is_success() {
        return Err(anyhow::anyhow!("status is not success: {status}"));
    }

    let token_response: TokenResponse = response.json()?;
    Ok(token_response.access_token)
}

/// One item in the /api/v3/athlete/activities response list.
#[derive(serde::Deserialize, serde::Serialize)]
struct ActivitySummary {
    id: u64,
    #[serde(with = "time::serde::rfc3339")]
    start_date: time::OffsetDateTime,
}

/// Lists activities: only minimal info that is cheap even for all activities.
fn list_activities(access_token: &str, page: u32) -> anyhow::Result<Vec<ActivitySummary>> {
    let url = format!(
        "https://www.strava.com/api/v3/athlete/activities?page={}&per_page=200",
        page
    );
    let mut response = isahc::Request::get(url)
        .header("Authorization", format!("Bearer {}", access_token))
        .body(())?
        .send()?;
    let status = response.status();
    if !status.is_success() {
        return Err(anyhow::anyhow!("status is not success: {status}"));
    }

    let activities: Vec<ActivitySummary> = response.json()?;
    Ok(activities)
}

/// Mirrors one activity if needed.
fn mirror_activity(
    access_token: &str,
    summary: &ActivitySummary,
    activities_dir: &std::path::Path,
) -> anyhow::Result<()> {
    let year = summary.start_date.year();
    let format = time::format_description::parse("[year]-[month]-[day]T[hour]-[minute]-[second]Z")?;
    let timestamp = summary.start_date.format(&format)?;
    let id = summary.id;
    let base_name = format!("{}_{}", timestamp, id);
    let year_dir = activities_dir.join(year.to_string());
    std::fs::create_dir_all(&year_dir)?;

    let meta_path = year_dir.join(format!("{}.meta.json", base_name));
    if !meta_path.exists() {
        let url = format!("https://www.strava.com/api/v3/activities/{}", id);
        let mut response = isahc::Request::get(url)
            .header("Authorization", format!("Bearer {}", access_token))
            .body(())?
            .send()?;

        let status = response.status();
        if !status.is_success() {
            return Err(anyhow::anyhow!("status is not success: {status}"));
        }

        let activity_json: serde_json::Value = response.json()?;
        std::fs::write(&meta_path, serde_json::to_string_pretty(&activity_json)?)?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let home = home::home_dir().context("home_dir() failed")?;

    let config = read_config(&home)?;
    let access_token = get_access_token(&config)?;

    let activities_dir = home
        .join(".local")
        .join("share")
        .join("strava-mirror")
        .join("activities");

    let mut page = 1;
    loop {
        let activities: Vec<ActivitySummary> = list_activities(&access_token, page)?;
        if activities.is_empty() {
            break;
        }

        for activity in activities {
            mirror_activity(&access_token, &activity, &activities_dir)?;
        }

        page += 1;
    }

    Ok(())
}
