/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! A darcs-like porcelain on top of git plumbing.

use anyhow::Context as _;
use clap::Parser as _;
use std::io::BufRead as _;
use std::io::Read as _;
use std::io::Write as _;

fn flushed_print(question: &str) -> anyhow::Result<()> {
    print!("{question} ");
    Ok(std::io::stdout().flush()?)
}

fn ask_string(question: &str) -> anyhow::Result<String> {
    flushed_print(question)?;
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().context("no first line")?;
    Ok(line?)
}

fn ask_char(question: &str) -> anyhow::Result<String> {
    flushed_print(question)?;
    let mut stdin = std::io::stdin();
    let fd = libc::STDIN_FILENO;
    let mut settings = termios::Termios::from_fd(fd)?;

    // Set raw mode.
    let old_settings = settings;
    settings.c_lflag &= !(termios::ICANON | libc::ECHO);
    termios::tcsetattr(fd, termios::TCSANOW, &settings)?;

    // Read a character.
    let mut buffer = [0; 1];
    stdin.read_exact(&mut buffer)?;

    // Restore old mode.
    termios::tcsetattr(fd, termios::TCSANOW, &old_settings)?;
    let ret = String::from_utf8(buffer.to_vec())?;
    println!("{}", ret);
    Ok(ret)
}

fn checked_run(first: &str, rest: &[&str]) -> anyhow::Result<()> {
    let exit_status = std::process::Command::new(first).args(rest).status()?;
    match exit_status.code().context("code() failed")? {
        0 => Ok(()),
        code => Err(anyhow::anyhow!(
            "failed to execute {first} {rest:?}: exit status is {code}"
        )),
    }
}

#[derive(clap::Args)]
struct Rec {
    files: Vec<String>,
}

#[derive(clap::Args)]
struct Rev {
    files: Vec<String>,
}

#[derive(clap::Args)]
struct What {
    #[arg(short, long)]
    summary: bool,
    files: Vec<String>,
}

#[derive(clap::Args)]
struct Push {}

#[derive(clap::Subcommand)]
enum Commands {
    Rec(Rec),
    Rev(Rev),
    What(What),
    Push(Push),
}

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn record(args: &Rec) -> anyhow::Result<()> {
    let exit_status = std::process::Command::new("git")
        .args(["diff", "--quiet", "HEAD"])
        .status()?;
    if exit_status.code().context("code() failed")? == 0 {
        println!("Ok, if you don't want to record anything, that's fine!");
        return Ok(());
    }
    let mut add = vec!["add", "--patch"];
    for file in &args.files {
        add.push(file);
    }
    checked_run("git", &add)?;
    let message = ask_string("What is the commit message?")?;
    let edit: bool;
    loop {
        let ret = ask_char("Do you want to add a long comment? [ynq]")?;
        if ret == "q" {
            return Ok(());
        } else if ret == "y" || ret == "n" {
            edit = ret == "y";
            break;
        }
        println!("Invalid response, try again!");
    }
    let mut commit = vec!["commit", "-m", &message];
    if edit {
        commit.push("-e");
    }
    checked_run("git", &commit)
}

fn revert(args: &Rev) -> anyhow::Result<()> {
    let exit_status = std::process::Command::new("git")
        .args(["diff", "--quiet", "HEAD"])
        .status()?;
    if exit_status.code().context("code() failed")? == 0 {
        println!("Ok, if you don't want to revert anything, that's fine!");
        return Ok(());
    }
    let mut checkout = vec!["checkout", "--patch"];
    for file in &args.files {
        checkout.push(file);
    }
    checked_run("git", &checkout)
}

fn whatsnew(args: &What) -> anyhow::Result<()> {
    let mut diff = vec!["diff", "HEAD", "-M", "-C", "--exit-code"];
    if args.summary {
        diff.push("--name-status");
    }
    for file in &args.files {
        diff.push(file);
    }
    let exit_status = std::process::Command::new("git").args(diff).status()?;
    if exit_status.code().context("code() failed")? == 0 {
        println!("No changes!");
    }
    Ok(())
}

fn push() -> anyhow::Result<()> {
    let output = std::process::Command::new("git")
        .args(["log", "HEAD@{upstream}.."])
        .output()?;
    if output.stdout.is_empty() {
        println!("No recorded local changes to push!");
        return Ok(());
    }
    println!("{}", String::from_utf8(output.stdout)?);
    loop {
        let ret = ask_char("Do you want to push these patches? [ynq]")?;
        if ret == "n" || ret == "q" {
            return Ok(());
        } else if ret == "y" {
            break;
        }
        println!("Invalid response, try again!");
    }
    let exit_status = std::process::Command::new("git").args(["push"]).status()?;
    if exit_status.code().context("code() failed")? != 0 {
        checked_run("git", &["pull", "-r"])?;
        checked_run("git", &["push"])?;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Rec(args) => record(args),
        Commands::Rev(args) => revert(args),
        Commands::What(args) => whatsnew(args),
        Commands::Push(_) => push(),
    }
}
