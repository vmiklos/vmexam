/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Provides the 'github-outdated' cmdline tool.

use anyhow::Context as _;
use isahc::ReadResponseExt as _;

#[derive(serde::Deserialize)]
struct Release {
    draft: bool,
    prerelease: bool,
    tag_name: String,
}

#[derive(serde::Deserialize)]
struct Config {
    access_token: String,
}

fn handle_action(config: &Config, job: &str, action: &str) -> anyhow::Result<()> {
    let mut tokens = action.split('@');
    let repo = tokens.next().context("next failed")?;
    let actual_version = tokens.next().context("next failed")?;
    let url = format!("https://api.github.com/repos/{repo}/releases");
    let client = isahc::HttpClient::builder()
        .default_header("Authorization", &format!("Bearer {}", config.access_token))
        .build()?;
    let mut response = client.get(&url)?;
    let text = response.text()?;
    let releases: Vec<Release> = serde_json::from_str(&text)?;
    for release in releases {
        if release.draft || release.prerelease {
            continue;
        }

        let expected_version = release.tag_name;
        if actual_version != expected_version {
            println!(
                "Job name: {job}, action name: {repo}, project version: {actual_version}, latest version: {expected_version}"
            );
        } else {
            println!("Job name: {job}, action name: {repo}, up to date");
        }
        break;
    }

    Ok(())
}

fn handle_rust(
    job: &str,
    parameters: &std::collections::HashMap<String, String>,
) -> anyhow::Result<()> {
    for (key, value) in parameters {
        if key == "toolchain" {
            let actual_version = value;
            let args = ["rustc", "--version"];
            let (first, rest) = args
                .split_first()
                .ok_or_else(|| anyhow::anyhow!("args is an empty list"))?;
            let output = std::process::Command::new(first).args(rest).output()?;
            let stdout = std::str::from_utf8(&output.stdout)?.to_string();
            let mut tokens = stdout.split(' ');
            let _name = tokens.next().context("next failed")?;
            let expected_version = tokens.next().context("next failed")?;
            if actual_version != expected_version {
                println!(
                    "Job name: {job}, rust version: {actual_version}, latest version: {expected_version}"
                );
            } else {
                println!("Job name: {job}, rust version is up to date");
            }
        }
    }
    Ok(())
}

#[derive(serde::Deserialize)]
struct Step {
    uses: Option<String>,
    with: Option<std::collections::HashMap<String, String>>,
}

#[derive(serde::Deserialize)]
struct Job {
    steps: Vec<Step>,
}

#[derive(serde::Deserialize)]
struct Workflow {
    jobs: std::collections::HashMap<String, Job>,
}

fn main() -> anyhow::Result<()> {
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let home_dir: String = home_dir.to_str().context("to_str() failed")?.into();
    let config_path = home_dir + "/.config/github-outdatedrc";
    let config_string = std::fs::read_to_string(&config_path)
        .context(format!("failed to read config from '{config_path}'"))?;
    let config: Config = toml::from_str(&config_string)?;

    let entries = std::fs::read_dir(".github/workflows")?;
    for entry in entries {
        let path = entry?.path();
        let extension = path.extension().context("no extension")?;
        if extension != "yml" {
            continue;
        }
        let data = std::fs::read_to_string(path)?;
        let workflow: Workflow = serde_yaml::from_str(&data)?;
        for (job_name, job) in workflow.jobs {
            for step in job.steps {
                if let Some(action) = step.uses {
                    handle_action(&config, &job_name, &action)?;

                    if action.starts_with("dtolnay/rust-toolchain") {
                        if let Some(with) = step.with {
                            handle_rust(&job_name, &with)?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
