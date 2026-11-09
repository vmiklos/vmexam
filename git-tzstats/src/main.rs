/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to git-tzstats.

use anyhow::Context as _;
use time_tz::Offset as _;
use time_tz::TimeZone as _;

/// Context implementation, backed by library calls.
struct StdContext {}

impl git_tzstats::Context for StdContext {
    fn command_output(&self, command: &str, args: &[&str]) -> anyhow::Result<String> {
        let output = std::process::Command::new(command).args(args).output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("{command} failed: {stderr}"));
        }
        String::from_utf8(output.stdout).context("from_utf8() failed")
    }

    fn env_args(&self) -> Vec<String> {
        std::env::args().collect()
    }

    fn print(&self, string: &str) {
        print!("{string}");
    }
}

/// Time implementation, backed by the system timezone.
struct StdTime {
    timezone: &'static time_tz::Tz,
}

impl git_tzstats::Time for StdTime {
    fn local_offset(&self, date_time: time::OffsetDateTime) -> anyhow::Result<time::UtcOffset> {
        Ok(self.timezone.get_offset_utc(&date_time).to_utc())
    }
}

fn main() -> anyhow::Result<()> {
    let timezone = time_tz::system::get_timezone().context("failed to get the local timezone")?;
    let ctx = StdContext {};
    let time = StdTime { timezone };
    git_tzstats::main(&ctx, &time)
}
