/*
 * Copyright 2022 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Opens a local directory in nextcloud, assuming the directory is inside a sync folder.

use anyhow::Context as _;
use std::collections::HashMap;
use url_open::UrlOpen as _;

#[derive(Debug, Default, Clone)]
struct Account {
    pub local_path: String,
    pub url: String,
}

fn main() -> anyhow::Result<()> {
    // Get the config path.
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let home_dir: String = home_dir.to_str().context("to_str() failed")?.into();
    let config_path = home_dir + "/.config/Nextcloud/nextcloud.cfg";

    let mut config = configparser::ini::Ini::new();
    let config_map = match config.load(config_path) {
        Ok(value) => value,
        Err(value) => {
            return Err(anyhow::anyhow!(value));
        }
    };

    // Get the config: each account has an ID.
    let accounts_config = &config_map["accounts"];
    let mut accounts: HashMap<i64, Account> = HashMap::new();
    for (account_key, account_value) in accounts_config {
        let mut tokens = account_key.split('\\');
        let id = tokens.next().context("no id")?;
        let id: i64 = match id.parse() {
            Ok(value) => value,
            Err(_) => {
                continue;
            }
        };
        let entry = accounts.entry(id).or_default();
        let key_suffix = tokens.next_back().context("no suffix")?;
        if let Some(value) = account_value {
            if key_suffix == "localpath" {
                entry.local_path = value.clone();
            } else if key_suffix == "url" {
                entry.url = value.clone();
            }
        }
    }

    // Build a list of accounts.
    let accounts: Vec<Account> = accounts.into_iter().map(|(_k, v)| v).collect();

    // Find out the abs path of the first user argument.
    let mut args = std::env::args();
    let _first = args.next().context("no self")?;
    let relative = args.next().context("no relative")?;
    let path_buf = std::fs::canonicalize(relative)?;
    let absolute = path_buf.to_str().context("to_str() failed")?;
    let mut account: Account = Default::default();
    for a in accounts.iter() {
        if absolute.starts_with(&a.local_path) {
            account = a.clone();
            break;
        }
    }

    // Build the final URL that can be opened.
    let path = absolute
        .strip_prefix(&account.local_path)
        .context("unexpected prefix")?;
    let encoded_path = urlencoding::encode(path);
    let full_url = format!("{}/apps/files/?dir=/{}/", account.url, encoded_path);
    let url = url::Url::parse(&full_url)?;

    // Finally open it.
    url.open();

    Ok(())
}
