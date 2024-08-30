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

fn mount_rbind(from: &str, to: &str) -> anyhow::Result<()> {
    let from = std::ffi::CString::new(from)?;
    let to = std::ffi::CString::new(to)?;
    if unsafe {
        libc::mount(
            from.as_ptr(),
            to.as_ptr(),
            std::ptr::null(),
            libc::MS_BIND | libc::MS_REC,
            std::ptr::null(),
        )
    } < 0
    {
        return Err(anyhow::anyhow!("mount() failed"));
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    // It's important to save these before we call unshare().
    let euid = unsafe { libc::geteuid() };
    let egid = unsafe { libc::getegid() };

    // Create new user and mount namespaces.
    let unshare_flags: libc::c_int = libc::CLONE_NEWUSER | libc::CLONE_NEWNS;
    if unsafe { libc::unshare(unshare_flags) } < 0 {
        return Err(anyhow::anyhow!("unshare() failed"));
    }

    // Map the current effective user and group IDs to root in the user
    // namespace.
    std::fs::write("/proc/self/uid_map", format!("0 {} 1", euid))
        .context("failed to write uid_map")?;
    std::fs::write("/proc/self/setgroups", "deny").context("failed to write setgroups")?;
    std::fs::write("/proc/self/gid_map", format!("0 {} 1", egid))
        .context("failed to write gid_map")?;

    // Create bind mounts.
    mount_rbind("/dev", "dev")?;
    mount_rbind("/proc", "proc")?;
    mount_rbind("/sys", "sys")?;

    // Change the root dir.
    let current_dir = std::ffi::CString::new(".")?;
    if unsafe { libc::chroot(current_dir.as_ptr()) } < 0 {
        return Err(anyhow::anyhow!("chroot() failed"));
    }
    std::env::set_current_dir(std::path::Path::new("/"))?;

    // Start bash.
    std::process::Command::new("bash").status()?;

    Ok(())
}
