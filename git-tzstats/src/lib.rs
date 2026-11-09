/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Statistics about unusual timezones in git history.

use anyhow::Context as _;
use std::collections::{HashMap, HashSet};

/// Context interface.
pub trait Context {
    /// Executes a command as a child process and collects its standard output.
    fn command_output(&self, command: &str, args: &[&str]) -> anyhow::Result<String>;

    /// Returns the arguments that this program was started with.
    fn env_args(&self) -> Vec<String>;

    /// Prints to the standard output.
    fn print(&self, string: &str);
}

/// Time interface.
pub trait Time {
    /// Returns the offset of the local timezone at the given instant.
    fn local_offset(&self, date_time: time::OffsetDateTime) -> anyhow::Result<time::UtcOffset>;
}

struct Commit {
    author: String,
    date_time: time::OffsetDateTime,
}

struct Travel {
    offset: time::UtcOffset,
    start: time::Date,
}

/// Number of days after (or before) a DST transition during which the old offset is still
/// considered normal.
const DST_GRACE_DAYS: i64 = 7;

fn parse_commits(output: &str) -> anyhow::Result<Vec<Commit>> {
    let mut commits = Vec::new();
    for record in output.split('\x1e') {
        let record = record.strip_prefix('\n').unwrap_or(record);
        if record.is_empty() {
            continue;
        }
        let (author, date) = record
            .split_once('\x1f')
            .context("missing field separator in git log output")?;
        let date_time =
            time::OffsetDateTime::parse(date, &time::format_description::well_known::Rfc3339)
                .with_context(|| format!("failed to parse author date '{date}'"))?;
        commits.push(Commit {
            author: author.to_string(),
            date_time,
        });
    }
    Ok(commits)
}

fn format_offset(offset: time::UtcOffset) -> String {
    let total_minutes = offset.whole_seconds() / 60;
    let sign = if total_minutes < 0 { '-' } else { '+' };
    let total_minutes = total_minutes.abs();
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;
    format!("{sign}{hours:02}{minutes:02}")
}

fn format_date(date: time::Date) -> String {
    format!(
        "{:04}-{:02}-{:02}",
        date.year(),
        u8::from(date.month()),
        date.day()
    )
}

fn expected_offset(
    time: &dyn Time,
    date_time: time::OffsetDateTime,
    delta_seconds: i64,
) -> anyhow::Result<time::UtcOffset> {
    let local_offset = time.local_offset(date_time)?;
    let expected_seconds = i64::from(local_offset.whole_seconds()) + delta_seconds;
    let expected_seconds = i32::try_from(expected_seconds).context("offset out of range")?;
    time::UtcOffset::from_whole_seconds(expected_seconds).context("invalid offset")
}

/// Returns true if the given offset is the normal home offset just before or after a DST
/// transition close to the given date. Such an offset is not a travel.
fn is_dst_grace_offset(
    time: &dyn Time,
    date_time: time::OffsetDateTime,
    delta_seconds: i64,
    offset: time::UtcOffset,
) -> anyhow::Result<bool> {
    for days in [-DST_GRACE_DAYS, DST_GRACE_DAYS] {
        let other_date_time = date_time + time::Duration::days(days);
        let other_expected = expected_offset(time, other_date_time, delta_seconds)?;
        if offset == other_expected {
            return Ok(true);
        }
    }
    Ok(false)
}

fn process_author(
    ctx: &dyn Context,
    time: &dyn Time,
    author: &str,
    commits: &mut [&Commit],
) -> anyhow::Result<()> {
    commits.sort_by_key(|commit| commit.date_time);
    let mut by_day: HashMap<time::Date, Vec<&Commit>> = HashMap::new();
    for commit in commits.iter() {
        by_day
            .entry(commit.date_time.date())
            .or_default()
            .push(commit);
    }
    let mut days: Vec<(time::Date, Vec<&Commit>)> = by_day.into_iter().collect();
    days.sort_by_key(|(date, _)| *date);
    let mut current_year = None;
    let mut delta_seconds = 0;
    let mut travel: Option<Travel> = None;
    for (date, day_commits) in days {
        let year = date.year();
        let first = day_commits[0];
        if current_year != Some(year) {
            current_year = Some(year);
            let local_offset = time.local_offset(first.date_time)?;
            delta_seconds = i64::from(first.date_time.offset().whole_seconds())
                - i64::from(local_offset.whole_seconds());
            travel = None;
        }
        let offsets: HashSet<time::UtcOffset> = day_commits
            .iter()
            .map(|commit| commit.date_time.offset())
            .collect();
        if offsets.len() > 1 {
            // A single day with multiple timezones is likely a misconfiguration, not travel.
            continue;
        }
        let offset = *offsets.iter().next().context("no offset")?;
        let expected = expected_offset(time, first.date_time, delta_seconds)?;
        if offset == expected {
            if let Some(travel) = travel.take() {
                ctx.print(&format!(
                    "{author} traveled to {} on {} and returned to {} on {}.\n",
                    format_offset(travel.offset),
                    format_date(travel.start),
                    format_offset(expected),
                    format_date(date)
                ));
            }
        } else if travel.is_none()
            && !is_dst_grace_offset(time, first.date_time, delta_seconds, offset)?
        {
            travel = Some(Travel {
                offset,
                start: date,
            });
        }
    }
    Ok(())
}

fn report(ctx: &dyn Context, time: &dyn Time, commits: &[Commit]) -> anyhow::Result<()> {
    let mut by_author: HashMap<&str, Vec<&Commit>> = HashMap::new();
    for commit in commits {
        by_author
            .entry(commit.author.as_str())
            .or_default()
            .push(commit);
    }
    let mut authors: Vec<(&str, Vec<&Commit>)> = by_author.into_iter().collect();
    authors.sort_by_key(|(author, _)| *author);
    for (author, mut commits) in authors {
        process_author(ctx, time, author, &mut commits)?;
    }
    Ok(())
}

/// Similar to plain main(), but with an interface that allows testing.
pub fn main(ctx: &dyn Context, time: &dyn Time) -> anyhow::Result<()> {
    let author = clap::Arg::new("author")
        .long("author")
        .help("limit commits to the given author");
    let since = clap::Arg::new("since")
        .long("since")
        .help("limit commits to those after the given date");
    let until = clap::Arg::new("until")
        .long("until")
        .help("limit commits to those before the given date");
    let app = clap::Command::new("git-tzstats").args([author, since, until]);
    let args = app.try_get_matches_from(ctx.env_args())?;

    let mut git_args: Vec<String> = vec![
        "log".to_string(),
        "--pretty=format:%an%x1f%aI%x1e".to_string(),
    ];
    if let Some(author) = args.get_one::<String>("author") {
        git_args.push(format!("--author={author}"));
    }
    if let Some(since) = args.get_one::<String>("since") {
        git_args.push(format!("--since={since}"));
    }
    if let Some(until) = args.get_one::<String>("until") {
        git_args.push(format!("--until={until}"));
    }
    let git_args: Vec<&str> = git_args.iter().map(String::as_str).collect();
    let output = ctx.command_output("git", &git_args)?;
    let commits = parse_commits(&output)?;
    report(ctx, time, &commits)
}

#[cfg(test)]
mod tests;
