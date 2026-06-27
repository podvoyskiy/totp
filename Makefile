test:
	cargo test -- --nocapture

build:
	cargo build --release --target x86_64-unknown-linux-musl

t: test
b: build