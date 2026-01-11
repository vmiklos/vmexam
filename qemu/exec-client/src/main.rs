/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to exec-client.

use anyhow::Context as _;
use isahc::ReadResponseExt as _;
use isahc::RequestExt as _;
use isahc::config::Configurable as _;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct InnerConfig {
    guest_ip: String,
    drive_letter: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
    qemu_exec_client: InnerConfig,
}

#[derive(serde::Serialize)]
struct Payload {
    command: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    sync: Option<String>,
}

/// Command, sync.
type AliasValue = (String, bool);

lazy_static! {
    static ref ALIASES: HashMap<String, AliasValue> = {
        let mut ret: HashMap<String, AliasValue> = HashMap::new();
        ret.insert(
            "acroread".into(),
            (
                "c:/program files/adobe/acrobat dc/acrobat/acrobat.exe".into(),
                false,
            ),
        );
        ret.insert(
            "excel".into(),
            (
                "c:/program files/microsoft office/root/office16/excel.exe".into(),
                false,
            ),
        );
        ret.insert(
            "powerpnt".into(),
            (
                "c:/program files/microsoft office/root/office16/powerpnt.exe".into(),
                false,
            ),
        );
        ret.insert(
            "winword".into(),
            (
                "c:/program files/microsoft office/root/office16/winword.exe".into(),
                false,
            ),
        );
        ret.insert(
            "mso-convert-tool".into(),
            (
                "c:/program files/mso-convert-tool/mso-convert-tool.exe".into(),
                true,
            ),
        );
        ret
    };
}

/// Parses the config.
fn get_config() -> anyhow::Result<Config> {
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let home_dir: String = home_dir.to_str().context("to_str() failed")?.into();
    let config_path = home_dir + "/.config/qemu-exec-clientrc";
    let config_string = std::fs::read_to_string(&config_path)
        .context(format!("failed to read config from '{config_path}'"))?;
    let config: Config = toml::from_str(&config_string)?;
    Ok(config)
}

/// Map from host path to guest path.
fn host_to_guest(argv: &[String], drive_letter: &str) -> anyhow::Result<Vec<String>> {
    let mut abs_argv: Vec<String> = Vec::new();
    for arg in argv.iter() {
        let path = std::path::Path::new(arg);
        if path.exists() {
            if path.is_absolute() {
                abs_argv.push(arg.to_string());
            } else {
                let absolute = std::path::absolute(arg)?;
                abs_argv.push(absolute.to_str().context("to_str() failed")?.to_string());
            }
        } else {
            abs_argv.push(arg.to_string());
        }
    }
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let home_dir: String = home_dir.to_str().context("to_str() failed")?.into();
    let fro = home_dir + "/git";
    abs_argv = abs_argv
        .iter()
        .map(|i| i.replace(&fro, drive_letter))
        .collect();
    abs_argv = abs_argv.iter().map(|i| i.replace("/", r#"\"#)).collect();
    Ok(abs_argv)
}

fn main() -> anyhow::Result<()> {
    let config = get_config()?;
    let guest_ip = config.qemu_exec_client.guest_ip;

    // If my name is an alias, inject it.
    let mut argv: Vec<_> = std::env::args().collect();
    let my_name = argv[0].to_string();
    argv = argv.iter().skip(1).cloned().collect();
    let mut command: String = "".into();
    let mut sync = false;
    for (key, value) in ALIASES.iter() {
        if !my_name.ends_with(key) {
            continue;
        }
        command = value.0.to_string();
        sync = value.1;
        break;
    }
    if !command.is_empty() {
        argv.insert(0, command);
    }

    let drive_letter = config.qemu_exec_client.drive_letter;
    argv = host_to_guest(&argv, &drive_letter)?;

    let sync = if sync { Some("true".to_string()) } else { None };
    let payload = Payload {
        command: argv,
        sync,
    };
    let json = serde_json::to_string(&payload)?;
    let url = format!("http://{guest_ip}:8000/exec");
    let mut buf = isahc::Request::post(url)
        .redirect_policy(isahc::config::RedirectPolicy::Limit(1))
        .body(json)?
        .send()?;
    println!("{}", buf.text()?);

    Ok(())
}
