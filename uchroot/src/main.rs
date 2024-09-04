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

/// Control the shared execution context without creating a new process.
fn unshare(flags: libc::c_int) -> anyhow::Result<()> {
    match unsafe { libc::unshare(flags) } {
        0 => Ok(()),
        _ => Err(anyhow::anyhow!(
            "failed to unshare: {}",
            std::io::Error::last_os_error()
        )),
    }
}

/// Attaches the filesystem specified by `source` to the location specified by `target`.
fn mount(source: &str, target: &str, flags: libc::c_ulong) -> anyhow::Result<()> {
    let c_source = std::ffi::CString::new(source)?;
    let c_target = std::ffi::CString::new(target)?;
    match unsafe {
        libc::mount(
            c_source.as_ptr(),
            c_target.as_ptr(),
            std::ptr::null(),
            flags,
            std::ptr::null(),
        )
    } {
        0 => Ok(()),
        _ => Err(anyhow::anyhow!(
            "failed to mount {} to {}: {}",
            source,
            target,
            std::io::Error::last_os_error()
        )),
    }
}

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
    unshare(libc::CLONE_NEWUSER | libc::CLONE_NEWNS)?;

    // Map the current effective user and group IDs to root in the user
    // namespace.
    std::fs::write("/proc/self/uid_map", format!("0 {} 1", euid))
        .context("failed to write uid_map")?;
    std::fs::write("/proc/self/setgroups", "deny").context("failed to write setgroups")?;
    std::fs::write("/proc/self/gid_map", format!("0 {} 1", egid))
        .context("failed to write gid_map")?;

    // Create bind mounts.
    mount("/dev", "dev", libc::MS_BIND | libc::MS_REC)?;
    mount("/proc", "proc", libc::MS_BIND | libc::MS_REC)?;
    mount("/sys", "sys", libc::MS_BIND | libc::MS_REC)?;

    // Change the root dir.
    std::os::unix::fs::chroot(newroot)?;
    std::env::set_current_dir(std::path::Path::new("/"))?;

    // Start the command.
    let (first, rest) = command.split_first().context("missing command")?;
    std::process::Command::new(first).args(rest).status()?;

    Ok(())
}
