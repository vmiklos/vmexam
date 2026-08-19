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

mod stats;

#[cfg(not(test))]
use log::info;

#[cfg(test)]
use std::println as info;

const ACTIVITY_TIMESTAMP_FORMAT: &str = "[year]-[month]-[day]T[hour]-[minute]-[second]Z";
pub(crate) const DISPLAY_TIMESTAMP_FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

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
pub(crate) struct ActivityMetadata {
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

/// Lists activities: list of ActivityMetadata and some more.
fn list_activities(
    ctx: &Context,
    page: u32,
    cookie: &str,
) -> anyhow::Result<Vec<serde_json::Value>> {
    let url = format!(
        "https://www.strava.com/athlete/training_activities?new_activity_only=false&page={}",
        page
    );
    let mut headers = HashMap::new();
    headers.insert("Cookie".to_string(), cookie.to_string());
    headers.insert(
        "Accept".to_string(),
        "text/javascript, application/javascript, application/ecmascript, application/x-ecmascript"
            .to_string(),
    );
    headers.insert("X-Requested-With".to_string(), "XMLHttpRequest".to_string());
    let response = ctx.network.get(&url, &headers)?;
    ctx.time.sleep(std::time::Duration::from_secs(1));
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
    ctx.time.sleep(std::time::Duration::from_secs(1));
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
pub(crate) struct QueriedActivity {
    pub(crate) country: String,
    pub(crate) metadata: ActivityMetadata,
}

/// Scans local activities and returns their metadata.
pub(crate) fn get_local_activities(
    ctx: &Context,
) -> anyhow::Result<Vec<(String, ActivityMetadata)>> {
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
pub(crate) fn get_countries(ctx: &Context) -> anyhow::Result<Vec<QueriedActivity>> {
    let mut countries = Vec::new();
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
            countries.push(activity);
        }
    }

    let cache_dir = cache_path.parent();
    cache_dir.create_dir_all()?;
    cache_path
        .create_file()?
        .write_all(serde_json::to_string_pretty(&cache)?.as_bytes())?;

    Ok(countries)
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

    /// Query stats from local activities. Valid values: 'countries', 'top-walks-by-time', 'top-walks-by-distance', 'top-walks-by-elevation', 'top-rides-by-time', 'top-rides-by-distance', 'top-rides-by-elevation', 'longest-rides-by-year', 'total-distance-by-year', 'all'.
    #[arg(long, value_name = "KIND")]
    pub query: Option<String>,

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
            return stats::query_countries(ctx);
        }
        if query == "top-walks-by-time" {
            return stats::query_top_walks_by_time(ctx);
        }
        if query == "top-walks-by-distance" {
            return stats::query_top_walks_by_distance(ctx);
        }
        if query == "top-walks-by-elevation" {
            return stats::query_top_walks_by_elevation(ctx);
        }
        if query == "top-rides-by-time" {
            return stats::query_top_rides_by_time(ctx);
        }
        if query == "top-rides-by-distance" {
            return stats::query_top_rides_by_distance(ctx);
        }
        if query == "top-rides-by-elevation" {
            return stats::query_top_rides_by_elevation(ctx);
        }
        if query == "longest-rides-by-year" {
            return stats::query_longest_rides_by_year(ctx);
        }
        if query == "total-distance-by-year" {
            return stats::query_total_distance_by_year(ctx);
        }
        if query == "activity-count-by-year" {
            return stats::query_activity_count_by_year(ctx);
        }
        if query == "activity-type-breakdown" {
            return stats::query_activity_type_breakdown(ctx);
        }
        if query == "all" {
            return stats::query_all(ctx);
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
        let activities: Vec<serde_json::Value> = list_activities(ctx, page, &cookie)?;
        if activities.is_empty() {
            break;
        }

        let mut partial_page = false;
        for activity in activities {
            if let Some(value) = after {
                let activity: ActivityMetadata = serde_json::from_value(activity.clone())?;
                if activity.start_time.unix_timestamp() <= value {
                    partial_page = true;
                    break;
                }
            }

            mirror_activity(ctx, &options, &activity)?;
        }

        if partial_page {
            break;
        }

        page += 1;
    }

    get_countries(ctx)?;

    Ok(())
}

#[cfg(test)]
mod tests;
