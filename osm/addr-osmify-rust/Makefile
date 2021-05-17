build: target/debug/addr-osmify

target/debug/addr-osmify: src/lib.rs src/main.rs Makefile
	cargo build

check: build check-unit check-format check-clippy

check-format: src/lib.rs src/main.rs Makefile
	rustfmt --check src/lib.rs src/main.rs

check-unit:
	cargo test --lib

check-clippy:
	cargo clippy
