/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to cap2exif.

use anyhow::Context as _;

fn main() -> anyhow::Result<()> {
    rexiv2::initialize()?;

    let content = std::fs::read_to_string("captions.txt")?;

    for line in content.lines() {
        let mut tokens = line.split('\t');
        let path = tokens.next().context("no filename")?;
        let caption = tokens.next().context("no caption")?;
        let meta = rexiv2::Metadata::new_from_path(path)?;
        meta.set_tag_string("Exif.Photo.UserComment", caption)?;
        meta.set_tag_string("Xmp.dc.title", caption)?;
        meta.save_to_file(path)?;
    }

    Ok(())
}
