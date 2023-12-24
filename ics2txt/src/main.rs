/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

//! Commandline interface to ics2txt.

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! An ICS printer for mutt with detailed time info.

/// Time implementation, backed by the the actual time.
pub struct StdTime {}

// Real time is intentionally mocked.
impl ics2txt::Time for StdTime {
    fn current_local_offset(&self) -> anyhow::Result<time::UtcOffset> {
        Ok(time::UtcOffset::current_local_offset()?)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let time = StdTime {};
    std::process::exit(ics2txt::main(args, &mut std::io::stdout(), &time))
}
