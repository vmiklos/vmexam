/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! The serde module contains structs used while parsing data using the serde crate.

use std::collections::HashMap;

/// Credentials for one specific nextcloud server.
#[derive(Clone, serde::Deserialize)]
pub struct Credential {
    pub user: String,
    pub principal: String,
    pub password: String,
}

/// Config of nextcloud-open for multiple servers.
#[derive(serde::Deserialize)]
pub struct Config {
    pub credentials: HashMap<String, Credential>,
}
