/*
 * Copyright 2022 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Simple IMAP folder lister for mutt, supports ignoring domain-specific subscribed folders.

use anyhow::Context as _;
use clap::Parser as _;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
struct Config {
    pub ignore: HashMap<String, Vec<String>>,
}

#[derive(clap::Parser)]
struct Arguments {
    /// Domain name of the IMAP server.
    server: String,

    /// User name.
    user: String,

    /// User password.
    password: String,
}

/// Parses the config.
fn parse_config() -> anyhow::Result<Config> {
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let home_dir: String = home_dir.to_str().context("home_dir is not utf8")?.into();
    let config_path = home_dir + "/.mutt/imap-lister.yaml";
    let config_str = std::fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&config_str)?;
    Ok(config)
}

/// Connects to the IMAP server.
fn create_session(
    args: &Arguments,
) -> anyhow::Result<imap::Session<native_tls::TlsStream<std::net::TcpStream>>> {
    let tls = native_tls::TlsConnector::builder().build()?;
    let server: &str = &args.server;
    let client = imap::connect((server, 993), server, &tls).context("connect failed")?;
    let session: imap::Session<native_tls::TlsStream<std::net::TcpStream>> =
        match client.login(&args.user, &args.password) {
            Ok(s) => s,
            Err((e, _)) => {
                return Err(anyhow::Error::new(e).context("login failed"));
            }
        };
    Ok(session)
}

/// Lists the subscribed folders.
fn get_subscribed_folders(
    session: &mut imap::Session<native_tls::TlsStream<std::net::TcpStream>>,
    args: &Arguments,
    config: &Config,
) -> anyhow::Result<Vec<String>> {
    let folders = session.lsub(None, Some("*"))?;
    let mut folder_names: Vec<String> = Vec::new();
    let ignored_folders = config.ignore.get(&args.server);
    for folder in folders.iter() {
        let folder_name: String = folder.name().to_string();
        if let Some(value) = ignored_folders {
            if value.contains(&folder_name) {
                continue;
            }
        }
        folder_names.push(folder_name);
    }
    folder_names.sort();
    Ok(folder_names)
}

/// Prints the non-ignored folders.
fn print_folders(args: &Arguments, folder_names: &Vec<String>) {
    print!(r#""imaps://{}/INBOX" "#, args.server);
    for folder_name in folder_names {
        if folder_name == "INBOX" {
            continue;
        }
        print!(r#""imaps://{}/{}" "#, args.server, folder_name);
    }
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    let config = parse_config()?;

    eprint!("Listing {}...", args.server);
    let mut session = create_session(&args)?;
    let folder_names = get_subscribed_folders(&mut session, &args, &config)?;
    eprintln!(" done.");

    print_folders(&args, &folder_names);
    Ok(())
}
