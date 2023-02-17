/*
 * Copyright 2022 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

use anyhow::Context as _;

/// Converts a tempfile to a path that external commands can access.
fn tempfile_to_path(tempfile: &tempfile::NamedTempFile) -> anyhow::Result<String> {
    Ok(tempfile
        .path()
        .to_str()
        .context("to_str() failed")?
        .to_string())
}

/// Downscales a bitmap (scanned document) to 96x96 DPI to avoid huge PDFs and then converts the
/// result to PDF.
///
/// Example usage: scan-document ~/Downloads/*.jpg out.pdf
fn main() -> anyhow::Result<()> {
    // Separate earlier input and last output arg.
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    let output = args.last().cloned().context("no output arg")?;
    args.pop();

    // Downconvert inputs.
    let mut converteds: Vec<tempfile::NamedTempFile> = Vec::new();
    for arg in args {
        let tempfile = tempfile::Builder::new().suffix(".jpg").tempfile()?;
        let path = tempfile_to_path(&tempfile)?;
        let convert_args = vec![arg.clone(), "-density".into(), "96".into(), path.clone()];
        println!("convert {}", convert_args.join(" "));
        let exit_status = std::process::Command::new("convert")
            .args(convert_args)
            .status()?;
        let exit_code = exit_status.code().context("code() failed")?;
        if exit_code != 0 {
            return Err(anyhow::anyhow!("convert failed"));
        }
        converteds.push(tempfile);
    }

    // Convert downconverted intputs into a single output.
    let mut convert_args: Vec<String> = converteds
        .iter()
        .map(tempfile_to_path)
        .collect::<anyhow::Result<Vec<String>>>()?;
    convert_args.push(output);
    println!("convert {}", convert_args.join(" "));
    let exit_status = std::process::Command::new("convert")
        .args(convert_args)
        .status()?;
    let exit_code = exit_status.code().context("code() failed")?;
    if exit_code != 0 {
        return Err(anyhow::anyhow!("convert failed"));
    }

    Ok(())
}
