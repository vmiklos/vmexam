/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Trivial wrapper around a cmdline, sends a note about its exit code.

use anyhow::Context as _;
use std::io::Read as _;
use std::rc::Rc;

/// Network interface.
pub trait Network {
    /// Posts a JSON to an URL.
    fn post(&self, url: String, data: String) -> anyhow::Result<()>;
}

/// Process interface.
pub trait Process {
    /// Runs a command and returns its exit code.
    fn command_status(&self, command: &str, args: &[&str]) -> anyhow::Result<i32>;

    /// Get the standard host name for the current machine.
    fn get_hostname(&self) -> anyhow::Result<String>;

    /// Returns the current working directory.
    fn get_current_dir(&self) -> anyhow::Result<String>;
}

/// Time interface.
pub trait Time {
    /// Returns the current local time.
    fn now(&self) -> time::OffsetDateTime;
}

/// Abstracts away the physical filesystem, network, processes and time.
pub struct Context {
    /// File system.
    pub fs: vfs::VfsPath,
    /// Network.
    pub network: Rc<dyn Network>,
    /// Process.
    pub process: Rc<dyn Process>,
    /// Time.
    pub time: Rc<dyn Time>,
}

impl Context {
    /// Creates a new Context.
    pub fn new(
        fs: vfs::VfsPath,
        network: Rc<dyn Network>,
        process: Rc<dyn Process>,
        time: Rc<dyn Time>,
    ) -> Self {
        Context {
            fs,
            network,
            process,
            time,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Message {
    msgtype: String,
    body: String,
}

#[derive(serde::Deserialize)]
struct Config {
    access_token: String,
    room_url: String,
}

/// Main logic of pushping.
pub fn run(args: Vec<String>, ctx: &Context) -> anyhow::Result<i32> {
    let start = ctx.time.now();

    // Run the command and build a json to be sent.
    let (_, subprocess_args) = args.split_first().context("args.split_first() failed")?;
    let (first, rest) = subprocess_args.split_first().context("missing command")?;
    let rest_strs: Vec<&str> = rest.iter().map(|s| s.as_str()).collect();
    let exit_code = ctx.process.command_status(first, &rest_strs)?;
    let command = subprocess_args.join(" ");
    // passed or failed
    let result = if exit_code == 0 {
        "\u{2713}"
    } else {
        "\u{2717}"
    };
    let host = ctx.process.get_hostname()?;
    let mut working_directory = ctx.process.get_current_dir()?;
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let home_dir: String = home_dir.to_str().context("to_str() failed")?.into();
    working_directory = working_directory.replace(&home_dir, "~");
    let duration = ctx.time.now() - start;
    let seconds = duration.whole_seconds() % 60;
    let minutes = duration.whole_minutes() % 60;
    let hours = duration.whole_hours();
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
    let mut config_file = ctx
        .fs
        .join(&config_path)?
        .open_file()
        .context(format!("failed to read config from '{config_path}'"))?;
    let mut config_string = String::new();
    config_file.read_to_string(&mut config_string)?;
    let config: Config = toml::from_str(&config_string)?;
    let url = format!(
        "{}/send/m.room.message?access_token={}",
        config.room_url, config.access_token
    );
    ctx.network.post(url, json)?;

    println!("Finished in {duration}");
    Ok(exit_code)
}

#[cfg(test)]
mod tests;
