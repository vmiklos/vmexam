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
use isahc::ReadResponseExt as _;
use isahc::config::Configurable as _;
use std::rc::Rc;

struct StdNetwork {}

impl nextcloud_open::Network for StdNetwork {
    fn open_browser(&self, url: &url::Url) {
        url_open::open(url);
    }

    fn send_request(
        &self,
        user: &str,
        password: &str,
        method: &str,
        url: &str,
        data: &str,
    ) -> anyhow::Result<String> {
        let client = isahc::HttpClient::builder()
            .authentication(isahc::auth::Authentication::basic())
            .credentials(isahc::auth::Credentials::new(user, password))
            .build()?;
        let request = isahc::Request::builder()
            .method(method)
            .uri(url)
            .body(data)
            .context("HTTP method failed")?;
        let mut buf = client.send(request)?;
        Ok(buf.text()?)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(clap::Parser)]
struct Arguments {
    #[clap(num_args = 1..)]
    user_paths: Vec<std::path::PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let root: vfs::VfsPath = vfs::PhysicalFS::new("/").into();
    let network = Rc::new(StdNetwork {});
    let args = Arguments::parse();
    let mut inputs: Vec<vfs::VfsPath> = Vec::new();
    for user_path in args.user_paths {
        let input = user_path
            .canonicalize()
            .with_context(|| format!("failed to canonicalize {:?}", user_path))?;
        inputs.push(root.join(input.to_string_lossy())?);
    }
    let ctx = nextcloud_open::Context::new(root, network);

    nextcloud_open::nextcloud_open(&ctx, &inputs)
}
