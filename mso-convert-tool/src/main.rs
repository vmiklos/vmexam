/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

//! Simple document conversion tool that runs inside a VM.

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

use anyhow::Context as _;

struct Arguments {
    word: bool,
    from: String,
    output: String,
    format: String,
}

impl Arguments {
    fn parse(argv: &[String]) -> anyhow::Result<Self> {
        let word_arg = clap::Arg::new("word")
            .short('w')
            .long("word")
            .action(clap::ArgAction::SetTrue)
            .help("Input file is a Word format");
        let from_arg = clap::Arg::new("from")
            .short('f')
            .long("from")
            .help("Input file path");
        let output_arg = clap::Arg::new("output")
            .short('o')
            .long("output")
            .help("Output file path");
        let type_arg = clap::Arg::new("type")
            .short('t')
            .long("type")
            .help("Output format: wdFormatDocument97");
        let args = [word_arg, from_arg, output_arg, type_arg];
        let app = clap::Command::new("mso-convert-tool");
        let matches = app.args(&args).try_get_matches_from(argv)?;
        let word = *matches
            .get_one::<bool>("word")
            .context("no transliterate arg")?;
        let from = matches
            .get_one::<String>("from")
            .cloned()
            .context("no from arg")?;
        let output = matches
            .get_one::<String>("output")
            .cloned()
            .context("no output arg")?;
        let format = matches
            .get_one::<String>("type")
            .cloned()
            .context("no type arg")?;
        Ok(Arguments {
            word,
            from,
            output,
            format,
        })
    }
}

/// Windows implementation, using MSO.
#[cfg(windows)]
fn convert(from: &str, output: &str, format: &str) -> anyhow::Result<()> {
    use winsafe::prelude::oleaut_IDispatch;

    struct WordApp {
        dispatch: winsafe::IDispatch,
    }

    impl WordApp {
        fn new() -> anyhow::Result<Self> {
            let clsid = winsafe::CLSIDFromProgID("Word.Application")?;
            let dispatch = winsafe::CoCreateInstance::<winsafe::IDispatch>(
                &clsid,
                None::<&winsafe::IUnknown>,
                winsafe::co::CLSCTX::LOCAL_SERVER,
            )?;
            Ok(WordApp { dispatch })
        }
    }

    impl Drop for WordApp {
        fn drop(&mut self) {
            self.dispatch.invoke_method("Quit", &[]).unwrap();
        }
    }

    struct WordDocument {
        dispatch: winsafe::IDispatch,
    }

    impl WordDocument {
        fn new(documents: &winsafe::IDispatch, from: &str) -> anyhow::Result<Self> {
            // Expected to be an absolute path, don't transform.
            let file_name = winsafe::Variant::from_str(from);
            let dispatch = documents
                .invoke_method("Open", &[&file_name])
                .map_err(|e| anyhow::anyhow!("Open() failed: {e}"))?
                .unwrap_dispatch();
            Ok(WordDocument { dispatch })
        }
    }

    impl Drop for WordDocument {
        fn drop(&mut self) {
            self.dispatch.invoke_method("Close", &[]).unwrap();
        }
    }

    let _com = winsafe::CoInitializeEx(winsafe::co::COINIT::APARTMENTTHREADED)?;

    let word_app = WordApp::new()?;

    let documents = word_app
        .dispatch
        .invoke_get("Documents", &[])
        .map_err(|e| anyhow::anyhow!("failed to get Documents: {e}"))?
        .unwrap_dispatch();

    let document = WordDocument::new(&documents, from)?;

    // Expected to be an absolute path, don't transform.
    let file_name = winsafe::Variant::from_str(output);
    // https://learn.microsoft.com/en-us/office/vba/api/word.wdsaveformat
    let file_format = match format {
        "wdFormatPDF" => winsafe::Variant::I4(17),
        "wdFormatDocument97" => winsafe::Variant::I4(0),
        "wdFormatDocumentDefault" => winsafe::Variant::I4(16),
        "wdFormatRTF" => winsafe::Variant::I4(6),
        _ => return Err(anyhow::anyhow!("unimplemented type value")),
    };
    document
        .dispatch
        .invoke_method("SaveAs", &[&file_name, &file_format])
        .map_err(|e| anyhow::anyhow!("SaveAs() failed: {e}"))?;

    Ok(())
}

/// Linux implementation, using 'soffice'.
#[cfg(unix)]
fn convert(from: &str, output: &str, format: &str) -> anyhow::Result<()> {
    let extension = match format {
        "wdFormatPDF" => "pdf",
        "wdFormatDocument97" => "doc",
        "wdFormatDocumentDefault" => "docx",
        "wdFormatRTF" => "rtf",
        _ => return Err(anyhow::anyhow!("unimplemented type value")),
    };
    let args = ["--convert-to", extension, from];
    let exit_status = std::process::Command::new("soffice")
        .args(args)
        .status()
        .context("failed to execute 'soffice' and collect its status")?;
    let exit_code = exit_status.code().context("code() failed")?;
    if exit_code != 0 {
        return Err(anyhow::anyhow!(
            "executing 'soffice' failed with exit code {exit_code}"
        ));
    }

    let mut soffice_output = std::path::PathBuf::from(from);
    soffice_output = std::path::PathBuf::from(soffice_output.file_name().context("no file name")?);
    soffice_output.set_extension(extension);
    let soffice_output_path = soffice_output
        .file_name()
        .context("no file name")?
        .to_str()
        .context("to_str() failed")?;
    if soffice_output_path != output {
        std::fs::rename(soffice_output_path, output)?;
    };

    Ok(())
}

fn our_main() -> anyhow::Result<()> {
    let argv: Vec<String> = std::env::args().collect();
    let args = Arguments::parse(&argv)?;

    if args.word {
        convert(&args.from, &args.output, &args.format)?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = our_main() {
        println!("mso-convert-tool our_main failed: {err:?}");
        std::process::exit(1);
    }
}
