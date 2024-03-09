.PHONY: install api api-prod test

run: 
	cargo run -- --config ${CONFIG} run-server --bind ${BIND} 

install:
	cargo install --path .

api:
	cargo run --bin api

api-prod:
	cargo run --release --bin api

test:
	cargo test

db-setup:
	diesel setup

migrate:
	diesel migration run

generate_schema:
	diesel print-schema > src/schema.rs

generate_models:
	diesel_ext --model > src/models.rs
