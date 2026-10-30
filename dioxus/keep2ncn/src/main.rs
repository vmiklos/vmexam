/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to keep2ncn.

use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

/// Converts Google Keep plain text into Nextcloud Notes markdown.
fn convert(content: &str) -> String {
    content
        .split_inclusive('\n')
        .map(|line| {
            if line.starts_with('[') {
                format!("- {line}")
            } else {
                line.to_string()
            }
        })
        .collect()
}

/// The root component.
pub fn app() -> Element {
    let mut content = use_signal(String::new);

    rsx! {
        div {
            label { "This converts from Google Keep to Nextcloud Notes markdown." }
        }
        textarea {
            rows: "10",
            cols: "60",
            value: "{content}",
            oninput: move |event| content.set(event.value()),
        }
        div {
            button {
                onclick: move |_| {
                    let converted = convert(&content());
                    content.set(converted);
                },
                "Convert"
            }
        }
    }
}
