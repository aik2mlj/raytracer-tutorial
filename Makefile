run:
	cargo run

run_release:
	cargo run --release

fmt:
	cargo fmt

clippy:
	cargo clippy --all-targets --all-features

test:
	cargo test --all-features

ci: fmt clippy test run_release

clean:
	cargo clean

.PHONY: run clean fmt clippy test
