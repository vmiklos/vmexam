/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

//! Commandline interface to addr_osmify.

use isahc::config::Configurable;
use isahc::ReadResponseExt;
use isahc::RequestExt;
use std::sync::Arc;

struct ReqwestUrllib {}

// Network traffic is intentionally mocked.
#[cfg(not(tarpaulin_include))]
impl addr_osmify::Urllib for ReqwestUrllib {
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

#[cfg(not(tarpaulin_include))]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let urllib: Arc<dyn addr_osmify::Urllib> = Arc::new(ReqwestUrllib {});
    std::process::exit(addr_osmify::main(args, &mut std::io::stdout(), &urllib))
}
