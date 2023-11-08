/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

//! Commandline interface to weesearch.

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

/// Time implementation, backed by the the actual time.
pub struct StdTime {}

// Real time is intentionally mocked.
impl weesearch::Time for StdTime {
    fn now(&self) -> time::OffsetDateTime {
        let now = time::OffsetDateTime::now_utc();
        let tz_offset = time::UtcOffset::current_local_offset().unwrap();
        now.to_offset(tz_offset)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let root: vfs::VfsPath = vfs::PhysicalFS::new("/").into();
    let time = StdTime {};
    std::process::exit(weesearch::main(args, &mut std::io::stdout(), &root, &time))
}
