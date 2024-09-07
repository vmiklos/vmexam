# uchroot

## Description

Unprivileged chroot: a combination of `unshare`, `mount`, and `chroot`, all unprivileged, similar to
what `podman` does, but without all the stateless complexity. This allows keeping your state while
deciding you need one more bind mount, similar to how it was possible with privileged `chroot` in
the past.

## Installation

```
cargo install --git https://github.com/vmiklos/vmexam uchroot
```

## Configuration mechanism

Mounts are only performed in case a `mounts.conf` is provided in the current directory. Format is
the same as containers-mounts.conf(5), typical contents is:

```
/dev:/dev
/proc:/proc
/sys:/sys
```
