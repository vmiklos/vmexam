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
const DISPLAY_TIMESTAMP_FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

/// Network response.
pub struct NetworkResponse {
    /// The headers.
    pub headers: HashMap<String, String>,
    /// The body.
    pub body: Vec<u8>,
}

/// Network interface.
pub trait Network {
    /// GET request.
    fn get(&self, url: &str, headers: &HashMap<String, String>) -> anyhow::Result<NetworkResponse>;
}

/// Process interface.
pub trait Process {
    /// Executes the command as a child process, waiting for it to finish and
    /// collecting all of its output.
    fn command_output(&self, command: &str, args: &[&str]) -> anyhow::Result<String>;
}

/// Time interface.
pub trait Time {
    /// Returns the current time in local time.
    fn now(&self) -> time::OffsetDateTime;
    /// Converts a Unix timestamp to local time.
    fn to_local_offset(&self, timestamp: i64) -> anyhow::Result<time::OffsetDateTime>;
    /// Sleeps for the given duration.
    fn sleep(&self, duration: std::time::Duration);
}

/// The context of the application.
pub struct Context {
    /// The filesystem to use.
    pub fs: vfs::VfsPath,
    /// The network to use.
    pub network: Rc<dyn Network>,
    /// The process runner to use.
    pub process: Rc<dyn Process>,
    /// The time to use.
    pub time: Rc<dyn Time>,
}

/// Contents of the config file.
#[derive(serde::Deserialize)]
struct Config {
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
    let format = time::format_description::parse_borrowed::<1>(DISPLAY_TIMESTAMP_FORMAT)?;
    let exp_formatted = exp_datetime
        .format(&format)
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

/// One .meta.json file in the mirrored activity list.
#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct ActivityMetadata {
    id: u64,
    name: String,
    #[serde(with = "time::serde::iso8601")]
    start_time: time::OffsetDateTime,
    sport_type: String,
    moving_time_raw: u64,
    elapsed_time_raw: u64,
    distance_raw: f64,
    elevation_gain_raw: f64,
}

/// Type of the /athlete/training_activities response.
#[derive(serde::Deserialize)]
struct ActivitiesResponse {
    /// ActivityMetadata and some more.
    models: Vec<serde_json::Value>,
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

    let format = time::format_description::parse_borrowed::<1>(ACTIVITY_TIMESTAMP_FORMAT)?;

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

/// Calculates the sleep duration if rate limits are reached.
fn get_sleep_duration(
    headers: &HashMap<String, String>,
    now: time::OffsetDateTime,
) -> anyhow::Result<std::time::Duration> {
    let limit = headers
        .get("x-readratelimit-limit")
        .or_else(|| headers.get("x-ratelimit-limit"))
        .context("no ratelimit-limit")?;
    let usage = headers
        .get("x-readratelimit-usage")
        .or_else(|| headers.get("x-ratelimit-usage"))
        .context("no ratelimit-usage")?;
    let limits: Vec<&str> = limit.split(',').collect();
    let usages: Vec<&str> = usage.split(',').collect();

    let min_limit: u64 = limits[0].parse()?;
    let min_usage: u64 = usages[0].parse()?;
    if min_usage >= min_limit {
        // Sleep until next 15-min boundary.
        let minutes = now.minute();
        let seconds = now.second();
        let next_boundary_minutes = (minutes / 15 + 1) * 15;
        let sleep_seconds = (next_boundary_minutes as u64 - minutes as u64) * 60 - seconds as u64;
        // Add a 10s buffer to be safe.
        return Ok(std::time::Duration::from_secs(sleep_seconds + 10));
    }

    let day_limit: u64 = limits.get(1).context("no 2nd limit")?.parse()?;
    let day_usage: u64 = usages.get(1).context("no 2nd usage")?.parse()?;
    if day_usage >= day_limit {
        // Sleep until next UTC midnight.
        let next_midnight = time::OffsetDateTime::new_in_offset(
            now.date() + time::Duration::days(1),
            time::macros::time!(00:00),
            time::UtcOffset::UTC,
        );
        let sleep_seconds = (next_midnight - now).whole_seconds() as u64;
        // Add a 10s buffer to be safe.
        return Ok(std::time::Duration::from_secs(sleep_seconds + 10));
    }

    Ok(std::time::Duration::from_secs(0))
}

/// Handles rate limiting based on response headers.
fn handle_rate_limit(ctx: &Context, headers: &HashMap<String, String>) {
    let Ok(sleep_duration) = get_sleep_duration(headers, ctx.time.now()) else {
        return;
    };
    if !sleep_duration.is_zero() {
        info!(
            "Sleeping for {} seconds due to rate limits",
            sleep_duration.as_secs()
        );
        ctx.time.sleep(sleep_duration);
    }
}

/// Lists activities: list of ActivityMetadata and some more.
fn list_activities(
    ctx: &Context,
    page: u32,
    _after: Option<i64>,
    cookie: &str,
) -> anyhow::Result<Vec<serde_json::Value>> {
    let url = "https://www.strava.com/athlete/training_activities?new_activity_only=false";
    if page > 1 {
        // TODO fetch older activities, too
        return Ok(Vec::new());
    }
    let mut headers = HashMap::new();
    headers.insert("Cookie".to_string(), cookie.to_string());
    headers.insert(
        "Accept".to_string(),
        "text/javascript, application/javascript, application/ecmascript, application/x-ecmascript"
            .to_string(),
    );
    headers.insert("X-Requested-With".to_string(), "XMLHttpRequest".to_string());
    let response = ctx.network.get(url, &headers)?;
    handle_rate_limit(ctx, &response.headers);
    let activities_r: ActivitiesResponse = serde_json::from_slice(&response.body)?;
    Ok(activities_r.models)
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
    let mut headers = HashMap::new();
    headers.insert("Cookie".to_string(), cookie.to_string());
    let response = ctx.network.get(&url, &headers)?;
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

/// Options for mirror_activity.
struct MirrorActivityOptions<'a> {
    activities_dir: &'a vfs::VfsPath,
    cookie: &'a str,
    mirrored_activities: &'a MirroredActivities,
    full_history: bool,
}

/// Checks if the metadata needs to be re-downloaded based on summary changes.
fn should_redownload_meta(metadata: &ActivityMetadata, summary: &ActivityMetadata) -> bool {
    metadata.name != summary.name || metadata.sport_type != summary.sport_type
}

/// Formats a duration in seconds as H:MM:SS.
fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    format!("{}:{:02}:{:02}", hours, minutes, seconds)
}

/// Formats a distance in meters as kilometers, rounded to 2 digits.
fn format_distance(meters: f64) -> String {
    format!("{:.2} km", meters / 1000.0)
}

/// Formats an elevation in meters, rounded to integer.
fn format_elevation(meters: f64) -> String {
    format!("{:.0} m", meters)
}

/// Mirrors one activity if needed.
fn mirror_activity(
    ctx: &Context,
    options: &MirrorActivityOptions,
    summary_unparsed: &serde_json::Value,
) -> anyhow::Result<()> {
    let summary: ActivityMetadata = serde_json::from_value(summary_unparsed.clone())?;
    let year = summary.start_time.year();
    let format = time::format_description::parse_borrowed::<1>(ACTIVITY_TIMESTAMP_FORMAT)?;
    let timestamp = summary.start_time.format(&format)?;
    let id = summary.id;
    let base_name = format!("{}_{}", timestamp, id);
    let year_dir = options.activities_dir.join(year.to_string())?;
    year_dir.create_dir_all()?;

    let mirrored_activity = options.mirrored_activities.get(&summary.start_time);
    let mut have_meta = mirrored_activity.is_some_and(|a| a.have_meta);

    if have_meta && options.full_history {
        let meta_path = year_dir.join(format!("{}.meta.json", base_name))?;
        let mut meta_content = String::new();
        meta_path.open_file()?.read_to_string(&mut meta_content)?;
        let metadata: ActivityMetadata = serde_json::from_str(&meta_content)?;
        if should_redownload_meta(&metadata, &summary) {
            have_meta = false;
        }
    }

    if !have_meta {
        info!("Mirroring activity, name is '{}'", summary.name);
        let meta_path = year_dir.join(format!("{}.meta.json", base_name))?;
        meta_path
            .create_file()?
            .write_all(serde_json::to_string_pretty(&summary_unparsed)?.as_bytes())?;
    }

    if mirrored_activity.is_none_or(|a| !a.have_data) {
        // Also download the actual activity.
        mirror_activity_data(ctx, id, &base_name, &year_dir, options.cookie)?;
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

#[derive(Clone)]
struct QueriedActivity {
    country: String,
    metadata: ActivityMetadata,
}

/// Scans local activities and returns their metadata.
fn get_local_activities(ctx: &Context) -> anyhow::Result<Vec<(String, ActivityMetadata)>> {
    let mut activities = Vec::new();
    let home = &ctx.fs;
    let activities_dir = home.join(".local/share/strava-mirror/activities")?;
    if !activities_dir.exists()? {
        return Ok(activities);
    }

    for year_dir in activities_dir.read_dir()? {
        if year_dir.is_file()? {
            continue;
        }

        for entry in year_dir.read_dir()? {
            let filename = entry.filename();
            if !filename.ends_with(".meta.json") {
                continue;
            }

            let mut meta_content = String::new();
            entry.open_file()?.read_to_string(&mut meta_content)?;
            let metadata: ActivityMetadata = serde_json::from_str(&meta_content)
                .context(format!("failed to parse {}", filename))?;
            let path = format!("{}/{}", year_dir.filename(), filename);
            activities.push((path, metadata));
        }
    }
    Ok(activities)
}

/// Gets the coordinates of an activity from a .fit file.
fn get_activity_lat_lon(ctx: &Context, filename: &str) -> anyhow::Result<(String, String)> {
    let home = &ctx.fs;
    let activities_dir = home.join(".local/share/strava-mirror/activities")?;
    let base_name = filename.strip_suffix(".meta.json").context("bad suffix")?;
    let data_path = activities_dir.join(format!("{base_name}.fit"))?;
    if !data_path.exists()? {
        return Err(anyhow::anyhow!("no data file: {data_path:?}"));
    }

    let home_dir = home::home_dir().context("home_dir() failed")?;
    let real_data_path = home_dir.join(data_path.as_str().trim_start_matches('/'));
    let real_data_path = real_data_path.to_str().context("to_str() failed")?;
    let args = [
        "-i",
        "garmin_fit",
        "-f",
        real_data_path,
        "-o",
        "geojson",
        "-F",
        "-",
    ];
    let output = ctx.process.command_output("gpsbabel", &args)?;

    let json: serde_json::Value = serde_json::from_str(&output)?;
    let point = json["features"][0]["geometry"]["coordinates"][0]
        .as_array()
        .context("no first coordinate")?;
    let lon = point[0].as_f64().context("longitude is not a float")?;
    let lat = point[1].as_f64().context("latitude is not a float")?;
    Ok((lat.to_string(), lon.to_string()))
}

/// Gets the country of one activity from its metadata.
fn get_activity_country(
    ctx: &Context,
    filename: &str,
    metadata: ActivityMetadata,
    cache: &mut HashMap<String, String>,
) -> anyhow::Result<Option<QueriedActivity>> {
    let format = time::format_description::parse_borrowed::<1>(ACTIVITY_TIMESTAMP_FORMAT)?;
    let timestamp = metadata.start_time.format(&format)?;
    let id = metadata.id;
    let cache_key = format!("{}_{}", timestamp, id);
    let country = if let Some(country) = cache.get(&cache_key) {
        country.to_string()
    } else {
        let Ok((lat, lon)) = get_activity_lat_lon(ctx, filename) else {
            return Ok(None);
        };
        let url = format!(
            "https://nominatim.openstreetmap.org/reverse?lat={}&lon={}&format=json",
            lat, lon,
        );
        let mut headers = HashMap::new();
        headers.insert("Accept-Language".to_string(), "en-US".to_string());
        let response = ctx.network.get(&url, &headers)?;
        let nominatim_response: NominatimResponse = serde_json::from_slice(&response.body)?;
        let country = nominatim_response.address.country;
        cache.insert(cache_key, country.clone());
        ctx.time.sleep(std::time::Duration::from_secs(1));
        country
    };
    let activity = QueriedActivity { country, metadata };
    Ok(Some(activity))
}

/// Gets the country of activities based on their start location.
fn get_countries(ctx: &Context) -> anyhow::Result<HashMap<String, QueriedActivity>> {
    let mut countries = HashMap::new();
    let home = &ctx.fs;

    let cache_path = home.join(".local/share/strava-mirror/countries-cache.json")?;
    let mut cache: HashMap<String, String> = if cache_path.exists()? {
        let mut cache_content = String::new();
        cache_path.open_file()?.read_to_string(&mut cache_content)?;
        serde_json::from_str(&cache_content)?
    } else {
        HashMap::new()
    };

    let local_activities = get_local_activities(ctx)?;
    for (filename, metadata) in local_activities {
        if let Some(activity) = get_activity_country(ctx, &filename, metadata, &mut cache)? {
            countries.insert(filename, activity);
        }
    }

    let cache_dir = cache_path.parent();
    cache_dir.create_dir_all()?;
    cache_path
        .create_file()?
        .write_all(serde_json::to_string_pretty(&cache)?.as_bytes())?;

    Ok(countries)
}

/// Queries all local activity metadata.
fn query_custom(ctx: &Context) -> anyhow::Result<()> {
    let local_activities = get_local_activities(ctx)?;
    let mut activities: Vec<ActivityMetadata> =
        local_activities.into_iter().map(|(_, m)| m).collect();
    activities.sort_by_key(|m| m.start_time);
    println!("{}", serde_json::to_string_pretty(&activities)?);
    Ok(())
}

/// Queries the top 10 longest walks by time.
fn query_top_walks_by_time(ctx: &Context) -> anyhow::Result<()> {
    let local_activities = get_local_activities(ctx)?;
    let markup = get_top_walks_by_time_content(local_activities)?;
    println!("{}", wrap_in_page(markup).into_string());
    Ok(())
}

/// Produces the HTML content for the top 10 longest walks by time.
fn get_top_walks_by_time_content(
    local_activities: Vec<(String, ActivityMetadata)>,
) -> anyhow::Result<maud::Markup> {
    get_top_activities_content(local_activities, "Walk", "Top walks by time", |m| {
        std::cmp::Reverse(m.moving_time_raw)
    })
}

/// Queries the top 10 longest walks by distance.
fn query_top_walks_by_distance(ctx: &Context) -> anyhow::Result<()> {
    let local_activities = get_local_activities(ctx)?;
    let markup = get_top_walks_by_distance_content(local_activities)?;
    println!("{}", wrap_in_page(markup).into_string());
    Ok(())
}

/// Produces the HTML content for the top 10 longest walks by distance.
fn get_top_walks_by_distance_content(
    local_activities: Vec<(String, ActivityMetadata)>,
) -> anyhow::Result<maud::Markup> {
    get_top_activities_content(local_activities, "Walk", "Top walks by distance", |m| {
        std::cmp::Reverse(m.distance_raw as u64)
    })
}

/// Queries the top 10 longest walks by elevation.
fn query_top_walks_by_elevation(ctx: &Context) -> anyhow::Result<()> {
    let local_activities = get_local_activities(ctx)?;
    let markup = get_top_walks_by_elevation_content(local_activities)?;
    println!("{}", wrap_in_page(markup).into_string());
    Ok(())
}

/// Produces the HTML content for the top 10 longest walks by elevation.
fn get_top_walks_by_elevation_content(
    local_activities: Vec<(String, ActivityMetadata)>,
) -> anyhow::Result<maud::Markup> {
    get_top_activities_content(local_activities, "Walk", "Top walks by elevation", |m| {
        std::cmp::Reverse(m.elevation_gain_raw as u64)
    })
}

/// Queries the top 10 longest rides by time.
fn query_top_rides_by_time(ctx: &Context) -> anyhow::Result<()> {
    let local_activities = get_local_activities(ctx)?;
    let markup = get_top_rides_by_time_content(local_activities)?;
    println!("{}", wrap_in_page(markup).into_string());
    Ok(())
}

/// Produces the HTML content for the top 10 longest rides by time.
fn get_top_rides_by_time_content(
    local_activities: Vec<(String, ActivityMetadata)>,
) -> anyhow::Result<maud::Markup> {
    get_top_activities_content(local_activities, "Ride", "Top rides by time", |m| {
        std::cmp::Reverse(m.moving_time_raw)
    })
}

/// Queries the top 10 longest rides by distance.
fn query_top_rides_by_distance(ctx: &Context) -> anyhow::Result<()> {
    let local_activities = get_local_activities(ctx)?;
    let markup = get_top_rides_by_distance_content(local_activities)?;
    println!("{}", wrap_in_page(markup).into_string());
    Ok(())
}

/// Produces the HTML content for the top 10 longest rides by distance.
fn get_top_rides_by_distance_content(
    local_activities: Vec<(String, ActivityMetadata)>,
) -> anyhow::Result<maud::Markup> {
    get_top_activities_content(local_activities, "Ride", "Top rides by distance", |m| {
        std::cmp::Reverse(m.distance_raw as u64)
    })
}

/// Queries the top 10 longest rides by elevation.
fn query_top_rides_by_elevation(ctx: &Context) -> anyhow::Result<()> {
    let local_activities = get_local_activities(ctx)?;
    let markup = get_top_rides_by_elevation_content(local_activities)?;
    println!("{}", wrap_in_page(markup).into_string());
    Ok(())
}

/// Produces the HTML content for the top 10 longest rides by elevation.
fn get_top_rides_by_elevation_content(
    local_activities: Vec<(String, ActivityMetadata)>,
) -> anyhow::Result<maud::Markup> {
    get_top_activities_content(local_activities, "Ride", "Top rides by elevation", |m| {
        std::cmp::Reverse(m.elevation_gain_raw as u64)
    })
}

/// Queries the longest ride by distance in each year.
fn query_longest_rides_by_year(ctx: &Context) -> anyhow::Result<()> {
    let local_activities = get_local_activities(ctx)?;
    let markup = get_longest_rides_by_year_content(local_activities)?;
    println!("{}", wrap_in_page(markup).into_string());
    Ok(())
}

/// Produces the HTML content for the longest ride by distance in each year.
fn get_longest_rides_by_year_content(
    mut local_activities: Vec<(String, ActivityMetadata)>,
) -> anyhow::Result<maud::Markup> {
    let mut by_year: HashMap<i32, ActivityMetadata> = HashMap::new();
    local_activities.sort_by_key(|m| std::cmp::Reverse(m.0.to_string()));
    for (_, m) in local_activities {
        if m.sport_type != "Ride" {
            continue;
        }
        by_year
            .entry(m.start_time.year())
            .and_modify(|best| {
                if m.distance_raw > best.distance_raw {
                    *best = m.clone();
                }
            })
            .or_insert(m);
    }
    let mut activities: Vec<ActivityMetadata> = by_year.into_values().collect();
    activities.sort_by_key(|m| std::cmp::Reverse(m.start_time.year()));
    render_activities_table(&activities, "Longest rides by year")
}

/// Produces the HTML content for the top 10 longest activities based on a custom sort key.
fn get_top_activities_content<K>(
    local_activities: Vec<(String, ActivityMetadata)>,
    sport_type: &str,
    title: &str,
    mut sort_key: impl FnMut(&ActivityMetadata) -> K,
) -> anyhow::Result<maud::Markup>
where
    K: Ord,
{
    let mut activities: Vec<ActivityMetadata> = local_activities
        .into_iter()
        .map(|(_, m)| m)
        .filter(|m| m.sport_type == sport_type)
        .collect();
    activities.sort_by_key(|m| sort_key(m));
    let top_10 = &activities[..std::cmp::min(10, activities.len())];
    render_activities_table(top_10, title)
}

/// Renders a list of activities as an HTML table with the given title.
fn render_activities_table(
    activities: &[ActivityMetadata],
    title: &str,
) -> anyhow::Result<maud::Markup> {
    let format = time::format_description::parse_borrowed::<1>(DISPLAY_TIMESTAMP_FORMAT)?;
    let markup = maud::html! {
        h1 { (title) }
        table border="1" {
            thead {
                tr {
                    th { "Sport type" }
                    th { "Start date" }
                    th { "Title" }
                    th { "Moving time" }
                    th { "Distance" }
                    th { "Elevation" }
                }
            }
            tbody {
                @for activity in activities {
                    tr {
                        td { (activity.sport_type) }
                        td { (activity.start_time.format(&format)?) }
                        td {
                            a href=(format!("https://www.strava.com/activities/{}", activity.id)) {
                                (activity.name)
                            }
                        }
                        td { (format_duration(activity.moving_time_raw)) }
                        td { (format_distance(activity.distance_raw)) }
                        td { (format_elevation(activity.elevation_gain_raw)) }
                    }
                }
            }
        }
    };
    Ok(markup)
}

/// Queries the country of an activity based on its start location.
fn query_countries(ctx: &Context) -> anyhow::Result<()> {
    let countries = get_countries(ctx)?;
    let mut filenames: Vec<_> = countries.keys().collect();
    filenames.sort();
    for filename in filenames {
        println!("{}: {}", filename, countries[filename].country);
    }
    Ok(())
}

/// Summarizes countries of activities based on their start location.
fn query_countries_summary(ctx: &Context) -> anyhow::Result<()> {
    let activities = get_countries(ctx)?;
    let mut counts = HashMap::new();
    for activity in activities.values() {
        let count = counts.entry(activity.country.clone()).or_insert(0);
        *count += 1;
    }
    let mut sorted_counts: Vec<_> = counts.into_iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    for (country, count) in sorted_counts {
        println!("{}: {}", country, count);
    }
    Ok(())
}

struct ActivityItem {
    timestamp: String,
    url: String,
    name: String,
}

struct CountryItem {
    name: String,
    count: usize,
    activities: Vec<ActivityItem>,
}

/// Summarizes countries of activities based on their start location in HTML format.
fn query_countries_html(ctx: &Context) -> anyhow::Result<()> {
    let activities_map = get_countries(ctx)?;
    let markup = get_countries_html_content(activities_map)?;
    println!("{}", wrap_in_page(markup).into_string());

    Ok(())
}

/// Produces the HTML content for country statistics.
fn get_countries_html_content(
    activities_map: HashMap<String, QueriedActivity>,
) -> anyhow::Result<maud::Markup> {
    let mut country_activities: HashMap<String, Vec<QueriedActivity>> = HashMap::new();

    for (_filename, activity) in activities_map {
        country_activities
            .entry(activity.country.to_string())
            .or_default()
            .push(activity);
    }

    let total_activities: usize = country_activities.values().map(|v| v.len()).sum();
    let total_countries = country_activities.len();

    let mut sorted_countries: Vec<_> = country_activities.into_iter().collect();
    sorted_countries.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then_with(|| a.0.cmp(&b.0)));

    let format = time::format_description::parse_borrowed::<1>(DISPLAY_TIMESTAMP_FORMAT)?;
    let mut country_items = Vec::new();
    for (country, mut activities) in sorted_countries {
        activities.sort_by_key(|b| std::cmp::Reverse(b.metadata.start_time));
        let count = activities.len();
        let mut list_items = Vec::new();
        for activity in activities {
            let timestamp = activity.metadata.start_time.format(&format)?;
            let url = format!("https://www.strava.com/activities/{}", activity.metadata.id);
            let name = activity.metadata.name;
            list_items.push(ActivityItem {
                timestamp,
                url,
                name,
            });
        }
        country_items.push(CountryItem {
            name: country,
            count,
            activities: list_items,
        });
    }

    let markup = maud::html! {
        h1 { "Countries" }
        p {
            (total_activities)
            " activities in "
            (total_countries)
            " countries."
        }
        @for country_item in country_items {
            details {
                summary {
                    (country_item.name)
                    ": "
                    (country_item.count)
                }
                ul {
                    @for item in country_item.activities {
                        li {
                            (item.timestamp)
                            ": "
                            a href=(item.url) {
                                (item.name)
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(markup)
}

/// Wraps content in a full HTML page.
fn wrap_in_page(content: maud::Markup) -> maud::Markup {
    maud::html! {
        (maud::DOCTYPE)
        html lang="en-US" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "sport-stats" }
            }
            body {
                (content)
                hr;
                p {
                    "Generated by "
                    a href="https://github.com/vmiklos/vmexam/tree/master/strava-mirror" {
                        "strava-mirror"
                    }
                }
            }
        }
    }
}

/// Queries all statistics.
fn query_all(ctx: &Context) -> anyhow::Result<()> {
    let activities_map = get_countries(ctx)?;
    let countries_content = get_countries_html_content(activities_map.clone())?;

    let local_activities: Vec<(String, ActivityMetadata)> = activities_map
        .into_iter()
        .map(|(f, a)| (f, a.metadata))
        .collect();
    let top_walks_time_content = get_top_walks_by_time_content(local_activities.clone())?;
    let top_walks_distance_content = get_top_walks_by_distance_content(local_activities.clone())?;
    let top_walks_elevation_content = get_top_walks_by_elevation_content(local_activities.clone())?;
    let top_rides_time_content = get_top_rides_by_time_content(local_activities.clone())?;
    let top_rides_distance_content = get_top_rides_by_distance_content(local_activities.clone())?;
    let top_rides_elevation_content = get_top_rides_by_elevation_content(local_activities.clone())?;
    let longest_rides_by_year_content = get_longest_rides_by_year_content(local_activities)?;

    let combined_content = maud::html! {
        (countries_content)
        (top_walks_time_content)
        (top_walks_distance_content)
        (top_walks_elevation_content)
        (top_rides_time_content)
        (top_rides_distance_content)
        (top_rides_elevation_content)
        (longest_rides_by_year_content)
    };
    println!("{}", wrap_in_page(combined_content).into_string());
    Ok(())
}

/// Sets up logging so it has local time timestamp as a prefix.
fn setup_logging(level: log::LevelFilter) -> anyhow::Result<()> {
    let mut builder = simplelog::ConfigBuilder::new();
    builder.set_time_format_custom(simplelog::format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second]"
    ));

    // Try to use local time, if possible.
    let _ret = builder.set_time_offset_to_local();

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

    /// Query stats from local activities. Valid values: 'countries', 'custom', 'top-walks-by-time', 'top-walks-by-distance', 'top-walks-by-elevation', 'top-rides-by-time', 'top-rides-by-distance', 'top-rides-by-elevation', 'longest-rides-by-year', 'all'.
    #[arg(long, value_name = "KIND")]
    pub query: Option<String>,

    /// Summarize query results.
    #[arg(long)]
    pub summary: bool,

    /// Output query results as HTML.
    #[arg(long)]
    pub html: bool,

    /// Fetch all activities, don't stop at the newest mirrored one.
    #[arg(long)]
    pub full_history: bool,
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
            if args.html {
                return query_countries_html(ctx);
            }
            return query_countries(ctx);
        }
        if query == "custom" {
            return query_custom(ctx);
        }
        if query == "top-walks-by-time" {
            return query_top_walks_by_time(ctx);
        }
        if query == "top-walks-by-distance" {
            return query_top_walks_by_distance(ctx);
        }
        if query == "top-walks-by-elevation" {
            return query_top_walks_by_elevation(ctx);
        }
        if query == "top-rides-by-time" {
            return query_top_rides_by_time(ctx);
        }
        if query == "top-rides-by-distance" {
            return query_top_rides_by_distance(ctx);
        }
        if query == "top-rides-by-elevation" {
            return query_top_rides_by_elevation(ctx);
        }
        if query == "longest-rides-by-year" {
            return query_longest_rides_by_year(ctx);
        }
        if query == "all" {
            return query_all(ctx);
        }
        return Err(anyhow::anyhow!("unknown query: {}", query));
    }

    let home = &ctx.fs;

    let config = read_config(ctx)?;

    let activities_dir = home.join(".local/share/strava-mirror/activities")?;

    let mirrored_activities = get_mirrored_activities(&activities_dir)?;
    let after = if args.full_history {
        None
    } else {
        let newest_mirrored_activity = mirrored_activities
            .iter()
            .filter(|(_, a)| a.have_meta && a.have_data)
            .max_by_key(|(d, _)| *d);
        newest_mirrored_activity.map(|(d, _)| d.unix_timestamp())
    };

    let cookie = jwt_to_cookie(ctx, &config.jwt)?;
    let options = MirrorActivityOptions {
        activities_dir: &activities_dir,
        cookie: &cookie,
        mirrored_activities: &mirrored_activities,
        full_history: args.full_history,
    };
    let mut page = 1;
    loop {
        let activities: Vec<serde_json::Value> = list_activities(ctx, page, after, &cookie)?;
        if activities.is_empty() {
            break;
        }

        for activity in activities {
            mirror_activity(ctx, &options, &activity)?;
        }

        page += 1;
    }

    Ok(())
}

#[cfg(test)]
mod tests;
