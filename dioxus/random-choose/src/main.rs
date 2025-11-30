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

#[cfg(feature = "web")]
fn web_clipboard_write_text(text: &str) {
    let text = text.to_string();
    wasm_bindgen_futures::spawn_local(async move {
        let window = web_sys::window().unwrap();
        let navigator = window.navigator().clipboard();
        let promise = navigator.write_text(&text);
        wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
    });
}

#[cfg(feature = "web")]
fn has_clipboard() -> bool {
    return true;
}

#[cfg(not(feature = "web"))]
fn has_clipboard() -> bool {
    return false;
}

/// The root component.
pub fn app() -> Element {
    let mut choices = use_signal(|| vec!["".to_string(), "".to_string()]);
    let mut choice = use_signal(|| "".to_string());

    rsx! {
        for (i , value) in choices.read().iter().enumerate() {
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
        div { "Choice: {choice}" }
        input {
            r#type: "button",
            value: "copy",
            onclick: move |_| {
                #[cfg(feature = "web")] web_clipboard_write_text(choice.read().as_str());
            },
            display: if has_clipboard() { "" } else { "none" },
        }
    }
}
