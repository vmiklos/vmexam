/*
 * Copyright 2022 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Trivial wrapper around a cmdline, sends a note about its exit code.

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
    let start = chrono::Local::now();

    // Run the command and build a json to be sent.
    let args: Vec<String> = std::env::args().collect();
    let (_, subprocess_args) = args.split_first().context("args.split_first() failed")?;
    let (first, rest) = subprocess_args.split_first().context("missing command")?;
    let exit_status = std::process::Command::new(first)
        .args(rest)
        .status()
        .context("failed to execute the command as a child process")?;
    let exit_code = exit_status.code().context("code() failed")?;
    let command = subprocess_args.join(" ");
    // passed or failed
    let result = if exit_code == 0 {
        "\u{2713}"
    } else {
        "\u{2717}"
    };
    let hostname = gethostname::gethostname();
    let host = hostname.to_str().context("to_str() failed")?;
    let current_dir = std::env::current_dir()?;
    let mut working_directory: String = current_dir.to_str().context("to_str() failed")?.into();
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let home_dir: String = home_dir.to_str().context("to_str() failed")?.into();
    working_directory = working_directory.replace(&home_dir, "~");
    let duration = chrono::Local::now() - start;
    let seconds = duration.num_seconds() % 60;
    let minutes = duration.num_minutes() % 60;
    let hours = duration.num_hours();
    let duration = format!("{hours}:{minutes:0>2}:{seconds:0>2}");
    let body = format!(
        "{result} {host}:{working_directory}$ {command}: exit code is {exit_code}, finished in {duration}"
    );
    let payload = Message {
        msgtype: "m.text".into(),
        body,
    };
    let json = serde_json::to_string(&payload)?;

    // Send the json to a URL based on a config.
    let config_path = home_dir + "/.config/pushpingrc";
    let config_string = std::fs::read_to_string(&config_path)
        .context(format!("failed to read config from '{config_path}'"))?;
    let config: Config = toml::from_str(&config_string)?;
    let url = format!(
        "{}/send/m.room.message?access_token={}",
        config.room_url, config.access_token
    );
    isahc::Request::post(url).body(json)?.send()?;

    println!("Finished in {duration}");
    std::process::exit(exit_code);
}
