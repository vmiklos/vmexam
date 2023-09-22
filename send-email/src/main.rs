/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! This is a really simple mail(1)-like script that supports authentication, if
//! you need something simpler than msmtp or postfix. Sample config:
//!
//! server = 'mail.example.com'
//! user = 'vmiklos'
//! password = '...'

use anyhow::Context as _;
use clap::Parser as _;
use lettre::Transport as _;

#[derive(serde::Deserialize)]
struct Config {
    server: String,
    user: String,
    password: String,
}

#[derive(clap::Parser)]
struct Arguments {
    /// Sender address.
    #[arg(short, long)]
    from: String,

    /// Mail subject.
    #[arg(short, long)]
    subject: String,

    /// Destination address.
    to: String,
}

fn main() -> anyhow::Result<()> {
    // Parse the arguments.
    let args = Arguments::parse();

    // Parse the config.
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let home_dir: String = home_dir.to_str().context("to_str() failed")?.into();
    let config_path = home_dir + "/.config/send-emailrc";
    let config_string = std::fs::read_to_string(&config_path)
        .context(format!("failed to read config from '{config_path}'"))?;
    let config: Config = toml::from_str(&config_string)?;

    // Read body from stdin.
    let stdin = std::io::stdin();
    let body = std::io::read_to_string(stdin)?;

    // Send the mail.
    let email = lettre::Message::builder()
        .from(args.from.parse()?)
        .to(args.to.parse()?)
        .subject(args.subject)
        .header(lettre::message::header::ContentType::TEXT_PLAIN)
        .message_id(None)
        .body(body)?;
    let creds =
        lettre::transport::smtp::authentication::Credentials::new(config.user, config.password);
    let mailer = lettre::SmtpTransport::relay(&config.server)?
        .credentials(creds)
        .build();
    mailer.send(&email).context("send failed")?;

    Ok(())
}
