build: target/debug/addr-osmify

target/debug/addr-osmify: src/lib.rs src/main.rs Cargo.toml Makefile
	cargo build

check: build check-unit check-format check-clippy

check-format:
	cargo fmt -- --check

# Without coverage: cargo test --lib
# Custom target dir to avoid rebuilds, see <https://github.com/xd009642/tarpaulin/discussions/772>.
check-unit:
	cargo tarpaulin --lib -v --skip-clean --fail-under 100 --target-dir $(PWD)/target-cov

check-clippy:
	cargo clippy

tags:
	rusty-tags vi

.PHONY: tags
