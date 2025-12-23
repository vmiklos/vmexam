/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to loim.

use dioxus::prelude::*;
use rand::Rng as _;

fn main() {
    dioxus::launch(app);
}

fn init_questions() -> Vec<String> {
    let ret: Vec<String> = [
        rand::rng().random_range(1..=110).to_string(),
        rand::rng().random_range(1..=90).to_string(),
        rand::rng().random_range(1..=94).to_string(),
        rand::rng().random_range(1..=90).to_string(),
        rand::rng().random_range(1..=86).to_string(),
        rand::rng().random_range(1..=74).to_string(),
        rand::rng().random_range(1..=70).to_string(),
        rand::rng().random_range(1..=66).to_string(),
        rand::rng().random_range(1..=62).to_string(),
        rand::rng().random_range(1..=58).to_string(),
        rand::rng().random_range(1..=46).to_string(),
        rand::rng().random_range(1..=42).to_string(),
        rand::rng().random_range(1..=38).to_string(),
        rand::rng().random_range(1..=34).to_string(),
        rand::rng().random_range(1..=30).to_string(),
    ].to_vec();
    ret
}

/// The root component.
pub fn app() -> Element {
    let questions = init_questions();

    rsx! {
        table {
            tr {
                th { "Nyeremény" }
                th { "Oldal" }
                th { "Kérdés" }
            }
            tr {
                td { "5 000 Ft" }
                td { "11" }
                td { "{questions[0]}" }
            }
            tr {
                td { "10 000 Ft" }
                td { "35" }
                td { "{questions[1]}" }
            }
            tr {
                td { "25 000 Ft" }
                td { "57" }
                td { "{questions[2]}" }
            }
            tr {
                td { "50 000 Ft" }
                td { "77" }
                td { "{questions[3]}" }
            }
            tr {
                td { "100 000 Ft" }
                td { "97" }
                td { "{questions[4]}" }
            }
            tr {
                td { "200 000 Ft" }
                td { "117" }
                td { "{questions[5]}" }
            }
            tr {
                td { "300 000 Ft" }
                td { "133" }
                td { "{questions[6]}" }
            }
            tr {
                td { "500 000 Ft" }
                td { "149" }
                td { "{questions[7]}" }
            }
            tr {
                td { "800 000 Ft" }
                td { "165" }
                td { "{questions[8]}" }
            }
            tr {
                td { "1 500 000 Ft" }
                td { "179" }
                td { "{questions[9]}" }
            }
            tr {
                td { "3 000 000 Ft" }
                td { "193" }
                td { "{questions[10]}" }
            }
            tr {
                td { "5 000 000 Ft" }
                td { "205" }
                td { "{questions[11]}" }
            }
            tr {
                td { "10 000 000 Ft" }
                td { "215" }
                td { "{questions[12]}" }
            }
            tr {
                td { "20 000 000 Ft" }
                td { "225" }
                td { "{questions[13]}" }
            }
            tr {
                td { "40 000 000 Ft" }
                td { "233" }
                td { "{questions[14]}" }
            }
        }
    }
}
