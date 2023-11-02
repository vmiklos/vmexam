/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

//! Simple search tool for weechat logs.
//!
//! Assumes that logs are placed in ~/.local/share/weechat/logs/YYYY/MM/channel.weechatlog files.

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

use anyhow::Context as _;
use clap::Parser as _;
use std::io::BufRead as _;

#[derive(clap::Parser)]
struct Arguments {
    /// Sender of the message (regex).
    #[arg(short, long)]
    from: Option<String>,
    /// Name of the channel where the message appeared (regex).
    #[arg(short, long)]
    channel: Option<String>,
    /// Date in a YYYY-MM form (regex), defaults to the current month, 'all' disables the filter.
    #[arg(short, long)]
    date: Option<String>,
    /// The content of the message (regex).
    content: Option<String>,
    /// Case-insensitive mode, disabled by default
    #[arg(short, long)]
    ignore_case: bool,
    /// Interpret filters as a fixed string (instead of a regular expression).
    #[arg(short = 'F', long)]
    fixed_strings: bool,
}

/// Regex or fixed string matcher.
struct Matcher {
    /// If this is Some, a regex match is performed.
    regex: Option<regex::Regex>,
    /// For the fixed string case.
    needle: String,
    /// Case-insensitive mode for the fixed string.
    ignore_case: bool,
}

impl Matcher {
    fn from_regex(value: &str, args: &Arguments) -> anyhow::Result<Self> {
        let regex = Some(
            regex::RegexBuilder::new(value)
                .case_insensitive(args.ignore_case)
                .build()?,
        );
        let needle = "".to_string();
        let ignore_case = false;
        Ok(Matcher {
            regex,
            needle,
            ignore_case,
        })
    }

    fn from_fixed(value: &str, args: &Arguments) -> anyhow::Result<Self> {
        let regex = None;
        let needle = if args.ignore_case {
            value.to_lowercase()
        } else {
            value.to_string()
        };
        let ignore_case = args.ignore_case;
        Ok(Matcher {
            regex,
            needle,
            ignore_case,
        })
    }

    fn new(needle: &str, args: &Arguments) -> anyhow::Result<Self> {
        let needle = unidecode::unidecode(needle);
        if args.fixed_strings {
            Self::from_fixed(&needle, args)
        } else {
            Self::from_regex(&needle, args)
        }
    }

    fn is_match(&self, haystack: &str) -> bool {
        let haystack = unidecode::unidecode(haystack);
        if let Some(ref regex) = self.regex {
            return regex.is_match(&haystack);
        }

        if self.ignore_case {
            haystack.to_lowercase().contains(&self.needle)
        } else {
            haystack.contains(&self.needle)
        }
    }
}

fn main() -> anyhow::Result<()> {
    // Parse the arguments.
    let args = Arguments::parse();

    // Set up the filters.
    let from_filter = match args.from {
        Some(ref value) => Some(Matcher::new(value.as_str(), &args)?),
        None => None,
    };
    let channel_filter = match args.channel {
        Some(ref value) => Some(Matcher::new(value.as_str(), &args)?),
        None => None,
    };
    let date_filter = match args.date {
        Some(ref date) => {
            if date == "all" {
                None
            } else {
                Some(Matcher::new(date.as_str(), &args)?)
            }
        }
        None => {
            // Default to the current month.
            let tz_offset = time::UtcOffset::current_local_offset()?;
            let now = time::OffsetDateTime::now_utc().to_offset(tz_offset);
            let format = time::format_description::parse("[year]-[month]")?;
            Some(Matcher::new(&now.format(&format)?, &args)?)
        }
    };
    let content_filter = match args.content {
        Some(ref value) => Some(Matcher::new(value.as_str(), &args)?),
        None => None,
    };

    // Search.
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let home_dir: String = home_dir.to_str().context("to_str() failed")?.into();
    let years = format!("{home_dir}/.local/share/weechat/logs");
    let mut results = Vec::new();
    for year in std::fs::read_dir(years)? {
        let year = year?;
        let year_file_name = year.file_name();
        let year_string = year_file_name.to_str().context("to_str() failed")?;
        if !year.metadata()?.is_dir() {
            continue;
        }
        for month in year.path().read_dir()? {
            let month = month?;
            let month_file_name = month.file_name();
            let month_string = month_file_name.to_str().context("to_str() failed")?;
            if !month.metadata()?.is_dir() {
                continue;
            }
            if let Some(ref date_filter) = date_filter {
                if !date_filter.is_match(&format!("{}-{}", year_string, month_string)) {
                    continue;
                }
            }
            for log in month.path().read_dir()? {
                let log = log?;
                let path = log.path();
                let extension = path
                    .extension()
                    .context("extension() failed")?
                    .to_str()
                    .context("to_str() failed")?;
                if extension != "weechatlog" {
                    continue;
                }
                let file = std::fs::File::open(&path)?;
                let log_file_name = log.file_name();
                let file_name = log_file_name.to_str().context("to_str() failed")?;
                if let Some(ref channel_filter) = channel_filter {
                    if !channel_filter.is_match(file_name) {
                        continue;
                    }
                }
                let reader = std::io::BufReader::new(file);
                for line in reader.lines() {
                    let line = line?;
                    let mut columns = line.split('\t');
                    let date = columns.next().context("no date col in log file")?;
                    let from = columns.next().context("no from col in log file")?;
                    if let Some(ref from_filter) = from_filter {
                        if !from_filter.is_match(from) {
                            continue;
                        }
                    }
                    let content = columns.collect::<Vec<_>>().join("\t");
                    if let Some(ref content_filter) = content_filter {
                        if !content_filter.is_match(&content) {
                            continue;
                        }
                    }
                    results.push(format!("{file_name}:{date}\t{from}\t{content}"));
                }
            }
        }
    }

    results.sort();
    results.dedup();
    for result in results {
        println!("{result}")
    }

    Ok(())
}
