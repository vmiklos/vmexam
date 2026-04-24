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
use std::collections::HashMap;
use std::io::Read as _;
use std::io::Write as _;
use std::rc::Rc;

#[cfg(not(test))]
use log::info;

#[cfg(test)]
use std::println as info;

const ACTIVITY_TIMESTAMP_FORMAT: &str = "[year]-[month]-[day]T[hour]-[minute]-[second]Z";

/// Network response.
pub struct NetworkResponse {
    /// The status code.
    pub status_code: u16,
    /// The headers.
    pub headers: HashMap<String, String>,
    /// The body.
    pub body: Vec<u8>,
}

/// Network interface.
pub trait Network {
    /// GET request.
    fn get(&self, url: &str, headers: &HashMap<String, String>) -> anyhow::Result<NetworkResponse>;
    /// POST request.
    fn post(&self, url: &str, body: &str) -> anyhow::Result<NetworkResponse>;
}

/// Time interface.
pub trait Time {
    /// Returns the current time in local time.
    fn now(&self) -> time::OffsetDateTime;
    /// Converts a Unix timestamp to local time.
    fn to_local_offset(&self, timestamp: i64) -> anyhow::Result<time::OffsetDateTime>;
}

/// The context of the application.
pub struct Context {
    /// The filesystem to use.
    pub fs: vfs::VfsPath,
    /// The network to use.
    pub network: Rc<dyn Network>,
    /// The time to use.
    pub time: Rc<dyn Time>,
}

/// Contents of the config file.
#[derive(serde::Deserialize)]
struct Config {
    client_id: String,
    client_secret: String,
    refresh_token: String,
    jwt: String,
}

/// Reads the config file.
fn read_config(ctx: &Context) -> anyhow::Result<Config> {
    let config_path = ctx.fs.join(".config/strava-mirrorrc")?;
    let mut config_content = String::new();
    config_path
        .open_file()?
        .read_to_string(&mut config_content)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

/// Response for a /oauth/token request.
#[derive(serde::Deserialize)]
struct TokenResponse {
    access_token: String,
}

/// Gets an access token from a refresh token.
fn get_access_token(ctx: &Context, config: &Config) -> anyhow::Result<String> {
    let url = "https://www.strava.com/oauth/token";
    let params = [
        ("client_id", &config.client_id),
        ("client_secret", &config.client_secret),
        ("refresh_token", &config.refresh_token),
        ("grant_type", &"refresh_token".to_string()),
    ];

    info!("POST '{}'", url);
    let response = ctx
        .network
        .post(url, &serde_urlencoded::to_string(params)?)?;
    if response.status_code != 200 {
        return Err(anyhow::anyhow!(
            "status is not success: {}",
            response.status_code
        ));
    }

    let token_response: TokenResponse = serde_json::from_slice(&response.body)?;
    Ok(token_response.access_token)
}

/// Contents of the JWT payload.
#[derive(serde::Deserialize)]
struct Jwt {
    sub: i64,
    exp: i64,
}

/// Parses the JWT to get a Cookie header value.
fn jwt_to_cookie(ctx: &Context, jwt: &str) -> anyhow::Result<String> {
    let parts: Vec<&str> = jwt.split('.').collect();
    if parts.len() != 3 {
        // Expected 'header.payload.signature'.
        return Err(anyhow::anyhow!("JWT doesn't have 3 parts"));
    }
    let payload_encoded = parts[1];
    let payload_bytes = base64::prelude::BASE64_URL_SAFE_NO_PAD.decode(payload_encoded)?;
    let jwt_payload: Jwt = serde_json::from_slice(&payload_bytes)?;
    let strava_remember_id = jwt_payload.sub;
    let exp_datetime = ctx.time.to_local_offset(jwt_payload.exp)?;
    let exp_formatted = exp_datetime
        .format(time::macros::format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second]"
        ))
        .expect("OffsetDateTime::format() failed");
    info!("JWT expires at {}", exp_formatted);
    let now = ctx.time.now();
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

/// Information about an activity that is already mirrored.
struct MirroredActivity {
    have_meta: bool,
    have_data: bool,
}

/// A map of mirrored activities, keyed by their start date.
type MirroredActivities = HashMap<time::OffsetDateTime, MirroredActivity>;

/// Scans the activities directory for existing .meta.json files.
fn get_mirrored_activities(activities_dir: &vfs::VfsPath) -> anyhow::Result<MirroredActivities> {
    let mut mirrored_activities = HashMap::new();
    if !activities_dir.exists()? {
        return Ok(mirrored_activities);
    }

    let format = time::format_description::parse(ACTIVITY_TIMESTAMP_FORMAT)?;

    for year_dir in activities_dir.read_dir()? {
        if year_dir.is_file()? {
            continue;
        }

        for entry in year_dir.read_dir()? {
            let filename = entry.filename();

            let timestamp_str = filename.split('_').next().context("next() failed")?;

            if let Ok(primitive) = time::PrimitiveDateTime::parse(timestamp_str, &format) {
                let start_date = primitive.assume_utc();
                let mirrored_activity =
                    mirrored_activities
                        .entry(start_date)
                        .or_insert(MirroredActivity {
                            have_meta: false,
                            have_data: false,
                        });
                if filename.ends_with(".meta.json") {
                    mirrored_activity.have_meta = true;
                } else {
                    mirrored_activity.have_data = true;
                }
            }
        }
    }

    Ok(mirrored_activities)
}

/// Lists activities: only minimal info that is cheap even for all activities.
fn list_activities(
    ctx: &Context,
    access_token: &str,
    page: u32,
    after: Option<i64>,
) -> anyhow::Result<Vec<ActivitySummary>> {
    let mut url = format!(
        "https://www.strava.com/api/v3/athlete/activities?page={}&per_page=200",
        page
    );
    if let Some(after) = after {
        url = format!("{}&after={}", url, after);
    }
    info!("GET '{}'", url);
    let mut headers = HashMap::new();
    headers.insert(
        "Authorization".to_string(),
        format!("Bearer {}", access_token),
    );
    let response = ctx.network.get(&url, &headers)?;
    if response.status_code != 200 {
        return Err(anyhow::anyhow!(
            "status is not success: {}",
            response.status_code
        ));
    }

    let activities: Vec<ActivitySummary> = serde_json::from_slice(&response.body)?;
    Ok(activities)
}

/// Mirrors the original data of one activity.
fn mirror_activity_data(
    ctx: &Context,
    id: u64,
    base_name: &str,
    year_dir: &vfs::VfsPath,
    cookie: &str,
) -> anyhow::Result<()> {
    let url = format!("https://www.strava.com/activities/{}/export_original", id);
    info!("GET '{}'", url);
    let mut headers = HashMap::new();
    headers.insert("Cookie".to_string(), cookie.to_string());
    let response = ctx.network.get(&url, &headers)?;
    if response.status_code != 200 {
        return Err(anyhow::anyhow!(
            "status is not success: {}",
            response.status_code
        ));
    }
    let content_disposition = response
        .headers
        .get("content-disposition")
        .context("missing content-disposition header")?;
    let filename = content_disposition
        .split("; ")
        .find(|item| item.starts_with("filename="))
        .context("failed to find filename in content-disposition")?
        .strip_prefix("filename=")
        .context("failed to strip filename= prefix")?
        .trim_matches('"');
    let extension = filename.split('.').next_back().context("no extension")?;
    let path = year_dir.join(format!("{}.{}", base_name, extension))?;
    path.create_file()?.write_all(&response.body)?;
    Ok(())
}

/// Mirrors one activity if needed.
fn mirror_activity(
    ctx: &Context,
    access_token: &str,
    summary: &ActivitySummary,
    activities_dir: &vfs::VfsPath,
    cookie: &str,
    mirrored_activities: &MirroredActivities,
) -> anyhow::Result<()> {
    let year = summary.start_date.year();
    let format = time::format_description::parse(ACTIVITY_TIMESTAMP_FORMAT)?;
    let timestamp = summary.start_date.format(&format)?;
    let id = summary.id;
    let base_name = format!("{}_{}", timestamp, id);
    let year_dir = activities_dir.join(year.to_string())?;
    year_dir.create_dir_all()?;

    let mirrored_activity = mirrored_activities.get(&summary.start_date);

    if mirrored_activity.is_none_or(|a| !a.have_meta) {
        let url = format!("https://www.strava.com/api/v3/activities/{}", id);
        info!("GET '{}', name is '{}'", url, summary.name);
        let mut headers = HashMap::new();
        headers.insert(
            "Authorization".to_string(),
            format!("Bearer {}", access_token),
        );
        let response = ctx.network.get(&url, &headers)?;

        if response.status_code != 200 {
            return Err(anyhow::anyhow!(
                "status is not success: {}",
                response.status_code
            ));
        }

        let activity_json: serde_json::Value = serde_json::from_slice(&response.body)?;
        let meta_path = year_dir.join(format!("{}.meta.json", base_name))?;
        meta_path
            .create_file()?
            .write_all(serde_json::to_string_pretty(&activity_json)?.as_bytes())?;
    }

    if mirrored_activity.is_none_or(|a| !a.have_data) {
        // Also download the actual activity.
        mirror_activity_data(ctx, id, &base_name, &year_dir, cookie)?;
    }

    Ok(())
}

#[derive(serde::Deserialize)]
struct NominatimResponse {
    address: NominatimAddress,
}

#[derive(serde::Deserialize)]
struct NominatimAddress {
    country: String,
}

#[derive(serde::Deserialize)]
struct ActivityMetadata {
    start_latlng: Option<Vec<f64>>,
}

/// Gets the country of one activity.
fn get_activity_country(
    ctx: &Context,
    entry: &vfs::VfsPath,
    cache: &mut HashMap<String, String>,
) -> anyhow::Result<Option<(String, String)>> {
    let filename = entry.filename();
    if !filename.ends_with(".meta.json") {
        return Ok(None);
    }

    let mut meta_content = String::new();
    entry.open_file()?.read_to_string(&mut meta_content)?;
    let metadata: ActivityMetadata = serde_json::from_str(&meta_content)?;
    let Some(start_latlng) = metadata.start_latlng else {
        return Ok(None);
    };
    if start_latlng.len() < 2 {
        return Ok(None);
    }

    let lat = start_latlng[0];
    let lon = start_latlng[1];
    let query = format!("lat={}&lon={}", lat, lon);
    let country = if let Some(country) = cache.get(&query) {
        country.to_string()
    } else {
        let url = format!(
            "https://nominatim.openstreetmap.org/reverse?{}&format=json",
            query
        );
        info!("GET '{}'", url);
        let response = ctx.network.get(&url, &HashMap::new())?;
        if response.status_code != 200 {
            return Err(anyhow::anyhow!(
                "status is not success: {}",
                response.status_code
            ));
        }
        let nominatim_response: NominatimResponse = serde_json::from_slice(&response.body)?;
        let country = nominatim_response.address.country;
        cache.insert(query, country.clone());
        std::thread::sleep(std::time::Duration::from_secs(1));
        country
    };
    Ok(Some((filename, country)))
}

/// Gets the country of activities based on their start location.
fn get_countries(ctx: &Context) -> anyhow::Result<HashMap<String, String>> {
    let mut countries = HashMap::new();
    let home = &ctx.fs;
    let activities_dir = home.join(".local/share/strava-mirror/activities")?;
    if !activities_dir.exists()? {
        return Ok(countries);
    }

    let cache_path = home.join(".local/share/strava-mirror/nominatim-cache.json")?;
    let mut cache: HashMap<String, String> = if cache_path.exists()? {
        let mut cache_content = String::new();
        cache_path.open_file()?.read_to_string(&mut cache_content)?;
        serde_json::from_str(&cache_content)?
    } else {
        HashMap::new()
    };

    for year_dir in activities_dir.read_dir()? {
        if year_dir.is_file()? {
            continue;
        }

        for entry in year_dir.read_dir()? {
            if let Some((filename, country)) = get_activity_country(ctx, &entry, &mut cache)? {
                countries.insert(filename, country);
            }
        }
    }

    let cache_dir = cache_path.parent();
    cache_dir.create_dir_all()?;
    cache_path
        .create_file()?
        .write_all(serde_json::to_string_pretty(&cache)?.as_bytes())?;

    Ok(countries)
}

/// Queries the country of an activity based on its start location.
fn query_countries(ctx: &Context) -> anyhow::Result<()> {
    let countries = get_countries(ctx)?;
    let mut filenames: Vec<_> = countries.keys().collect();
    filenames.sort();
    for filename in filenames {
        println!("{}: {}", filename, countries[filename]);
    }
    Ok(())
}

/// Summarizes countries of activities based on their start location.
fn query_countries_summary(ctx: &Context) -> anyhow::Result<()> {
    let countries = get_countries(ctx)?;
    let mut counts = HashMap::new();
    for country in countries.values() {
        let count = counts.entry(country.clone()).or_insert(0);
        *count += 1;
    }
    let mut sorted_counts: Vec<_> = counts.into_iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    for (country, count) in sorted_counts {
        println!("{}: {}", country, count);
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
    let _ = simplelog::CombinedLogger::init(vec![simplelog::TermLogger::new(
        level,
        config,
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Never,
    )]);
    Ok(())
}

/// Command-line arguments.
#[derive(clap::Parser)]
pub struct Args {
    /// Be quiet.
    #[arg(short, long)]
    pub quiet: bool,

    /// Query stats from local activities. Valid values: 'countries'.
    #[arg(long, value_name = "KIND")]
    pub query: Option<String>,

    /// Summarize query results.
    #[arg(long)]
    pub summary: bool,
}

/// Mirrors your Strava activities.
pub fn run(args: Vec<String>, ctx: &Context) -> anyhow::Result<()> {
    let args = Args::parse_from(args);
    let log_level = if args.quiet {
        log::LevelFilter::Error
    } else {
        log::LevelFilter::Info
    };
    setup_logging(log_level)?;

    if let Some(query) = args.query {
        if query == "countries" {
            if args.summary {
                return query_countries_summary(ctx);
            }
            return query_countries(ctx);
        }
        return Err(anyhow::anyhow!("unknown query: {}", query));
    }

    let home = &ctx.fs;

    let config = read_config(ctx)?;
    let access_token = get_access_token(ctx, &config)?;

    let activities_dir = home.join(".local/share/strava-mirror/activities")?;

    let mirrored_activities = get_mirrored_activities(&activities_dir)?;
    let newest_mirrored_activity = mirrored_activities
        .iter()
        .filter(|(_, a)| a.have_meta && a.have_data)
        .max_by_key(|(d, _)| *d);
    let after = newest_mirrored_activity.map(|(d, _)| d.unix_timestamp());

    let cookie = jwt_to_cookie(ctx, &config.jwt)?;
    let mut page = 1;
    loop {
        let activities: Vec<ActivitySummary> = list_activities(ctx, &access_token, page, after)?;
        if activities.is_empty() {
            break;
        }

        for activity in activities {
            mirror_activity(
                ctx,
                &access_token,
                &activity,
                &activities_dir,
                &cookie,
                &mirrored_activities,
            )?;
        }

        page += 1;
    }

    Ok(())
}

#[cfg(test)]
mod tests;
