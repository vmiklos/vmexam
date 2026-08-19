/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Web frontend for child-slide.

use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};

type CollectionData = HashMap<String, HashMap<String, f64>>;

fn find_min_max(data: &CollectionData, kid: &str) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for ages in data.values() {
        if let Some(&age) = ages.get(kid) {
            if age < min {
                min = age;
            }
            if age > max {
                max = age;
            }
        }
    }
    (min, max)
}

fn find_closest_video(data: &CollectionData, kid: &str, target_age: f64) -> Option<String> {
    let mut best_url = None;
    let mut best_diff = f64::INFINITY;
    for (url, ages) in data {
        if let Some(&age) = ages.get(kid) {
            let diff = (age - target_age).abs();
            if diff < best_diff {
                best_diff = diff;
                best_url = Some(url.clone());
            }
        }
    }
    best_url
}

fn get_token() -> Option<String> {
    let window = web_sys::window()?;
    let search = window.location().search().ok()?;
    let params = web_sys::UrlSearchParams::new_with_str(&search).ok()?;
    params.get("c")
}

/// The root component.
pub fn app() -> Element {
    let token = get_token();
    let token_for_resource = token.clone();

    let data_resource = use_resource(move || {
        let token = token_for_resource.clone();
        async move {
            match token {
                Some(t) => {
                    let relative = format!("{}.json", t);
                    let base = web_sys::window().unwrap().location().href().unwrap();
                    let url = web_sys::Url::new_with_base(&relative, &base)
                        .unwrap()
                        .href();
                    let resp = match reqwest::get(&url).await {
                        Ok(r) => r,
                        Err(e) => {
                            return Err(format!("Fetch error for {url}: {e}"));
                        }
                    };
                    resp.json::<CollectionData>()
                        .await
                        .map_err(|e| format!("JSON parse error for {url}: {e}"))
                }
                None => Err("Missing token".to_string()),
            }
        }
    });

    let mut selected_kid = use_signal(String::new);
    let mut slider_value = use_signal(|| 0.0_f64);
    let mut slider_min = use_signal(|| 0.0_f64);
    let mut slider_max = use_signal(|| 100.0_f64);
    let mut slider_disabled = use_signal(|| true);

    let on_kid_change = move |e: Event<FormData>| {
        let kid = e.value();
        if kid.is_empty() {
            selected_kid.set(String::new());
            slider_disabled.set(true);
            return;
        }
        if let Some(Ok(ref d)) = *data_resource.read() {
            let (min, max) = find_min_max(d, &kid);
            slider_min.set(min);
            slider_max.set(max);
            slider_value.set(min);
            slider_disabled.set(false);
            selected_kid.set(kid);
        }
    };

    let on_slider_input = move |e: Event<FormData>| {
        if let Ok(v) = e.value().parse::<f64>() {
            slider_value.set(v);
        }
    };

    let on_go = move |_| {
        let kid = selected_kid.read().clone();
        if kid.is_empty() {
            return;
        }
        if let Some(Ok(ref d)) = *data_resource.read() {
            if let Some(url) = find_closest_video(d, &kid, *slider_value.read()) {
                web_sys::window().unwrap().location().set_href(&url).ok();
            }
        }
    };

    let kids = if let Some(Ok(ref d)) = *data_resource.read() {
        let mut kids_set = HashSet::new();
        for ages in d.values() {
            for kid in ages.keys() {
                kids_set.insert(kid.clone());
            }
        }
        let mut sorted: Vec<String> = kids_set.into_iter().collect();
        sorted.sort();
        sorted
    } else {
        Vec::new()
    };

    rsx! {
        div { style: "font-family: sans-serif; max-width: 400px; margin: 2rem auto;",
            if token.is_none() {
                "Missing ?c= parameter."
            } else if data_resource.read().is_none() {
                "Loading..."
            } else if let Some(Err(_)) = *data_resource.read() {
                "Failed to load data."
            } else {
                span { "Kid name:" }
                br {}
                select {
                    value: "{selected_kid}",
                    onchange: on_kid_change,
                    option { value: "", "" }
                    for kid in kids.iter() {
                        option { value: "{kid}", "{kid}" }
                    }
                }

                br {}
                span { "Age: " }
                input {
                    r#type: "range",
                    disabled: slider_disabled(),
                    min: "{slider_min}",
                    max: "{slider_max}",
                    value: "{slider_value}",
                    style: "width: 100%",
                    oninput: on_slider_input,
                }
                span { "{slider_value}" }

                br {}
                button { onclick: on_go, "Go!" }
            }
        }
    }
}

fn main() {
    dioxus::launch(app);
}

// vim: shiftwidth=4 softtabstop=4 expandtab:
