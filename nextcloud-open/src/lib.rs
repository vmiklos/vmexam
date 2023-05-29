/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Opens a local directory or file in nextcloud, assuming the directory is inside a sync folder.

use anyhow::Context as _;
use std::collections::HashMap;
use std::rc::Rc;

/// Network interface.
pub trait Network {
    /// Opens an URL in a browser.
    fn open_browser(&self, url: &url::Url);
}

/// Abstracts away the physical filesystem / browser opener.
pub struct Context {
    fs: vfs::VfsPath,
    network: Rc<dyn Network>,
}

impl Context {
    /// Creates a new Context.
    pub fn new(fs: vfs::VfsPath, network: Rc<dyn Network>) -> Self {
        Context { fs, network }
    }
}

#[derive(Debug, Default)]
struct Account {
    pub local_path: String,
    pub url: String,
}

fn get_nextcloud_config(
    ctx: &Context,
) -> anyhow::Result<HashMap<String, HashMap<String, Option<String>>>> {
    let mut config_file = ctx
        .fs
        .join(".config/Nextcloud/nextcloud.cfg")?
        .open_file()?;
    let mut content = String::new();
    config_file.read_to_string(&mut content)?;

    let mut config = configparser::ini::Ini::new();
    match config.read(content) {
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

    Ok(accounts.into_values().collect())
}

struct UserPath {
    pub parent: String,
    pub file_name: String,
}

fn get_first_user_path(ctx: &Context, input: &std::path::Path) -> anyhow::Result<UserPath> {
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let path = input
        .strip_prefix(home_dir.as_path())?
        .as_os_str()
        .to_str()
        .context("to_str() failed")?;
    let path = ctx.fs.join(path)?;
    if path.is_dir()? {
        let parent = input.to_str().context("to_str() failed")?.to_string();
        let file_name = "".into();
        Ok(UserPath { parent, file_name })
    } else {
        let parent = input
            .parent()
            .context("no parent")?
            .to_str()
            .context("to_str() failed")?
            .to_string();
        let file_name = input
            .file_name()
            .context("no file_name")?
            .to_str()
            .context("to_str() failed")?
            .to_string();
        Ok(UserPath { parent, file_name })
    }
}

fn get_account<'a>(accounts: &'a [Account], absolute: &str) -> anyhow::Result<&'a Account> {
    for account in accounts.iter() {
        if absolute.starts_with(&account.local_path) {
            return Ok(account);
        }
    }

    Err(anyhow::anyhow!("local path not in sync directory"))
}

fn get_url(account: &Account, user_path: &UserPath) -> anyhow::Result<url::Url> {
    let path = user_path
        .parent
        .strip_prefix(&account.local_path)
        .context("unexpected prefix")?;
    let encoded_path = urlencoding::encode(path);
    let mut full_url = format!("{}/apps/files/?dir=/{}/", account.url, encoded_path);
    if !user_path.file_name.is_empty() {
        full_url += &format!("&scrollto={}", urlencoding::encode(&user_path.file_name));
    }
    println!("Opening <{}>.", full_url);
    Ok(url::Url::parse(&full_url)?)
}

/// Opens the server version of `input` in a browser.
pub fn nextcloud_open(ctx: &Context, input: &std::path::Path) -> anyhow::Result<()> {
    let nextcloud_config = get_nextcloud_config(ctx)?;
    let accounts = get_accounts(&nextcloud_config)?;
    let user_path = get_first_user_path(ctx, input)?;
    let account = get_account(&accounts, &user_path.parent)?;
    let url = get_url(account, &user_path)?;
    ctx.network.open_browser(&url);
    Ok(())
}
