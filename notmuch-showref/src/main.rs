/*
 * Copyright 2022 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Extracts a subject and a date for a message-id, similar to e.g. `git show -s --pretty=ref
//! <commit>`.
//!
//! Usage:
//!
//! notmuch showref <message-id>
//!
//! Sample output:
//!
//! Y0VoTZCMwdHdDyiq@collabora.com (test @ 14:57, 2022-11-05)

use anyhow::Context;

#[derive(serde::Deserialize)]
struct NotmuchItem {
    timestamp: i64,
    subject: String,
}

fn main() -> anyhow::Result<()> {
    // Figure out the msgid.
    let mut args = std::env::args();
    // Ignore self name.
    args.next().context("missing self name")?;
    let msgid = args.next().context("missing msgid")?;

    let query = format!("id:{}", msgid);
    let output = std::process::Command::new("notmuch")
        .arg("search")
        .arg("--format=json")
        .arg(query)
        .output()?;

    let items: Vec<NotmuchItem> = serde_json::from_slice(&output.stdout)?;
    let item = &items[0];
    let date_time = chrono::NaiveDateTime::from_timestamp(item.timestamp, 0);
    let date = date_time.format("%Y-%m-%d").to_string();

    println!("{} ({}, {})", msgid, item.subject, date);

    Ok(())
}