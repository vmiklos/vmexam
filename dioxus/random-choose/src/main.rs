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
use std::collections::HashMap;

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

#[cfg(feature = "web")]
fn init_choices() -> Vec<String> {
    // /?choices=a,b,c,d can be used to init the list, otherwise use the default.
    let window = web_sys::window().unwrap();
    let mut search = window.location().search().unwrap();
    if let Some(value) = search.strip_prefix("?") {
        search = value.to_string();
    }
    let iter = form_urlencoded::parse(search.as_bytes());
    let pairs: HashMap<String, String> = iter.into_owned().collect();
    match pairs.get("choices") {
        Some(value) => value.split(",").map(|i| i.to_string()).collect(),
        None => vec!["".to_string(), "".to_string()],
    }
}

#[cfg(not(feature = "web"))]
fn init_choices() -> Vec<String> {
    vec!["".to_string(), "".to_string()]
}

/// The root component.
pub fn app() -> Element {
    let mut choices = use_signal(|| init_choices());
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
