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
use std::rc::Rc;

struct StdNetwork {}

impl nextcloud_open::Network for StdNetwork {
    fn open_browser(&self, url: &url::Url) {
        url_open::open(url);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(clap::Parser)]
struct Arguments {
    user_path: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let root: vfs::VfsPath = vfs::PhysicalFS::new("/").into();
    let network = Rc::new(StdNetwork {});
    let ctx = nextcloud_open::Context::new(root, network);
    let args = Arguments::parse();
    let input: std::path::PathBuf = args
        .user_path
        .canonicalize()
        .context(format!("failed to canonicalize {:?}", args.user_path))?;

    nextcloud_open::nextcloud_open(&ctx, &input)
}
