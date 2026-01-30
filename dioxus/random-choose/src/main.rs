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
    async fn try_write_text(text: &str) -> anyhow::Result<()> {
        let window = web_sys::window().context("no window")?;
        let navigator = window.navigator().clipboard();
        let promise = navigator.write_text(text);
        if let Err(err) = wasm_bindgen_futures::JsFuture::from(promise).await {
            return Err(anyhow::anyhow!("write_text() failed: {:?}", err));
        }
        Ok(())
    }

    let text = text.to_string();
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(err) = try_write_text(&text).await {
            tracing::error!("try_write_text() failed: {:?}", err);
        }
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
    fn try_init_choices() -> anyhow::Result<Vec<String>> {
        // /?choices=a,b,c,d can be used to init the list, otherwise use the default.
        let window = web_sys::window().context("no window")?;
        let Ok(search) = window.location().search() else {
            return Err(anyhow::anyhow!("no location search"));
        };
        let Ok(pairs) = web_sys::UrlSearchParams::new_with_str(&search) else {
            return Err(anyhow::anyhow!("failed to create UrlSearchParams"));
        };
        let choices = pairs.get("choices").context("no choices")?;
        Ok(choices.split(",").map(|i| i.to_string()).collect())
    }
    // /?choices=a,b,c,d can be used to init the list, otherwise use the default.
    match try_init_choices() {
        Ok(value) => value,
        Err(_) => vec!["".to_string(), "".to_string()],
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
                    remove_choice.set(event.checked());
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
