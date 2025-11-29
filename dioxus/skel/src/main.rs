/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to skel.

use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

/// The root component.
pub fn app() -> Element {
    rsx! {
        div { "skel" }
    }
}
