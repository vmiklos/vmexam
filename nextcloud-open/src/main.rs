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

#[derive(Debug, Default)]
struct Account {
    pub local_path: String,
    pub url: String,
}

fn get_nextcloud_config() -> anyhow::Result<HashMap<String, HashMap<String, Option<String>>>> {
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let home_dir: String = home_dir.to_str().context("to_str() failed")?.into();
    let config_path = home_dir + "/.config/Nextcloud/nextcloud.cfg";

    let mut config = configparser::ini::Ini::new();
    match config.load(config_path) {
        Ok(value) => Ok(value),
        Err(value) => Err(anyhow::anyhow!(value)),
    }
}

fn get_accounts(
    nextcloud_config: &HashMap<String, HashMap<String, Option<String>>>,
) -> anyhow::Result<Vec<Account>> {
    // Each account has an ID.
    let accounts_config = &nextcloud_config["accounts"];
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
                entry.local_path = value.to_string();
            } else if key_suffix == "url" {
                entry.url = value.to_string();
            }
        }
    }

    Ok(accounts.into_iter().map(|(_k, v)| v).collect())
}

fn get_first_user_path() -> anyhow::Result<String> {
    let mut args = std::env::args();
    let _first = args.next().context("no self")?;
    let relative = args.next().context("no relative")?;
    let path_buf = std::fs::canonicalize(relative)?;
    Ok(path_buf.to_str().context("to_str() failed")?.to_string())
}

fn get_account<'a>(accounts: &'a [Account], absolute: &str) -> anyhow::Result<&'a Account> {
    for account in accounts.iter() {
        if absolute.starts_with(&account.local_path) {
            return Ok(account);
        }
    }

    Err(anyhow::anyhow!("local path not in sync directory"))
}

fn get_url(account: &Account, absolute: &str) -> anyhow::Result<url::Url> {
    let path = absolute
        .strip_prefix(&account.local_path)
        .context("unexpected prefix")?;
    let encoded_path = urlencoding::encode(path);
    let full_url = format!("{}/apps/files/?dir=/{}/", account.url, encoded_path);
    Ok(url::Url::parse(&full_url)?)
}

fn main() -> anyhow::Result<()> {
    let nextcloud_config = get_nextcloud_config()?;
    let accounts = get_accounts(&nextcloud_config)?;
    let user_path = get_first_user_path()?;
    let account = get_account(&accounts, &user_path)?;
    let url = get_url(account, &user_path)?;
    url.open();
    Ok(())
}
