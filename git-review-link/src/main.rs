/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Provides the 'git-review-link' cmdline tool.

use anyhow::Context as _;
use clap::Parser as _;
use isahc::ReadResponseExt as _;

#[derive(clap::Parser)]
struct Arguments {
    /// Hash of the git commit.
    commit: String,
}

#[derive(serde::Deserialize)]
struct Config {
    access_token: String,
}

fn get_owner_repo() -> anyhow::Result<String> {
    let content = std::fs::read_to_string(".git/config").context("can't open .git/config")?;
    for line in content.lines() {
        if line.contains("github.com") {
            let mut it = line.split(":");
            return Ok(it.nth(1).context("no ':' in URL")?.to_string());
        }
    }

    Err(anyhow::anyhow!(
        "github owner/repo is not found in .git/config"
    ))
}

#[derive(serde::Deserialize)]
struct Pull {
    html_url: String,
}

#[derive(serde::Deserialize)]
struct Error {
    message: String,
}

fn get_first_pull(config: &Config, url: &str) -> anyhow::Result<String> {
    let client = isahc::HttpClient::builder()
        .default_header("Authorization", &format!("Bearer {}", config.access_token))
        .build()?;
    let mut response = client.get(url)?;
    let text = response.text()?;
    let pulls: Vec<Pull> = match serde_json::from_str(&text) {
        Ok(value) => value,
        Err(_) => {
            let error: Error = serde_json::from_str(&text)?;
            return Err(anyhow::anyhow!(error.message));
        }
    };
    let pull = match pulls.first() {
        Some(value) => value,
        None => {
            return Err(anyhow::anyhow!("No pull request found for this commit"));
        }
    };
    Ok(pull.html_url.to_string())
}

fn main() -> anyhow::Result<()> {
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let home_dir: String = home_dir.to_str().context("to_str() failed")?.into();
    let config_path = home_dir + "/.config/git-review-linkrc";
    let config_string = std::fs::read_to_string(&config_path)
        .context(format!("failed to read config from '{config_path}'"))?;
    let config: Config = toml::from_str(&config_string)?;
    let args = Arguments::parse();
    let owner_repo = get_owner_repo().context("failed to determine owner/repo")?;
    let commit = args.commit;
    let api_url = format!("https://api.github.com/repos/{owner_repo}/commits/{commit}/pulls");
    let pull = get_first_pull(&config, &api_url)?;
    println!("{pull}");

    Ok(())
}
