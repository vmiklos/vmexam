/*
 * Copyright 2019 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
*/

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

use super::*;
use isahc::config::Configurable;
use isahc::ReadResponseExt;
use isahc::RequestExt;
use std::io::IsTerminal as _;

/// Real (not test) network trait implementation.
pub struct StdNetwork {}

impl Network for StdNetwork {
    fn urlopen(&self, url: &str, data: &str) -> anyhow::Result<String> {
        if !data.is_empty() {
            let mut buf = isahc::Request::post(url)
                .redirect_policy(isahc::config::RedirectPolicy::Limit(1))
                .body(data)?
                .send()?;
            let ret = buf.text()?;
            return Ok(ret);
        }

        let mut buf = isahc::Request::get(url)
            .redirect_policy(isahc::config::RedirectPolicy::Limit(1))
            .body(())?
            .send()?;
        let ret = buf.text()?;
        Ok(ret)
    }

    fn isatty(&self) -> bool {
        std::io::stdout().is_terminal()
    }
}
