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
    fn from_regex(value: &str, ignore_case: bool, transliterate: bool) -> anyhow::Result<Self> {
        let regex = Some(
            regex::RegexBuilder::new(value)
                .case_insensitive(ignore_case)
                .build()?,
        );
        let needle = "".to_string();
        let ignore_case = false;
        Ok(Matcher {
            regex,
            needle,
            ignore_case,
            transliterate,
        })
    }

    fn from_fixed(value: &str, ignore_case: bool, transliterate: bool) -> anyhow::Result<Self> {
        let regex = None;
        let needle = if ignore_case {
            value.to_lowercase()
        } else {
            value.to_string()
        };
        Ok(Matcher {
            regex,
            needle,
            ignore_case,
            transliterate,
        })
    }

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
        if fixed_strings {
            Self::from_fixed(&needle, ignore_case, transliterate)
        } else {
            Self::from_regex(&needle, ignore_case, transliterate)
        }
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

fn our_main(
    argv: Vec<String>,
    stream: &mut dyn std::io::Write,
    fs: &vfs::VfsPath,
    time: &dyn Time,
) -> anyhow::Result<()> {
    // Parse the arguments.
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
    let from = matches.get_one::<String>("from");
    let channel = matches.get_one::<String>("channel");
    let date = matches.get_one::<String>("date");
    let content = matches.get_one::<String>("content");
    let ignore_case: bool = match matches.get_one::<bool>("ignore-case") {
        Some(value) => *value,
        None => false,
    };
    let transliterate: bool = match matches.get_one::<bool>("transliterate") {
        Some(value) => *value,
        None => false,
    };
    let fixed_strings: bool = match matches.get_one::<bool>("fixed-strings") {
        Some(value) => *value,
        None => false,
    };

    // Set up the filters.
    let from_filter = match from {
        Some(value) => Some(Matcher::new(
            value.as_str(),
            ignore_case,
            transliterate,
            fixed_strings,
        )?),
        None => None,
    };
    let channel_filter = match channel {
        Some(value) => Some(Matcher::new(
            value.as_str(),
            ignore_case,
            transliterate,
            fixed_strings,
        )?),
        None => None,
    };
    let date_filter = match date {
        Some(date) => {
            if date == "all" {
                None
            } else {
                Some(Matcher::new(
                    date.as_str(),
                    ignore_case,
                    transliterate,
                    fixed_strings,
                )?)
            }
        }
        None => {
            // Default to the current month.
            let now = time.now();
            let format = time::format_description::parse("[year]-[month]")?;
            Some(Matcher::new(
                &now.format(&format)?,
                transliterate,
                ignore_case,
                fixed_strings,
            )?)
        }
    };
    let content_filter = match content {
        Some(value) => Some(Matcher::new(
            value.as_str(),
            ignore_case,
            transliterate,
            fixed_strings,
        )?),
        None => None,
    };

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
            let month_string = month.filename();
            if month.metadata()?.file_type != vfs::VfsFileType::Directory {
                continue;
            }
            if let Some(ref date_filter) = date_filter {
                if !date_filter.is_match(&format!("{}-{}", year_string, month_string)) {
                    continue;
                }
            }
            for log in month.read_dir()? {
                let Some(extension) = log.extension() else {
                    continue;
                };
                if extension != "weechatlog" {
                    continue;
                }
                let file = log.open_file()?;
                let log_string = log.filename();
                if let Some(ref channel_filter) = channel_filter {
                    if !channel_filter.is_match(&log_string) {
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
                    results.push(format!("{log_string}:{date}\t{from}\t{content}"));
                }
            }
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
