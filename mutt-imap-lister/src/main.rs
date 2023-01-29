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
use std::collections::HashMap;

#[derive(serde::Deserialize)]
struct Config {
    pub ignore: HashMap<String, Vec<String>>,
}

fn main() -> anyhow::Result<()> {
    // Parse the arguments.
    let mut args = std::env::args();
    let _ = args.next();
    let domain: &str = &args.next().context("no domain")?;
    let user = args.next().context("no user")?;
    let password = args.next().context("no password")?;

    // Parse the config.
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let home_dir: String = home_dir.to_str().context("home_dir is not utf8")?.into();
    let config_path = home_dir + "/.mutt/imap-lister.yaml";
    let config_str = std::fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&config_str)?;

    // Connect to the IMAP server.
    eprint!("Listing {}...", domain);
    let tls = native_tls::TlsConnector::builder().build()?;
    let client = imap::connect((domain, 993), domain, &tls).context("connect failed")?;
    let mut session = match client.login(user, password) {
        Ok(s) => s,
        Err((e, _)) => {
            return Err(anyhow::Error::new(e).context("login failed"));
        }
    };

    // List the subscribed folders.
    let folders = session.lsub(None, Some("*"))?;
    let mut folder_names: Vec<String> = Vec::new();
    let ignored_folders = config.ignore.get(domain);
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
    eprintln!(" done.");

    // Print the non-ignored folders.
    print!(r#""imaps://{}/INBOX" "#, domain);
    for folder_name in folder_names {
        if folder_name == "INBOX" {
            continue;
        }
        print!(r#""imaps://{}/{}" "#, domain, folder_name);
    }

    Ok(())
}
