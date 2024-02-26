format:
	cargo fmt --quiet

lint: 
	cargo clippy --quiet

test:
	cargo test -- --nocapture

run: 
	cargo run

all: format lint test run