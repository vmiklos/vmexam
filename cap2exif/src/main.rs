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
    exif_to_filename: bool,
    filename_to_exif: bool,
    inverse: bool,
    dry_run: bool,
}

impl Arguments {
    fn parse(argv: &[String]) -> anyhow::Result<Self> {
        let exif_to_filename = clap::Arg::new("exif_to_filename")
            .long("exif-to-filename")
            .action(clap::ArgAction::SetTrue)
            .help("Rename files based on exif date, instead of writing exif.");
        let filename_to_exif = clap::Arg::new("filename_to_exif")
            .long("filename-to-exif")
            .action(clap::ArgAction::SetTrue)
            .help("Set exif date based on the filename, if it's missing.");
        let inverse = clap::Arg::new("inverse")
            .short('i')
            .long("inverse")
            .action(clap::ArgAction::SetTrue)
            .help("Read exif from images and generate captions.txt.");
        let dry_run = clap::Arg::new("dry_run")
            .short('n')
            .long("dry-run")
            .action(clap::ArgAction::SetTrue)
            .help("Print what would be done, without modifying files.");
        let args = [exif_to_filename, filename_to_exif, inverse, dry_run];
        let app = clap::Command::new("cap2exif");
        let matches = app.args(&args).try_get_matches_from(argv)?;
        let exif_to_filename = *matches
            .get_one::<bool>("exif_to_filename")
            .context("no exif_to_filename arg")?;
        let filename_to_exif = *matches
            .get_one::<bool>("filename_to_exif")
            .context("no filename_to_exif arg")?;
        let inverse = *matches
            .get_one::<bool>("inverse")
            .context("no inverse arg")?;
        let dry_run = *matches
            .get_one::<bool>("dry_run")
            .context("no dry_run arg")?;
        let mode_count = [exif_to_filename, filename_to_exif, inverse]
            .into_iter()
            .filter(|&mode| mode)
            .count();
        if mode_count > 1 {
            anyhow::bail!(
                "only one of --exif-to-filename, --filename-to-exif or --inverse can be used"
            );
        }
        if dry_run && !exif_to_filename && !filename_to_exif {
            anyhow::bail!(
                "--dry-run only works together with --exif-to-filename or --filename-to-exif"
            );
        }
        Ok(Arguments {
            exif_to_filename,
            filename_to_exif,
            inverse,
            dry_run,
        })
    }
}

fn exif_to_filename(dry_run: bool) -> anyhow::Result<()> {
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
        let exif_format = time::format_description::parse_borrowed::<2>(
            "[year]:[month]:[day] [hour]:[minute]:[second]",
        )?;
        let Ok(parsed) = time::PrimitiveDateTime::parse(&date_time, &exif_format) else {
            println!("WARNING: failed to parse {date_time:?} as a date time in {old_file_name:?}");
            continue;
        };
        // E.g. '20250725_092556.jpg'.
        let fs_format = time::format_description::parse_borrowed::<2>(
            "./[year][month][day]_[hour][minute][second].jpg",
        )?;
        let new_file_name: std::ffi::OsString = parsed.format(&fs_format)?.into();
        if old_file_name != new_file_name {
            println!("rename: {old_file_name:?} -> {new_file_name:?}");
            if !dry_run {
                std::fs::rename(old_file_name, new_file_name)?;
            }
        }
    }

    Ok(())
}

fn filename_to_exif(dry_run: bool) -> anyhow::Result<()> {
    let fs_format =
        time::format_description::parse_borrowed::<2>("[year][month][day]_[hour][minute][second]")?;
    let exif_format = time::format_description::parse_borrowed::<2>(
        "[year]:[month]:[day] [hour]:[minute]:[second]",
    )?;
    for entry in std::fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        let Some(extension) = path.extension() else {
            continue;
        };
        if extension != "jpg" && extension != "JPG" {
            continue;
        }
        let file_name = path.to_str().context("non-utf8 filename")?;
        let file_name = file_name.strip_prefix("./").unwrap_or(file_name);
        let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        // E.g. 'IMG_20170802_075657', without the optional prefix: '20170802_075657'.
        let date_part = match file_stem.split_once('_') {
            Some((prefix, rest))
                if prefix.len() == 3 && prefix.bytes().all(|b| b.is_ascii_alphabetic()) =>
            {
                rest
            }
            _ => file_stem,
        };
        if date_part.len() != 15 {
            continue;
        }
        let Ok(parsed) = time::PrimitiveDateTime::parse(date_part, &fs_format) else {
            println!("WARNING: failed to parse {date_part:?} as a date time in {file_name:?}");
            continue;
        };
        let exif_value = parsed.format(&exif_format)?;
        let meta = rexiv2::Metadata::new_from_path(file_name)?;
        let mut changed = false;
        if meta.get_tag_string("Exif.Image.DateTime").is_err() {
            meta.set_tag_string("Exif.Image.DateTime", &exif_value)?;
            changed = true;
        }
        if meta.get_tag_string("Exif.Photo.DateTimeOriginal").is_err() {
            meta.set_tag_string("Exif.Photo.DateTimeOriginal", &exif_value)?;
            changed = true;
        }
        if changed {
            println!("update: {file_name:?}");
            if !dry_run {
                // E.g. '2025:07:14 22:27:39'.
                meta.save_to_file(file_name)?;
            }
        }
    }

    Ok(())
}

fn inverse() -> anyhow::Result<()> {
    let mut lines: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        let Some(extension) = path.extension() else {
            continue;
        };
        if extension != "jpg" && extension != "JPG" {
            continue;
        }
        let file_name = path.to_str().context("non-utf8 filename")?;
        let file_name = file_name.strip_prefix("./").unwrap_or(file_name);
        let meta = rexiv2::Metadata::new_from_path(file_name)?;
        let caption = meta
            .get_tag_multiple_strings("Xmp.dc.title")
            .ok()
            .and_then(|titles| titles.into_iter().next())
            .filter(|s| !s.is_empty())
            .or_else(|| {
                meta.get_tag_string("Exif.Photo.UserComment")
                    .ok()
                    .filter(|s| !s.is_empty())
            });
        let line = match caption {
            Some(caption) => format!("{file_name}\t{caption}\n"),
            None => format!("{file_name}\t\n"),
        };
        lines.push(line);
    }
    lines.sort();
    std::fs::write("captions.txt", lines.concat())?;
    Ok(())
}

fn caption_matches(meta: &rexiv2::Metadata, caption: &str) -> bool {
    let user_comment_matches = meta
        .get_tag_string("Exif.Photo.UserComment")
        .ok()
        .is_some_and(|value| value == caption);
    let title_matches = meta
        .get_tag_multiple_strings("Xmp.dc.title")
        .ok()
        .is_some_and(|values| values.iter().any(|value| value == caption));
    user_comment_matches && title_matches
}

fn main() -> anyhow::Result<()> {
    let argv: Vec<String> = std::env::args().collect();
    let args = Arguments::parse(&argv)?;

    rexiv2::initialize()?;

    if args.exif_to_filename {
        return exif_to_filename(args.dry_run);
    }

    if args.filename_to_exif {
        return filename_to_exif(args.dry_run);
    }

    if args.inverse {
        return inverse();
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
        if caption_matches(&meta, caption) {
            // Tags are already set here, so no need to rewrite the file.
            continue;
        }
        meta.set_tag_string("Exif.Photo.UserComment", caption)?;
        meta.set_tag_string("Xmp.dc.title", caption)?;
        meta.save_to_file(path)?;
    }

    Ok(())
}
