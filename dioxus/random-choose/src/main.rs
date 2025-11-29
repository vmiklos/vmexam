/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to random-choose.

use dioxus::prelude::*;
use rand::Rng as _;

fn main() {
    dioxus::launch(app);
}

/// The root component.
pub fn app() -> Element {
    let mut choices = use_signal(|| vec!["".to_string(), "".to_string()]);
    let mut choice = use_signal(|| "".to_string());

    rsx! {
        for (i, value) in choices.read().iter().enumerate() {
            input {
                id: "input{i}",
                value: "{value}",
                oninput: move |event| {
                    let mut vec = choices.write();
                    vec[i] = event.value().to_string();
                },
            }
        }
        input {
            r#type: "button",
            value: "+",
            onclick: move |_| {
                choices.write().push("".to_string());
            },
        }
        input {
            r#type: "button",
            value: "choose",
            onclick: move |_| {
                let vec = choices.write();
                let index = rand::rng().random_range(0..vec.len());
                *choice.write() = vec[index].to_string();
            },
        }
        div {
            "Choice: {choice}"
        }
    }
}
