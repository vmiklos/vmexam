/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to cpm-gui.

use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    use dioxus::desktop::tao;
    let window = tao::window::WindowBuilder::new().with_title("cpm-gui");
    dioxus::LaunchBuilder::new()
        .with_cfg(
            dioxus::desktop::Config::new()
                .with_window(window)
                .with_menu(None),
        )
        .launch(app);
}

/// One password entry in the database.
#[derive(serde::Deserialize)]
struct Password {
    #[serde(rename(deserialize = "ID"))]
    id: i64,
    #[serde(rename(deserialize = "Machine"))]
    machine: String,
    #[serde(rename(deserialize = "Service"))]
    service: String,
    #[serde(rename(deserialize = "User"))]
    user: String,
    #[serde(rename(deserialize = "Password"))]
    password: String,
    #[serde(rename(deserialize = "PasswordType"))]
    password_type: String,
    #[serde(rename(deserialize = "Archived"))]
    archived: bool,
    #[serde(rename(deserialize = "Created"))]
    created: String,
    #[serde(rename(deserialize = "Modified"))]
    modified: String,
}

/// Fetch current passwords from a remote server named 'cpm'.
fn fetch_passwords() -> Vec<Password> {
    let output = std::process::Command::new("ssh")
        .args(["cpm", "cpm", "export"])
        .stdout(std::process::Stdio::piped())
        .output()
        .unwrap();
    let json_string = String::from_utf8(output.stdout).unwrap();
    let passwords: Vec<Password> = serde_json::from_str(&json_string).unwrap();
    passwords
}

/// Does this password match the search query?
fn show_password(password: &Password, filter: &str) -> bool {
    let s = format!(
        "{} {} {} {} {}",
        password.id, password.machine, password.service, password.user, password.password_type
    );
    s.contains(filter)
}

/// The root component.
pub fn app() -> Element {
    let passwords = use_signal(|| fetch_passwords());
    let mut filter = use_signal(|| "".to_string());

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        input {
            r#type: "text",
            placeholder: "Search...",
            oninput: move |event| {
                *filter.write() = event.value().to_string();
            },
        }
        table {
            tr {
                th { "ID" }
                th { "Machine" }
                th { "Service" }
                th { "User" }
                th { "Password" }
                th { "PasswordType" }
                th { "Archived" }
                th { "Created" }
                th { "Modified" }
            }
            for password in passwords.read().iter().filter(|i| show_password(i, &filter.read())) {
                tr { key: "{password.id}",
                    td { "{password.id}" }
                    td { "{password.machine}" }
                    td { "{password.service}" }
                    td { "{password.user}" }
                    td { "{password.password}" }
                    td { "{password.password_type}" }
                    td { "{password.archived}" }
                    td { "{password.created}" }
                    td { "{password.modified}" }
                }
            }
        }
    }
}
