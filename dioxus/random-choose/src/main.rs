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

#[cfg(feature = "web")]
fn init_choices() -> Vec<String> {
    // /?choices=a,b,c,d can be used to init the list, otherwise use the default.
    let window = web_sys::window().unwrap();
    let search = window.location().search().unwrap();
    let pairs = web_sys::UrlSearchParams::new_with_str(&search).unwrap();
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
    let mut remove_choice = use_signal(|| false);

    rsx! {
        div {
            "Choices: "
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
        }
        div {
            input {
                id: "remove-checkbox",
                r#type: "checkbox",
                onchange: move |event| {
                    remove_choice.set(event.value().parse::<bool>().unwrap());
                },
            }
            label { r#for: "remove-checkbox", "Remove choice" }
        }
        div {
            input {
                r#type: "button",
                value: "Choose",
                onclick: move |_| {
                    let mut vec = choices.write();
                    if vec.is_empty() {
                        *choice.write() = "".to_string();
                        return;
                    }
                    let index = rand::rng().random_range(0..vec.len());
                    *choice.write() = vec[index].to_string();
                    if *remove_choice.read() {
                        vec.remove(index);
                    }
                },
            }
        }
        div { "Choice: {choice}" }
        input {
            r#type: "button",
            value: "Copy",
            onclick: move |_| {
                #[cfg(feature = "web")] web_clipboard_write_text(choice.read().as_str());
            },
            display: if has_clipboard() { "" } else { "none" },
        }
    }
}
