/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to rubik-scramble.

use dioxus::prelude::*;
#[cfg(target_os = "android")]
use preferences::Preferences as _;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[cfg(feature = "desktop")]
fn main() {
    use dioxus::desktop::tao;
    let window = tao::window::WindowBuilder::new().with_title("rubik-scramble");
    dioxus::LaunchBuilder::new()
        .with_cfg(
            dioxus::desktop::Config::new()
                .with_window(window)
                .with_menu(None),
        )
        .launch(app);
}

#[cfg(not(feature = "desktop"))]
fn main() {
    dioxus::launch(app);
}

const TABLE: &[u8] = include_bytes!("../bin/table.bin");

#[derive(PartialEq, Clone)]
enum Scramble {
    Wide,
    Normal,
    F2lSolved,
    OllSolved,
    Megaminx,
}

impl TryFrom<&str> for Scramble {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "wide" => Ok(Scramble::Wide),
            "normal" => Ok(Scramble::Normal),
            "f2l-solved" => Ok(Scramble::F2lSolved),
            "oll-solved" => Ok(Scramble::OllSolved),
            "megaminx" => Ok(Scramble::Megaminx),
            _ => Err(anyhow::anyhow!("invalid value: {value}")),
        }
    }
}

fn make_scramble(kind: Scramble) -> anyhow::Result<String> {
    if kind == Scramble::F2lSolved || kind == Scramble::OllSolved {
        // Generate a scramble that allows practicing solving the last layer.
        let table = kewb::fs::decode_table(TABLE)?;
        let mut solver = kewb::Solver::new(&table, 25, None);
        let mut states = Vec::new();

        let state = if kind == Scramble::F2lSolved {
            kewb::generators::generate_state_f2l_solved()
        } else {
            kewb::generators::generate_state_oll_solved()
        };
        let scramble = kewb::scramble::scramble_from_state(state, &mut solver)?;

        states.push(state);
        solver.clear();

        let stringified = scramble
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        return Ok(stringified);
    }

    let lang = "en";
    let wide = kind == Scramble::Wide;
    let megaminx = kind == Scramble::Megaminx;
    rubik::shuffle(lang, wide, megaminx)
}

#[cfg(feature = "web")]
fn local_storage_set_item(key: &str, value: &str) {
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    storage.set(key, value).unwrap();
}

#[cfg(feature = "web")]
fn local_storage_get_item(key: &str) -> Option<String> {
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    storage.get(key).unwrap()
}

#[cfg(target_os = "android")]
const APP_INFO: preferences::AppInfo = preferences::AppInfo {
    name: "rubik-scramble",
    author: "vmiklos",
};

#[cfg(target_os = "android")]
fn local_storage_set_item(key: &str, value: &str) {
    value.to_string().save(&APP_INFO, key).unwrap()
}

#[cfg(target_os = "android")]
fn local_storage_get_item(key: &str) -> Option<String> {
    match String::load(&APP_INFO, key) {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

fn is_font_size_selected(scramble_font_size: Signal<String>, font_size: &str) -> bool {
    scramble_font_size() == font_size
}

/// The root component.
pub fn app() -> Element {
    let mut scramble_type = use_signal(|| Scramble::Wide);
    let mut scramble = use_signal(|| "".to_string());
    let mut scramble_font_size =
        use_signal(|| local_storage_get_item("scrambleFontSize").unwrap_or("medium".to_string()));
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        label { r#for: "scramble-select", "Type: " }
        select {
            id: "scramble-select",
            onchange: move |event| {
                scramble_type.set(Scramble::try_from(event.value().as_str())?);
                Ok(())
            },
            option { value: "wide", "4x4 (blue goes to the top)" }
            option { value: "normal", "3x3" }
            option { value: "f2l-solved", "3x3 f2l-solved (yellow goes to the top, kewb)" }
            option { value: "oll-solved", "3x3 oll-solved (yellow goes to the top, kewb)" }
            option { value: "megaminx", "megaminx" }
        }
        input {
            r#type: "button",
            value: "OK",
            onclick: move |_| {
                scramble.set(make_scramble(scramble_type.read().clone())?);
                Ok(())
            },
        }
        div { "Scramble:" }
        div { font_size: scramble_font_size, "{scramble}" }
        div {
            span { "Powered by " }
            a { href: "https://github.com/vmiklos/vmexam/tree/master/dioxus/rubik-scramble",
                "dioxus"
            }
            span { " and " }
            a { href: "https://crates.io/crates/kewb", "kewb" }
        }

        // Settings
        input { r#type: "checkbox", id: "settings-toggle", hidden: true }
        label {
            r#for: "settings-toggle",
            class: "gear-button",
            title: "Settings",
            "⚙ "
        }
        div { class: "settings-overlay",
            div { class: "settings-panel",
                label { r#for: "settings-toggle", class: "close-button", "✕" }

                h2 { "Settings" }

                label { r#for: "font-size-select", "Font size:" }
                select {
                    id: "font-size-select",
                    onchange: move |event| {
                        let value = event.value();
                        scramble_font_size.set(value.to_string());
                        local_storage_set_item("scrambleFontSize", &value);
                        Ok(())
                    },
                    option {
                        value: "xx-small",
                        selected: is_font_size_selected(scramble_font_size, "xx-small"),
                        "extra extra small"
                    }
                    option {
                        value: "x-small",
                        selected: is_font_size_selected(scramble_font_size, "x-small"),
                        "extra small"
                    }
                    option {
                        value: "small",
                        selected: is_font_size_selected(scramble_font_size, "small"),
                        "small"
                    }
                    option {
                        value: "medium",
                        selected: is_font_size_selected(scramble_font_size, "medium"),
                        "medium"
                    }
                    option {
                        value: "large",
                        selected: is_font_size_selected(scramble_font_size, "large"),
                        "large"
                    }
                    option {
                        value: "x-large",
                        selected: is_font_size_selected(scramble_font_size, "x-large"),
                        "extra large"
                    }
                    option {
                        value: "xx-large",
                        selected: is_font_size_selected(scramble_font_size, "xx-large"),
                        "extra extra large"
                    }
                    option {
                        value: "xxx-large",
                        selected: is_font_size_selected(scramble_font_size, "xxx-large"),
                        "extra extra extra large"
                    }
                }
            }
        }
    }
}
