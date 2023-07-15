/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

//! Wrapper around docto, makes it similar to 'soffice --convert-to <format> <file>'.

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

use clap::Parser as _;

#[derive(clap::Parser)]
struct Arguments {
    #[arg(short, long)]
    to: String,
    input: String,
}

fn main() {
    let args = Arguments::parse();
    let input = std::path::PathBuf::from(&args.input);
    let input_file_name = input.file_name().unwrap().to_str().unwrap();
    let mut output = std::path::PathBuf::from(input_file_name);
    // E.g. if our input is test.docx, the output should be test.pdf.
    output.set_extension(args.to);
    let current_dir = std::env::current_dir().unwrap();
    let working_directory: String = current_dir.to_str().unwrap().into();
    // Convert to abs path, so host -> guest path can be converted.
    let output_file_name = format!(
        "{}/{}",
        working_directory,
        output.file_name().unwrap().to_str().unwrap()
    );
    // Remove output, so outdated output is not around on failure.
    if std::path::Path::new(&output_file_name).exists() {
        std::fs::remove_file(&output_file_name).unwrap();
    }

    // Format is hardcoded for now, could be generated based on --to.
    let args = [
        "-WD",
        "-f",
        &args.input,
        "-o",
        &output_file_name,
        "-t",
        "wdFormatPDF",
    ];
    let exit_status = std::process::Command::new("docto")
        .args(args)
        .status()
        .unwrap();
    let exit_code = exit_status.code().unwrap();
    assert_eq!(exit_code, 0);
}
