/*
 * Copyright 2022 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

use anyhow::Context as _;

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
    let mut converteds: Vec<String> = Vec::new();
    for arg in args {
        let tempfile = tempfile::Builder::new().suffix(".jpg").tempfile()?;
        let path = tempfile
            .path()
            .file_name()
            .context("file_name() failed")?
            .to_str()
            .context("to_str() failed")?;
        let convert_args = vec![arg.clone(), "-density".into(), "96".into(), path.into()];
        println!("convert {}", convert_args.join(" "));
        let exit_status = std::process::Command::new("convert")
            .args(convert_args)
            .status()?;
        let exit_code = exit_status.code().context("code() failed")?;
        if exit_code != 0 {
            return Err(anyhow::anyhow!("convert failed"));
        }
        converteds.push(path.to_string());
    }

    // Convert downconverted intputs into a single output.
    let mut convert_args = converteds.clone();
    convert_args.push(output.clone());
    println!("convert {}", convert_args.join(" "));
    let exit_status = std::process::Command::new("convert")
        .args(convert_args)
        .status()?;
    let exit_code = exit_status.code().context("code() failed")?;
    if exit_code != 0 {
        return Err(anyhow::anyhow!("convert failed"));
    }

    for converted in converteds {
        std::fs::remove_file(converted)?;
    }

    Ok(())
}
