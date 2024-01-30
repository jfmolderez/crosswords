format:
	cargo fmt --quiet

lint: 
	cargo clippy --quiet

test:
	cargo test --all-features

run: 
	cargo run

all: format lint test run