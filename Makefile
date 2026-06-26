run:
	cargo run

test:
	cargo test -- --nocapture

build:
	cargo build --release --target x86_64-unknown-linux-musl

clean:
	cargo clean

r: run
t: test
b: build
c: clean