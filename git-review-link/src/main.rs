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

#[derive(Clone, serde::Deserialize)]
struct Pull {
    url: String,
    html_url: String,
}

#[derive(serde::Deserialize)]
struct Error {
    message: String,
}

fn get_first_pull(client: &isahc::HttpClient, url: &str) -> anyhow::Result<Pull> {
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
    Ok(pull.clone())
}

#[derive(serde::Deserialize)]
struct User {
    login: String,
}

#[derive(serde::Deserialize)]
struct Review {
    state: String,
    user: User,
}

fn get_approvers(client: &isahc::HttpClient, url: &str) -> anyhow::Result<Vec<String>> {
    let mut response = client.get(url)?;
    let text = response.text()?;
    let reviews: Vec<Review> = match serde_json::from_str(&text) {
        Ok(value) => value,
        Err(_) => {
            let error: Error = serde_json::from_str(&text)?;
            return Err(anyhow::anyhow!(error.message));
        }
    };

    let reviewers: Vec<_> = reviews
        .iter()
        .filter(|i| i.state == "APPROVED")
        .map(|i| i.user.login.to_string())
        .collect();
    Ok(reviewers)
}

struct Run {
    name: String,
    status: String,
    conclusion: String,
}

#[derive(serde::Deserialize)]
struct CheckRun {
    name: String,
    status: String,
    conclusion: String,
}

#[derive(serde::Deserialize)]
struct CheckRunsResponse {
    check_runs: Vec<CheckRun>,
}

fn get_check_runs(
    client: &isahc::HttpClient,
    owner_repo: &str,
    commit: &str,
    statuses: &mut Vec<Run>,
) -> anyhow::Result<()> {
    let checks_url =
        format!("https://api.github.com/repos/{owner_repo}/commits/{commit}/check-runs");
    let mut response = client.get(checks_url)?;
    let text = response.text()?;
    let check_runs_response: CheckRunsResponse = match serde_json::from_str(&text) {
        Ok(value) => value,
        Err(_) => {
            let error: Error = serde_json::from_str(&text)?;
            return Err(anyhow::anyhow!(
                "failed to fetch check-runs: {}",
                error.message
            ));
        }
    };

    for check_run in check_runs_response.check_runs {
        statuses.push(Run {
            name: check_run.name,
            status: check_run.status,
            conclusion: check_run.conclusion,
        });
    }

    Ok(())
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
    let client = isahc::HttpClient::builder()
        .default_header("Authorization", &format!("Bearer {}", config.access_token))
        .build()?;
    let pull = get_first_pull(&client, &api_url)?;
    println!("Reviewed-on: {}", pull.html_url);

    // Search for reviewers
    let reviews_url = format!("{}/reviews", pull.url);
    let approvers = get_approvers(&client, &reviews_url)?;
    for approver in approvers {
        println!("Reviewed-by: {}", approver);
    }

    let mut statuses: Vec<Run> = Vec::new();
    get_check_runs(&client, &owner_repo, &commit, &mut statuses)?;
    for status in statuses {
        println!(
            "status: name is {}, status is {}, conclusion is {}",
            status.name, status.status, status.conclusion
        );
    }

    Ok(())
}
