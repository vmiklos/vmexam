/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! The serde module contains structs used while parsing data using the serde crate.

/// Contents of the JWT payload.
#[derive(serde::Deserialize)]
pub(crate) struct Jwt {
    pub(crate) sub: String,
    pub(crate) exp: i64,
}
