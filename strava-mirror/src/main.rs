/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to strava_mirror.

use anyhow::Context as _;
use isahc::RequestExt as _;
use std::collections::HashMap;
use std::rc::Rc;

/// Real network implementation, using isahc.
struct RealNetwork {}

impl strava_mirror::Network for RealNetwork {
    fn get(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> anyhow::Result<isahc::Response<isahc::Body>> {
        let mut request = isahc::Request::get(url);
        for (key, value) in headers {
            request = request.header(key, value);
        }
        let response = request.body(())?.send()?;
        Ok(response)
    }

    fn post(&self, url: &str, body: &str) -> anyhow::Result<isahc::Response<isahc::Body>> {
        let response = isahc::post(url, body)?;
        Ok(response)
    }
}

fn main() -> anyhow::Result<()> {
    let home = home::home_dir().context("home_dir() failed")?;
    let fs: vfs::VfsPath = vfs::PhysicalFS::new(home).into();
    let network = Rc::new(RealNetwork {});
    let ctx = strava_mirror::Context { fs, network };

    strava_mirror::run(std::env::args().collect(), &ctx)
}
