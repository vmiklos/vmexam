/*
 * Copyright 2024 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Simple wrapper around tesseract to OCR PDF files.

use anyhow::Context as _;
use clap::Parser as _;

#[derive(clap::Parser)]
struct Arguments {
    /// Input PDF to OCR.
    fro: String,

    /// Output PDF.
    to: String,
}

/// Converts a tempfile to a path that external commands can access.
fn tempfile_to_path(tempfile: &tempfile::NamedTempFile) -> anyhow::Result<String> {
    Ok(tempfile
        .path()
        .to_str()
        .context("to_str() failed")?
        .to_string())
}

fn convert(fro: &str, to: &str) -> anyhow::Result<()> {
    // 4x of the CSS default 96 DPI.
    let args = vec![
        "-density".into(),
        "384".into(),
        fro.to_string(),
        to.to_string(),
    ];
    println!("convert {}", args.join(" "));
    let exit_status = std::process::Command::new("convert").args(args).status()?;
    let exit_code = exit_status.code().context("code() failed")?;
    if exit_code != 0 {
        return Err(anyhow::anyhow!("convert failed"));
    }

    Ok(())
}

fn tesseract(fro: &str, to: &str) -> anyhow::Result<()> {
    // TODO: infer e.g. '-l hun' from $LANG.
    let to = to
        .strip_suffix(".pdf")
        .context("output doesn't end with .pdf")?;
    let args = vec![fro.to_string(), to.to_string(), "pdf".to_string()];
    println!("tesseract {}", args.join(" "));
    let exit_status = std::process::Command::new("tesseract")
        .args(args)
        .status()?;
    let exit_code = exit_status.code().context("code() failed")?;
    if exit_code != 0 {
        return Err(anyhow::anyhow!("tesseract failed"));
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    let tempfile = tempfile::Builder::new().suffix(".png").tempfile()?;
    let path = tempfile_to_path(&tempfile)?;
    convert(&args.fro, &path)?;
    tesseract(&path, &args.to)?;

    Ok(())
}
