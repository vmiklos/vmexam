/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
*/

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! The serde module contains structs used while parsing data using the serde crate.

/// TurboTags contains various tags about one Overpass element.
#[derive(serde::Deserialize)]
pub struct TurboTags {
    #[serde(rename(deserialize = "addr:city"))]
    pub city: String,
    #[serde(rename(deserialize = "addr:housenumber"))]
    pub housenumber: String,
    #[serde(rename(deserialize = "addr:postcode"))]
    pub postcode: String,
    #[serde(rename(deserialize = "addr:street"))]
    pub street: String,
}

/// TurboElement represents one result from Overpass.
#[derive(serde::Deserialize)]
pub struct TurboElement {
    pub tags: TurboTags,
}

/// TurboResult is the result from Overpass.
#[derive(serde::Deserialize)]
pub struct TurboResult {
    pub elements: Vec<TurboElement>,
}

/// NominatimResult represents one element in the result array from Nominatim.
#[derive(Clone, serde::Deserialize)]
pub struct NominatimResult {
    pub class: String,
    pub lat: String,
    pub lon: String,
    pub osm_type: String,
    pub osm_id: u64,
}
