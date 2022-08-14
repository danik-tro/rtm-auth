.PHONY: install api api-prod test


install:
	cargo install --path .

api:
	cargo run --bin api

api-prod:
	cargo run --release --bin api

test:
	cargo test
