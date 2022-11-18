/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
*/

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

use super::*;
use isahc::config::Configurable;
use isahc::ReadResponseExt;
use isahc::RequestExt;

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
        atty::is(atty::Stream::Stdout)
    }
}
