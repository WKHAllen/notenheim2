all: build

build:
	cd frontend && trunk build && cd .. && cd backend && cargo build

run:
	cd backend && cargo run

test:
	cargo test

lint:
	cargo clippy -- -D warnings

clean:
	cd frontend && trunk clean && cd .. && cargo clean
