/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Display filter for mutt.

use std::io::Write as _;

/// Try to improve input_date by wrapping a non-local date in a local one.
fn improve_date(input_date: &str) -> anyhow::Result<String> {
    let format = time::format_description::well_known::Rfc2822;
    let mut date_time = time::OffsetDateTime::parse(input_date, &format)?;
    let local_offset = time::UtcOffset::current_local_offset()?;
    if date_time.offset() == local_offset {
        return Err(anyhow::anyhow!("matching offset"));
    }
    date_time = date_time.to_offset(local_offset);
    Ok(date_time.format(&format)?)
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    let mut in_header = true;
    for line in bytelines::ByteLines::new(stdin) {
        let line = line?;
        if line.is_empty() {
            in_header = false;
        }
        if in_header && let Some(input_date) = line.strip_prefix(b"Date: ") {
            let input_date = String::from_utf8(input_date.to_vec())?;
            if let Ok(improved) = improve_date(&input_date) {
                println!("Date: {improved} ({input_date})");
                continue;
            }
        }
        stdout.write_all(&line)?;
        stdout.write_all(b"\n")?;
    }

    Ok(())
}
