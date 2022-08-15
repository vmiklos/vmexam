/*
 * Copyright 2022 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Trivial wrapper around a cmdline, sends a note about its exit code.
//! Config file:
//!
//! access_token='...'
//! room_url='https://server.example.com:8448/_matrix/client/r0/rooms/!roomhash:example.com'
//!
//! Create the access token using:
//! curl -X POST -d '{"type":"m.login.password", "user":"...", "password":"..."}' "https://server.example.com:8448/_matrix/client/r0/login"
//!
//! The 'pushping' name refers to <https://www.pushbullet.com/>, which provides something similar, but
//! not with your self-hosted matrix instance.

use anyhow::Context as _;
use isahc::RequestExt as _;

#[derive(serde::Serialize)]
struct Message {
    msgtype: String,
    body: String,
}

#[derive(serde::Deserialize)]
struct Config {
    access_token: String,
    room_url: String,
}

fn main() -> anyhow::Result<()> {
    // Run the command and build a json to be sent.
    let args: Vec<String> = std::env::args().collect();
    let (_, subprocess_args) = args.split_first().context("args.split_first() failed")?;
    let (first, rest) = subprocess_args
        .split_first()
        .context("subprocess_args.split_first() failed")?;
    let exit_status = std::process::Command::new(first).args(rest).status()?;
    let exit_code = exit_status.code().context("code() failed")?;
    let command = subprocess_args.join(" ");
    let result = if exit_code == 0 { "passed" } else { "failed" };
    let hostname = gethostname::gethostname();
    let host = hostname.to_str().context("to_str() failed")?;
    let current_dir = std::env::current_dir()?;
    let mut working_directory: String = current_dir.to_str().context("to_str() failed")?.into();
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let home_dir: String = home_dir.to_str().context("to_str() failed")?.into();
    working_directory = working_directory.replace(&home_dir, "~");
    let body = format!(
        "{}: {}, host: {}, working directory: {}, exit code: {}",
        command, result, host, working_directory, exit_code
    );
    let payload = Message {
        msgtype: "m.text".into(),
        body,
    };
    let json = serde_json::to_string(&payload)?;

    // Send the json to a URL based on a config.
    let config: Config =
        toml::from_str(&std::fs::read_to_string(home_dir + "/.config/pushpingrc")?).unwrap();
    let url = format!(
        "{}/send/m.room.message?access_token={}",
        config.room_url, config.access_token
    );
    isahc::Request::post(url).body(json)?.send()?;

    std::process::exit(exit_code);
}
