/*
 * Copyright 2024 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Trivial wrapper around unshare, mount and chroot.

use anyhow::Context as _;

fn main() -> anyhow::Result<()> {
    let argv: Vec<String> = std::env::args().collect();
    let newroot_arg = clap::Arg::new("newroot");
    let command_arg = clap::Arg::new("command")
        .trailing_var_arg(true)
        .num_args(0..);
    let args = [newroot_arg, command_arg];
    let app = clap::Command::new("uchroot");
    let args = app.args(&args).try_get_matches_from(argv)?;
    let newroot: &str = args
        .get_one::<String>("newroot")
        .context("newroot is required")?;
    let command: Vec<String> = match args.get_many("command") {
        Some(value) => value.cloned().collect(),
        None => vec!["bash".to_string()],
    };

    // It's important to save these before we call unshare().
    let euid = nix::unistd::geteuid();
    let egid = nix::unistd::getegid();

    // Create new user and mount namespaces.
    nix::sched::unshare(
        nix::sched::CloneFlags::CLONE_NEWUSER | nix::sched::CloneFlags::CLONE_NEWNS,
    )?;

    // Map the current effective user and group IDs to root in the user
    // namespace.
    std::fs::write("/proc/self/uid_map", format!("0 {} 1", euid))
        .context("failed to write uid_map")?;
    std::fs::write("/proc/self/setgroups", "deny").context("failed to write setgroups")?;
    std::fs::write("/proc/self/gid_map", format!("0 {} 1", egid))
        .context("failed to write gid_map")?;

    // Create bind mounts.
    let none: Option<&'static [u8]> = None;
    nix::mount::mount(
        Some("/dev"),
        "dev",
        none,
        nix::mount::MsFlags::MS_BIND | nix::mount::MsFlags::MS_REC,
        none,
    )?;
    nix::mount::mount(
        Some("/proc"),
        "proc",
        none,
        nix::mount::MsFlags::MS_BIND | nix::mount::MsFlags::MS_REC,
        none,
    )?;
    nix::mount::mount(
        Some("/sys"),
        "sys",
        none,
        nix::mount::MsFlags::MS_BIND | nix::mount::MsFlags::MS_REC,
        none,
    )?;

    // Change the root dir.
    std::os::unix::fs::chroot(newroot)?;
    std::env::set_current_dir(std::path::Path::new("/"))?;

    // Start the command.
    let (first, rest) = command.split_first().context("missing command")?;
    std::process::Command::new(first).args(rest).status()?;

    Ok(())
}
