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
use std::io::BufRead as _;

/// Time interface.
pub trait Time {
    /// Calculates the current time.
    fn now(&self) -> time::OffsetDateTime;
}

/// Regex or fixed string matcher.
struct Matcher {
    /// If this is Some, a regex match is performed.
    regex: Option<regex::Regex>,
    /// For the fixed string case.
    needle: String,
    /// Case-insensitive mode for the fixed string.
    ignore_case: bool,
    /// Ignore accents.
    transliterate: bool,
}

impl Matcher {
    fn new(
        needle: &str,
        ignore_case: bool,
        transliterate: bool,
        fixed_strings: bool,
    ) -> anyhow::Result<Self> {
        let needle = if transliterate {
            unidecode::unidecode(needle)
        } else {
            needle.to_string()
        };
        let regex = if fixed_strings {
            None
        } else {
            Some(
                regex::RegexBuilder::new(&needle)
                    .case_insensitive(ignore_case)
                    .build()?,
            )
        };
        let needle = if ignore_case {
            needle.to_lowercase()
        } else {
            needle.to_string()
        };
        Ok(Matcher {
            regex,
            needle,
            ignore_case,
            transliterate,
        })
    }

    fn is_match(&self, haystack: &str) -> bool {
        let haystack = if self.transliterate {
            unidecode::unidecode(haystack)
        } else {
            haystack.to_string()
        };
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

struct Arguments {
    from: Option<String>,
    channel: Option<String>,
    date: Option<String>,
    content: Option<String>,
    ignore_case: bool,
    transliterate: bool,
    fixed_strings: bool,
}

impl Arguments {
    fn parse(argv: &[String]) -> anyhow::Result<Self> {
        let from_arg = clap::Arg::new("from")
            .short('f')
            .long("from")
            .required(false)
            .help("Sender of the message (regex)");
        let channel_arg = clap::Arg::new("channel")
            .short('c')
            .long("channel")
            .required(false)
            .help("Name of the channel where the message appeared (regex)");
        let date_arg = clap::Arg::new("date")
        .short('d')
        .long("date")
        .required(false)
        .help("Date in a YYYY-MM form (regex), defaults to the current month, 'all' disables the filter");
        let content_arg = clap::Arg::new("content")
            .index(1)
            .required(false)
            .help("The content of the message (regex)");
        let ignore_case_arg = clap::Arg::new("ignore-case")
            .short('i')
            .long("ignore-case")
            .action(clap::ArgAction::SetTrue)
            .help("Case-insensitive mode, disabled by default");
        let transliterate_arg = clap::Arg::new("transliterate")
            .short('t')
            .long("transliterate")
            .action(clap::ArgAction::SetTrue)
            .help("Ignore accents using transliteration, disabled by default");
        let fixed_strings_arg = clap::Arg::new("fixed-strings")
            .short('F')
            .long("fixed-strings")
            .action(clap::ArgAction::SetTrue)
            .help("Interpret filters as a fixed string (instead of a regular expression)");
        let args = [
            from_arg,
            channel_arg,
            date_arg,
            content_arg,
            ignore_case_arg,
            transliterate_arg,
            fixed_strings_arg,
        ];
        let app = clap::Command::new("weesearch");
        let matches = app.args(&args).try_get_matches_from(argv)?;
        let from = matches.get_one::<String>("from").cloned();
        let channel = matches.get_one::<String>("channel").cloned();
        let date = matches.get_one::<String>("date").cloned();
        let content = matches.get_one::<String>("content").cloned();
        let ignore_case = *matches
            .get_one::<bool>("ignore-case")
            .context("no ignore-case arg")?;
        let transliterate = *matches
            .get_one::<bool>("transliterate")
            .context("no transliterate arg")?;
        let fixed_strings = *matches
            .get_one::<bool>("fixed-strings")
            .context("no fixed-strings arg")?;
        Ok(Arguments {
            from,
            channel,
            date,
            content,
            ignore_case,
            transliterate,
            fixed_strings,
        })
    }
}

struct Filters {
    from: Option<Matcher>,
    channel: Option<Matcher>,
    date: Option<Matcher>,
    content: Option<Matcher>,
}

impl Filters {
    fn new(args: &Arguments, time: &dyn Time) -> anyhow::Result<Self> {
        let from = match args.from {
            Some(ref value) => Some(Matcher::new(
                value.as_str(),
                args.ignore_case,
                args.transliterate,
                args.fixed_strings,
            )?),
            None => None,
        };
        let channel = match args.channel {
            Some(ref value) => Some(Matcher::new(
                value.as_str(),
                args.ignore_case,
                args.transliterate,
                args.fixed_strings,
            )?),
            None => None,
        };
        let date = match args.date {
            Some(ref date) => {
                if date == "all" {
                    None
                } else {
                    Some(Matcher::new(
                        date.as_str(),
                        args.ignore_case,
                        args.transliterate,
                        args.fixed_strings,
                    )?)
                }
            }
            None => {
                // Default to the current month.
                let now = time.now();
                let format = time::format_description::parse("[year]-[month]")?;
                let needle = &now.format(&format)?;
                Some(Matcher::new(
                    needle,
                    args.transliterate,
                    args.ignore_case,
                    args.fixed_strings,
                )?)
            }
        };
        let content = match args.content {
            Some(ref value) => Some(Matcher::new(
                value.as_str(),
                args.ignore_case,
                args.transliterate,
                args.fixed_strings,
            )?),
            None => None,
        };
        Ok(Filters {
            from,
            channel,
            date,
            content,
        })
    }
}

/// Handles one line in a channel log.
fn handle_line(
    log_string: &str,
    line: &str,
    from_filter: &Option<Matcher>,
    content_filter: &Option<Matcher>,
) -> anyhow::Result<Option<String>> {
    let mut columns = line.split('\t');
    let date = columns.next().context("no date col in log file")?;
    let from = columns.next().context("no from col in log file")?;
    if let Some(ref from_filter) = from_filter {
        if !from_filter.is_match(from) {
            return Ok(None);
        }
    }
    let content = columns.collect::<Vec<_>>().join("\t");
    if let Some(ref content_filter) = content_filter {
        if !content_filter.is_match(&content) {
            return Ok(None);
        }
    }
    Ok(Some(format!("{log_string}:{date}\t{from}\t{content}")))
}

/// Handles one channel in a month.
fn handle_channel(log: &vfs::VfsPath, filters: &Filters) -> anyhow::Result<Vec<String>> {
    let mut results = Vec::new();
    let Some(extension) = log.extension() else {
        return Ok(results);
    };
    if extension != "weechatlog" {
        return Ok(results);
    }
    let file = log.open_file()?;
    let mut log_string = log.as_str().to_string();
    let current_dir = std::env::current_dir()?;
    let mut working_directory: String = current_dir.to_str().context("to_str() failed")?.into();
    working_directory += "/";
    log_string = log_string.replace(&working_directory, "");
    if let Some(ref channel_filter) = filters.channel {
        if !channel_filter.is_match(&log_string) {
            return Ok(results);
        }
    }
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let Some(result) = handle_line(&log_string, &line?, &filters.from, &filters.content)?
        else {
            continue;
        };
        results.push(result);
    }
    Ok(results)
}

/// Handles one month in a year.
fn handle_month(
    year: &str,
    month: &vfs::VfsPath,
    filters: &Filters,
) -> anyhow::Result<Vec<String>> {
    let mut results = Vec::new();
    let month_string = month.filename();
    if month.metadata()?.file_type != vfs::VfsFileType::Directory {
        return Ok(results);
    }
    if let Some(ref date_filter) = filters.date {
        if !date_filter.is_match(&format!("{}-{}", year, month_string)) {
            return Ok(results);
        }
    }
    for log in month.read_dir()? {
        results.append(&mut handle_channel(&log, filters)?);
    }
    Ok(results)
}

fn our_main(
    argv: Vec<String>,
    stream: &mut dyn std::io::Write,
    fs: &vfs::VfsPath,
    time: &dyn Time,
) -> anyhow::Result<()> {
    // Parse the arguments.
    let args = Arguments::parse(&argv)?;

    // Set up the filters.
    let filters = Filters::new(&args, time)?;

    // Search.
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let home_dir: String = home_dir.to_str().context("to_str() failed")?.into();
    let years = format!("{home_dir}/.local/share/weechat/logs");
    let mut results = Vec::new();
    for year in fs.join(years)?.read_dir()? {
        let year_string = year.filename();
        if year.metadata()?.file_type != vfs::VfsFileType::Directory {
            continue;
        }
        for month in year.read_dir()? {
            results.append(&mut handle_month(&year_string, &month, &filters)?);
        }
    }

    results.sort();
    results.dedup();
    for result in results {
        stream.write_all(format!("{result}\n").as_bytes())?;
    }

    Ok(())
}

/// Similar to plain main(), but with an interface that allows testing.
pub fn main(
    args: Vec<String>,
    stream: &mut dyn std::io::Write,
    fs: &vfs::VfsPath,
    time: &dyn Time,
) -> i32 {
    match our_main(args, stream, fs, time) {
        Ok(_) => 0,
        Err(err) => {
            stream.write_all(format!("{:?}\n", err).as_bytes()).unwrap();
            1
        }
    }
}

#[cfg(test)]
mod tests;
