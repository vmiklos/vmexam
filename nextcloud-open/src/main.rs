/*
 * Copyright 2022 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Opens a local directory or file in nextcloud, assuming the directory is inside a sync folder.

use anyhow::Context as _;
use clap::Parser as _;

#[derive(clap::Parser)]
struct Arguments {
    user_path: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let root: vfs::VfsPath = vfs::PhysicalFS::new(home_dir.as_path()).into();
    let args = Arguments::parse();
    let input: std::path::PathBuf = args
        .user_path
        .canonicalize()
        .context(format!("failed to canonicalize {:?}", args.user_path))?;
    nextcloud_open::nextcloud_open(&root, &input)
}
