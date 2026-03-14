/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to json-logger.

/// Physical time implementation.
pub struct PhysicalTime {}

impl Default for PhysicalTime {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicalTime {
    /// Creates a new PhysicalTime.
    pub fn new() -> Self {
        PhysicalTime {}
    }
}

impl json_logger::Time for PhysicalTime {
    fn now(&self) -> time::OffsetDateTime {
        time::OffsetDateTime::now_local().unwrap_or_else(|_| time::OffsetDateTime::now_utc())
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let ctx = json_logger::Context::new(
        vfs::PhysicalFS::new("/").into(),
        std::rc::Rc::new(PhysicalTime::new()),
    );
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    json_logger::run(args, &ctx, &mut stdin, &mut stdout)
}
