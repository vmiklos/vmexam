/*
 * Copyright 2022 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

use anyhow::Context as _;

/// Downscales images in a scanned PDF to avoid huge PDFs.
///
/// Example usage: scan-document ~/Downloads/in.pdf out.pdf
fn main() -> anyhow::Result<()> {
    // Separate earlier input and last output arg.
    let mut inputs: Vec<String> = std::env::args().collect();
    inputs.remove(0);
    let output = inputs.last().cloned().context("no output arg")?;
    inputs.pop();

    let mut args: Vec<String> = [
        "-sDEVICE=pdfwrite".into(),
        "-dCompatibilityLevel=1.4".into(),
        "-dPDFSETTINGS=/ebook".into(),
        "-dNOPAUSE".into(),
        "-dQUIET".into(),
        "-dBATCH".into(),
        format!("-sOutputFile={output}"),
    ]
    .to_vec();
    args.append(&mut inputs);
    println!("gs {}", args.join(" "));
    let exit_status = std::process::Command::new("gs").args(args).status()?;
    let exit_code = exit_status.code().context("code() failed")?;
    if exit_code != 0 {
        return Err(anyhow::anyhow!("convert failed"));
    }

    Ok(())
}
