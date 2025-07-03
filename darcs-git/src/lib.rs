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
use std::io::Write as _;

/// Context interface.
pub trait Context {
    /// Executes a command as a child process, waiting for it to finish and
    /// collecting its status.
    fn command_status(&self, command: &str, args: &[&str]) -> anyhow::Result<i32>;

    /// Executes the command as a child process, waiting for it to finish and
    /// collecting all of its output.
    fn command_output(&self, command: &str, args: &[&str]) -> anyhow::Result<String>;

    /// Returns the arguments that this program was started with (normally passed
    /// via the command line).
    fn env_args(&self) -> Vec<String>;

    /// Prints to the standard output.
    fn print(&self, string: &str);

    /// Reads a line from the standard input.
    fn readln(&self) -> anyhow::Result<String>;

    /// Reads a single character from the standard input.
    fn readch(&self) -> anyhow::Result<String>;
}

fn flushed_print(ctx: &dyn Context, question: &str) -> anyhow::Result<()> {
    ctx.print(&format!("{question} "));
    Ok(std::io::stdout().flush()?)
}

fn ask_string(ctx: &dyn Context, question: &str) -> anyhow::Result<String> {
    flushed_print(ctx, question)?;
    ctx.readln()
}

fn ask_char(ctx: &dyn Context, question: &str) -> anyhow::Result<String> {
    flushed_print(ctx, question)?;
    let ret = ctx.readch()?;
    println!("{ret}");
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
        ctx.print("Ok, if you don't want to record anything, that's fine!\n");
        return Ok(());
    }
    let mut add = vec!["add", "--patch"];
    for file in &args.files {
        add.push(file);
    }
    checked_run(ctx, "git", &add)?;
    let message = ask_string(ctx, "What is the commit message?")?;
    let edit: bool;
    loop {
        let ret = ask_char(ctx, "Do you want to add a long comment? [ynq]")?;
        if ret == "q" {
            return Ok(());
        } else if ret == "y" || ret == "n" {
            edit = ret == "y";
            break;
        }
        ctx.print("Invalid response, try again!\n");
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
        ctx.print("Ok, if you don't want to revert anything, that's fine!\n");
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
        ctx.print("No changes!\n");
    }
    Ok(())
}

fn push(ctx: &dyn Context) -> anyhow::Result<()> {
    let output = ctx.command_output("git", &["log", "HEAD@{upstream}.."])?;
    if output.is_empty() {
        ctx.print("No recorded local changes to push!\n");
        return Ok(());
    }
    println!("{output}");
    loop {
        let ret = ask_char(ctx, "Do you want to push these patches? [ynq]")?;
        if ret == "n" || ret == "q" {
            return Ok(());
        } else if ret == "y" {
            break;
        }
        ctx.print("Invalid response, try again!\n");
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
        let ret = ask_char(ctx, "Do you want to unrecord this patch? [ynq]")?;
        if ret == "n" || ret == "q" {
            return Ok(());
        } else if ret == "y" {
            break;
        }
        ctx.print("Invalid response, try again!\n");
    }
    checked_run(ctx, "git", &["reset", "--quiet", "HEAD^"])?;
    ctx.print("Finished unrecording.\n");
    Ok(())
}

fn unpull(ctx: &dyn Context) -> anyhow::Result<()> {
    checked_run(ctx, "git", &["log", "-1"])?;
    loop {
        let ret = ask_char(ctx, "Do you want to unpull this patch? [ynq]")?;
        if ret == "n" || ret == "q" {
            return Ok(());
        } else if ret == "y" {
            break;
        }
        ctx.print("Invalid response, try again!\n");
    }
    checked_run(ctx, "git", &["reset", "--hard", "HEAD^"])?;
    ctx.print("Finished unpulling.\n");
    Ok(())
}

fn get_subcommands() -> Vec<clap::Command> {
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

    let push = clap::Command::new("push");

    let unrec = clap::Command::new("unrec");

    let unpull = clap::Command::new("unpull");

    vec![rec, rev, what, push, unrec, unpull]
}

/// Similar to plain main(), but with an interface that allows testing.
pub fn main(ctx: &dyn Context) -> anyhow::Result<()> {
    let app = clap::Command::new("darcs-git").subcommand_required(true);

    // This will fail when the subcommand is unrecognized.
    let matches = app
        .subcommands(get_subcommands())
        .try_get_matches_from(ctx.env_args())?;

    let subcommand = matches.subcommand().context("subcommand failed")?;
    handle_subcommand(ctx, subcommand)
}

fn handle_subcommand(
    ctx: &dyn Context,
    subcommand: (&str, &clap::ArgMatches),
) -> anyhow::Result<()> {
    match subcommand {
        ("rec", args) => record(ctx, args),
        ("rev", args) => revert(ctx, args),
        ("what", args) => whatsnew(ctx, args),
        ("push", _args) => push(ctx),
        ("unrec", _args) => unrec(ctx),
        ("unpull", _args) => unpull(ctx),
        _ => Err(anyhow::anyhow!("unrecognized subcommand")),
    }
}

#[cfg(test)]
mod tests;
