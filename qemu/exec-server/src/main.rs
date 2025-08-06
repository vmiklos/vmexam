/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to exec-server.

use anyhow::Context as _;
use std::io::Read as _;

#[derive(serde::Deserialize)]
struct Payload {
    command: Vec<String>,
}

fn run(args: Vec<String>) -> anyhow::Result<String> {
    let (first, rest) = args
        .split_first()
        .ok_or_else(|| anyhow::anyhow!("args is an empty list"))?;
    let output = std::process::Command::new(first).args(rest).output()?;
    Ok(std::str::from_utf8(&output.stdout)?.to_string())
}

fn our_app(request: &rouille::Request) -> anyhow::Result<String> {
    let mut request_data = Vec::new();
    let mut reader = request.data().context("data() gave None")?;
    reader.read_to_end(&mut request_data)?;
    let payload = String::from_utf8(request_data)?;

    let payload: Payload = serde_json::from_str(&payload)?;
    let mut body = "OK".to_string();
    let out = run(payload.command)?;
    if !out.is_empty() {
        body += &format!("\n{}", out.trim());
    }

    Ok(body)
}

fn app(request: &rouille::Request) -> rouille::Response {
    match our_app(request) {
        Ok(text) => rouille::Response::text(text),
        Err(err) => rouille::Response::text(format!("Err: {err:?}")),
    }
}

fn main() {
    let port = 8000;
    println!("Starting the server at <http://127.0.0.1:{port}/>.");
    rouille::start_server_with_pool(format!("127.0.0.1:{port}"), None, move |request| {
        app(request)
    });
}
