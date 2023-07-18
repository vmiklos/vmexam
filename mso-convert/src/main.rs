/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

//! Wrapper around docto, makes it similar to 'soffice --convert-to <format> <file>'.

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

use anyhow::Context as _;
use clap::Parser as _;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
enum TargetExtension {
    Pdf,
    Doc,
    Docx,
    Rtf,
}

#[derive(clap::Parser)]
struct Arguments {
    #[arg(short, long, value_enum)]
    to: TargetExtension,
    input: String,
}

fn get_format(to: TargetExtension) -> &'static str {
    // https://learn.microsoft.com/en-us/office/vba/api/word.wdsaveformat
    match to {
        TargetExtension::Pdf => "wdFormatPDF",
        TargetExtension::Doc => "wdFormatDocument97",
        TargetExtension::Docx => "wdFormatDocumentDefault",
        TargetExtension::Rtf => "wdFormatRTF",
    }
}

fn get_extension(to: TargetExtension) -> &'static str {
    match to {
        TargetExtension::Pdf => "pdf",
        TargetExtension::Doc => "doc",
        TargetExtension::Docx => "docx",
        TargetExtension::Rtf => "rtf",
    }
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    let input = std::path::PathBuf::from(&args.input);
    let input_file_name = input
        .file_name()
        .context("no file name")?
        .to_str()
        .context("to_str() failed")?;
    let mut output = std::path::PathBuf::from(input_file_name);
    // E.g. if our input is test.docx, the output should be test.pdf.
    output.set_extension(get_extension(args.to));
    let current_dir = std::env::current_dir()?;
    let working_directory: String = current_dir.to_str().context("to_str() failed")?.into();
    // Convert to abs path, so host -> guest path can be converted.
    let output_file_name = format!(
        "{}/{}",
        working_directory,
        output
            .file_name()
            .context("no file name")?
            .to_str()
            .context("to_str() failed")?
    );
    // Remove output, so outdated output is not around on failure.
    if std::path::Path::new(&output_file_name).exists() {
        std::fs::remove_file(&output_file_name)?;
    }

    let args = [
        "-WD",
        "-f",
        &args.input,
        "-o",
        &output_file_name,
        "-t",
        get_format(args.to),
    ];
    let exit_status = std::process::Command::new("docto")
        .args(args)
        .status()
        .context("failed to execute 'docto' and collect its status")?;
    let exit_code = exit_status.code().context("code() failed")?;
    match exit_code {
        0 => Ok(()),
        _ => Err(anyhow::anyhow!(
            "executing docto failed with exit code {exit_code}"
        )),
    }
}
