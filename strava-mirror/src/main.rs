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
use base64::Engine as _;
use clap::Parser as _;
use isahc::ReadResponseExt as _;
use isahc::RequestExt as _;
use log::info;

/// Contents of the config file.
#[derive(serde::Deserialize)]
struct Config {
    client_id: String,
    client_secret: String,
    refresh_token: String,
    jwt: String,
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

    info!("POST '{}'", url);
    let mut response = isahc::post(url, serde_urlencoded::to_string(params)?)?;
    let status = response.status();
    if !status.is_success() {
        return Err(anyhow::anyhow!("status is not success: {status}"));
    }

    let token_response: TokenResponse = response.json()?;
    Ok(token_response.access_token)
}

/// Contents of the JWT payload.
#[derive(serde::Deserialize)]
struct Jwt {
    sub: i64,
    exp: i64,
}

/// Parses the JWT to get a Cookie header value.
fn jwt_to_cookie(jwt: &str) -> anyhow::Result<String> {
    let parts: Vec<&str> = jwt.split('.').collect();
    if parts.len() != 3 {
        // Expected 'header.payload.signature'.
        return Err(anyhow::anyhow!("JWT doesn't have 3 parts"));
    }
    let payload_encoded = parts[1];
    let payload_bytes = base64::prelude::BASE64_URL_SAFE_NO_PAD.decode(payload_encoded)?;
    let jwt_payload: Jwt = serde_json::from_slice(&payload_bytes)?;
    let strava_remember_id = jwt_payload.sub;
    let exp_datetime = time::OffsetDateTime::from_unix_timestamp(jwt_payload.exp)?;
    let local_offset = time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);
    let exp_formatted =
        exp_datetime
            .to_offset(local_offset)
            .format(time::macros::format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second]"
            ))?;
    info!("JWT expires at {}", exp_formatted);
    let now = time::OffsetDateTime::now_utc();
    if exp_datetime <= now {
        return Err(anyhow::anyhow!("JWT has expired"));
    }
    Ok(format!(
        "strava_remember_id={}; strava_remember_token={}",
        strava_remember_id, jwt
    ))
}

/// One item in the /api/v3/athlete/activities response list.
#[derive(serde::Deserialize, serde::Serialize)]
struct ActivitySummary {
    id: u64,
    name: String,
    #[serde(with = "time::serde::rfc3339")]
    start_date: time::OffsetDateTime,
}

/// Lists activities: only minimal info that is cheap even for all activities.
fn list_activities(access_token: &str, page: u32) -> anyhow::Result<Vec<ActivitySummary>> {
    let url = format!(
        "https://www.strava.com/api/v3/athlete/activities?page={}&per_page=200",
        page
    );
    info!("GET '{}'", url);
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

/// Mirrors the original data of one activity.
fn mirror_activity_data(
    id: u64,
    base_name: &str,
    year_dir: &std::path::Path,
    cookie: &str,
) -> anyhow::Result<()> {
    let url = format!("https://www.strava.com/activities/{}/export_original", id);
    info!("GET '{}'", url);
    let mut response = isahc::Request::get(url)
        .header("Cookie", cookie)
        .body(())?
        .send()?;
    let status = response.status();
    if !status.is_success() {
        return Err(anyhow::anyhow!("status is not success: {status}"));
    }
    let content_disposition = response
        .headers()
        .get("content-disposition")
        .context("missing content-disposition header")?
        .to_str()?;
    let filename = content_disposition
        .split("; ")
        .find(|item| item.starts_with("filename="))
        .context("failed to find filename in content-disposition")?
        .strip_prefix("filename=")
        .context("failed to strip filename= prefix")?
        .trim_matches('"');
    let extension = filename.split('.').next_back().context("no extension")?;
    let path = year_dir.join(format!("{}.{}", base_name, extension));
    let body = response.bytes()?;
    std::fs::write(&path, body)?;
    Ok(())
}

/// Mirrors one activity if needed.
fn mirror_activity(
    access_token: &str,
    summary: &ActivitySummary,
    activities_dir: &std::path::Path,
    cookie: &str,
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
        info!("GET '{}', name is '{}'", url, summary.name);
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

        // Also download the actual activity. We don't really know what'll be the filename of this
        // till we download it, so assume that .meta.json and .fit are downloaded together.
        mirror_activity_data(id, &base_name, &year_dir, cookie)?;
    }

    Ok(())
}

/// Sets up logging so it has local time timestamp as a prefix.
fn setup_logging(level: log::LevelFilter) -> anyhow::Result<()> {
    let mut builder = simplelog::ConfigBuilder::new();
    builder.set_time_format_custom(simplelog::format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second]"
    ));
    if builder.set_time_offset_to_local().is_err() {
        return Err(anyhow::anyhow!("offset to local failed"));
    }
    let config = builder.build();
    simplelog::CombinedLogger::init(vec![simplelog::TermLogger::new(
        level,
        config,
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Never,
    )])?;
    Ok(())
}

#[derive(clap::Parser)]
struct Args {
    #[arg(short, long)]
    quiet: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let log_level = if args.quiet {
        log::LevelFilter::Error
    } else {
        log::LevelFilter::Info
    };
    setup_logging(log_level)?;

    let home = home::home_dir().context("home_dir() failed")?;

    let config = read_config(&home)?;
    let access_token = get_access_token(&config)?;

    let activities_dir = home
        .join(".local")
        .join("share")
        .join("strava-mirror")
        .join("activities");

    let cookie = jwt_to_cookie(&config.jwt)?;
    let mut page = 1;
    loop {
        let activities: Vec<ActivitySummary> = list_activities(&access_token, page)?;
        if activities.is_empty() {
            break;
        }

        for activity in activities {
            mirror_activity(&access_token, &activity, &activities_dir, &cookie)?;
        }

        page += 1;
    }

    Ok(())
}
