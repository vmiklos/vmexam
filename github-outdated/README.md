# github-outdated

Simple cmdline tool that scans the `.github/workflows/` for workflow files and tries to flag
outdated dependencies.

So far it checks:

- the version of the used actions
- if a Rust toolchain version is used, that version (vs local `rustc --version`)

## Installation

```
cargo install --git https://github.com/vmiklos/vmexam github-outdated
```
