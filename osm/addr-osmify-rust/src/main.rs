/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#![warn(missing_docs)]

//! Commandline interface to addr_osmify.

extern crate reqwest;

struct ReqwestUrllib {}

// Network traffic is intentionally mocked.
#[cfg(not(tarpaulin_include))]
impl addr_osmify::Urllib for ReqwestUrllib {
    fn urlopen(&self, url: &str, data: &str) -> addr_osmify::BoxResult<String> {
        if !data.is_empty() {
            let client = reqwest::blocking::Client::new();
            let body = String::from(data);
            let buf = client.post(url).body(body).send()?;
            return Ok(buf.text()?);
        }

        let buf = reqwest::blocking::get(url)?;

        Ok(buf.text()?)
    }
}

#[cfg(not(tarpaulin_include))]
fn main() -> addr_osmify::BoxResult<()> {
    let args: Vec<String> = std::env::args().collect();
    let urllib: Box<dyn addr_osmify::Urllib> = Box::new(ReqwestUrllib {});
    addr_osmify::main(args, &mut std::io::stdout(), urllib)?;

    Ok(())
}
