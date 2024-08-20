/*
 * Copyright 2024 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Calculates some simple stats on a markdown checklist, writing the result in make(1) format.

use anyhow::Context as _;
use std::io::BufRead as _;
use std::io::Write as _;

fn main() -> anyhow::Result<()> {
    let mut iter = std::env::args();
    iter.next().context("no self arg")?;
    let markdown_path = iter.next().context("no markdown input")?;
    let make_path = iter.next().context("no make output")?;
    let markdown_file = std::fs::File::open(markdown_path).context("can't open markdown input")?;
    let markdown_reader = std::io::BufReader::new(markdown_file);
    let mut checkmark_todo = 0;
    let mut checkmark_done = 0;
    for line in markdown_reader.lines() {
        let line = line?;
        if line.starts_with("- [ ] ") {
            checkmark_todo += 1;
        } else if line.starts_with("- [x] ") {
            checkmark_done += 1;
        }
    }

    let mut make_file = std::fs::File::create(make_path).context("can't create make output")?;
    writeln!(make_file, "CHECKMARK_DONE = {}", checkmark_done)?;
    let checkmark_total = checkmark_todo + checkmark_done;
    writeln!(make_file, "CHECKMARK_TOTAL = {}", checkmark_total)?;
    writeln!(
        make_file,
        "CHECKMARK_PROGRESS = {0:.2}",
        checkmark_done as f64 / checkmark_total as f64 * 100.0
    )?;

    Ok(())
}
