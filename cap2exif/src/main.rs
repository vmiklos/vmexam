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

struct Arguments {
    rename: bool,
}

impl Arguments {
    fn parse(argv: &[String]) -> anyhow::Result<Self> {
        let rename = clap::Arg::new("rename")
            .short('r')
            .long("rename")
            .action(clap::ArgAction::SetTrue)
            .help("Rename files based on exif date, instead of writing exif.");
        let args = [rename];
        let app = clap::Command::new("cap2exif");
        let matches = app.args(&args).try_get_matches_from(argv)?;
        let rename = *matches.get_one::<bool>("rename").context("no rename arg")?;
        Ok(Arguments { rename })
    }
}

fn rename() -> anyhow::Result<()> {
    for entry in std::fs::read_dir(".")? {
        let entry = entry?;
        let old_path = entry.path();
        let Some(old_extension) = old_path.extension() else {
            continue;
        };
        if old_extension != "jpg" && old_extension != "JPG" {
            continue;
        }
        let old_file_name = old_path.into_os_string();
        let meta = rexiv2::Metadata::new_from_path(&old_file_name)?;
        let Ok(date_time) = meta.get_tag_string("Exif.Image.DateTime") else {
            println!("WARNING: no Exif.Image.DateTime in {old_file_name:?}");
            continue;
        };
        // E.g. '2025:07:14 22:27:39'.
        let exif_format =
            time::format_description::parse("[year]:[month]:[day] [hour]:[minute]:[second]")?;
        let Ok(parsed) = time::PrimitiveDateTime::parse(&date_time, &exif_format) else {
            println!("WARNING: failed to parse {date_time:?} as a date time in {old_file_name:?}");
            continue;
        };
        // E.g. '20250725_092556.jpg'.
        let fs_format =
            time::format_description::parse("./[year][month][day]_[hour][minute][second].jpg")?;
        let new_file_name: std::ffi::OsString = parsed.format(&fs_format)?.into();
        if old_file_name != new_file_name {
            println!("rename: {old_file_name:?} -> {new_file_name:?}");
            std::fs::rename(old_file_name, new_file_name)?;
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let argv: Vec<String> = std::env::args().collect();
    let args = Arguments::parse(&argv)?;

    rexiv2::initialize()?;

    if args.rename {
        return rename();
    }

    let content = std::fs::read_to_string("captions.txt").context("can't open captions.txt")?;

    for line in content.lines() {
        if line.starts_with("#") {
            // This line is a comment, ignore.
            continue;
        }

        let mut tokens = line.split('\t');
        let path = tokens.next().context("no filename")?;
        let Some(caption) = tokens.next() else {
            // No caption, ignore.
            continue;
        };
        let meta = rexiv2::Metadata::new_from_path(path)?;
        meta.set_tag_string("Exif.Photo.UserComment", caption)?;
        meta.set_tag_string("Xmp.dc.title", caption)?;
        meta.save_to_file(path)?;
    }

    Ok(())
}
