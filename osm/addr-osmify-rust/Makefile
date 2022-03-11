build: target/debug/addr-osmify

target/debug/addr-osmify: src/lib.rs src/main.rs Cargo.toml Makefile
	cargo build

check: build check-unit check-format check-clippy

check-format:
	cargo fmt -- --check

# Without coverage: cargo test --lib
check-unit:
	cargo llvm-cov --lib --no-report
	cargo llvm-cov --lib --no-run --show-missing-lines --fail-under-lines 100

check-clippy:
	cargo clippy

tags:
	rusty-tags vi

.PHONY: tags
