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
use std::io::BufRead as _;
use std::io::Read as _;
use std::io::Write as _;

/// Context interface.
trait Context {
    /// Executes a command as a child process, waiting for it to finish and
    /// collecting its status.
    fn command_status(&self, command: &str, args: &[&str]) -> anyhow::Result<i32>;

    /// Executes the command as a child process, waiting for it to finish and
    /// collecting all of its output.
    fn command_output(&self, command: &str, args: &[&str]) -> anyhow::Result<String>;
}

/// Context implementation, backed by library calls.
struct StdContext {}

impl Context for StdContext {
    fn command_status(&self, command: &str, args: &[&str]) -> anyhow::Result<i32> {
        let exit_status = std::process::Command::new(command).args(args).status()?;
        exit_status.code().context("code() failed")
    }

    fn command_output(&self, command: &str, args: &[&str]) -> anyhow::Result<String> {
        let output = std::process::Command::new(command).args(args).output()?;
        String::from_utf8(output.stdout).context("from_utf8() failed")
    }
}

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

fn checked_run(ctx: &dyn Context, first: &str, rest: &[&str]) -> anyhow::Result<()> {
    let code = ctx.command_status(first, rest)?;
    match code {
        0 => Ok(()),
        code => Err(anyhow::anyhow!(
            "failed to execute {first} {rest:?}: exit status is {code}"
        )),
    }
}

struct Rec {
    files: Vec<String>,
}

struct Rev {
    files: Vec<String>,
}

struct What {
    summary: bool,
    files: Vec<String>,
}

fn record(ctx: &dyn Context, args: &clap::ArgMatches) -> anyhow::Result<()> {
    let args = Rec {
        files: args
            .get_many::<String>("files")
            .unwrap_or_default()
            .cloned()
            .collect(),
    };
    let code = ctx.command_status("git", &["diff", "--quiet", "HEAD"])?;
    if code == 0 {
        println!("Ok, if you don't want to record anything, that's fine!");
        return Ok(());
    }
    let mut add = vec!["add", "--patch"];
    for file in &args.files {
        add.push(file);
    }
    checked_run(ctx, "git", &add)?;
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
    checked_run(ctx, "git", &commit)
}

fn revert(ctx: &dyn Context, args: &clap::ArgMatches) -> anyhow::Result<()> {
    let args = Rev {
        files: args
            .get_many::<String>("files")
            .unwrap_or_default()
            .cloned()
            .collect(),
    };
    let code = ctx.command_status("git", &["diff", "--quiet", "HEAD"])?;
    if code == 0 {
        println!("Ok, if you don't want to revert anything, that's fine!");
        return Ok(());
    }
    let mut checkout = vec!["checkout", "--patch"];
    for file in &args.files {
        checkout.push(file);
    }
    checked_run(ctx, "git", &checkout)
}

fn whatsnew(ctx: &dyn Context, args: &clap::ArgMatches) -> anyhow::Result<()> {
    let args = What {
        summary: args.get_flag("summary"),
        files: args
            .get_many::<String>("files")
            .unwrap_or_default()
            .cloned()
            .collect(),
    };
    let mut diff = vec!["diff", "HEAD", "-M", "-C", "--exit-code"];
    if args.summary {
        diff.push("--name-status");
    }
    for file in &args.files {
        diff.push(file);
    }
    let code = ctx.command_status("git", &diff)?;
    if code == 0 {
        println!("No changes!");
    }
    Ok(())
}

fn push(ctx: &dyn Context) -> anyhow::Result<()> {
    let output = ctx.command_output("git", &["log", "HEAD@{upstream}.."])?;
    if output.is_empty() {
        println!("No recorded local changes to push!");
        return Ok(());
    }
    println!("{output}");
    loop {
        let ret = ask_char("Do you want to push these patches? [ynq]")?;
        if ret == "n" || ret == "q" {
            return Ok(());
        } else if ret == "y" {
            break;
        }
        println!("Invalid response, try again!");
    }
    let code = ctx.command_status("git", &["push"])?;
    if code != 0 {
        checked_run(ctx, "git", &["pull", "-r"])?;
        checked_run(ctx, "git", &["push"])?;
    }
    Ok(())
}

fn unrec(ctx: &dyn Context) -> anyhow::Result<()> {
    checked_run(ctx, "git", &["log", "-1"])?;
    loop {
        let ret = ask_char("Do you want to unrecord this patch? [ynq]")?;
        if ret == "n" || ret == "q" {
            return Ok(());
        } else if ret == "y" {
            break;
        }
        println!("Invalid response, try again!");
    }
    checked_run(ctx, "git", &["reset", "--quiet", "HEAD^"])?;
    println!("Finished unrecording.");
    Ok(())
}

fn unpull(ctx: &dyn Context) -> anyhow::Result<()> {
    checked_run(ctx, "git", &["log", "-1"])?;
    loop {
        let ret = ask_char("Do you want to unpull this patch? [ynq]")?;
        if ret == "n" || ret == "q" {
            return Ok(());
        } else if ret == "y" {
            break;
        }
        println!("Invalid response, try again!");
    }
    checked_run(ctx, "git", &["reset", "--hard", "HEAD^"])?;
    println!("Finished unpulling.");
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let ctx = StdContext {};
    let args: Vec<String> = std::env::args().collect();
    let app = clap::Command::new("darcs-git").subcommand_required(true);

    let rec_args = [clap::Arg::new("files").trailing_var_arg(true).num_args(1..)];
    let rec = clap::Command::new("rec").args(rec_args);

    let rev_args = [clap::Arg::new("files").trailing_var_arg(true).num_args(1..)];
    let rev = clap::Command::new("rev").args(rev_args);

    let what_args = [
        clap::Arg::new("summary")
            .short('s')
            .long("summary")
            .action(clap::ArgAction::SetTrue),
        clap::Arg::new("files").trailing_var_arg(true).num_args(1..),
    ];
    let what = clap::Command::new("what").args(what_args);

    let p = clap::Command::new("push");

    let unr = clap::Command::new("unrec");

    let unp = clap::Command::new("unpull");

    let subcommands = vec![rec, rev, what, p, unr, unp];
    let matches = app.subcommands(subcommands).try_get_matches_from(args)?;
    let subcommand = matches.subcommand().context("subcommand failed")?;
    match subcommand {
        ("rec", args) => record(&ctx, args),
        ("rev", args) => revert(&ctx, args),
        ("what", args) => whatsnew(&ctx, args),
        ("push", _args) => push(&ctx),
        ("unrec", _args) => unrec(&ctx),
        ("unpull", _args) => unpull(&ctx),
        _ => Err(anyhow::anyhow!("unrecognized subcommand")),
    }
}
