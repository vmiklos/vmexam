# addr-osmify-rust

Takes an nominatim query (e.g. 'Mészáros utca 58/a, Budapest') and turns it
into a string that is readable (so that you can save it to your contacts) and
is also machine-friendly, e.g. OsmAnd can parse it as well.

This implementation is written in Rust:

- [x] static typing

- [x] consistent code formatting (rustfmt)

- [x] documentation (this file for users, code comments for developers)

- [x] tests (100% line coverage)

- [x] static code analysis (cargo clippy)

## Install

```
cargo build --release
cargo run
```
