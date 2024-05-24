/*
 * Copyright 2024 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Tool to list projects (Rust packages as a start) in a Git repo.

use anyhow::Context as _;

#[derive(serde::Deserialize)]
struct Config {
    projects: Vec<String>,
}

struct Project {
    dir: std::path::PathBuf,
    config: std::path::PathBuf,
}

#[derive(serde::Deserialize)]
struct GitDate {
    #[serde(with = "time::serde::iso8601")]
    ci: time::OffsetDateTime,
    cr: String,
}

fn main() -> anyhow::Result<()> {
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let config_path = home_dir.join(".config").join("git-ls-projects.toml");
    let config_string = std::fs::read_to_string(&config_path)
        .context(format!("failed to read config from '{config_path:?}'"))?;
    let config: Config = toml::from_str(&config_string)?;
    let mut projects = Vec::new();
    for project in config.projects {
        let project_config = project + "/Cargo.toml";
        for result in glob::glob(&project_config)? {
            let entry = result?;
            let dir = entry.parent().context("no parent")?.to_path_buf();
            let config = entry.clone();
            let project = Project { dir, config };
            projects.push(project);
        }
    }

    let now = time::OffsetDateTime::now_utc().to_offset(time::UtcOffset::current_local_offset()?);
    for project in projects {
        let dir = project.dir.to_str().context("no str")?;
        let config = project
            .config
            .file_name()
            .context("no file name")?
            .to_str()
            .context("no str")?;
        let args = [
            "-C",
            dir,
            "log",
            "--date=relative",
            r#"--pretty=format:{"ci": "%cI", "cr": "%cr"}"#,
            "-1",
            "--",
            config,
        ];
        let output = std::process::Command::new("git").args(args).output()?;
        let date: GitDate =
            serde_json::from_slice(output.stdout.as_slice()).context("failed to parse json")?;
        let duration = now - date.ci;
        if duration.whole_seconds() > 365 * 24 * 60 * 60 {
            println!("{}: {}", dir, date.cr);
        }
    }

    Ok(())
}
