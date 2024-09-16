build:
	cargo build

release:
	cargo build --release

test:
	cargo build --target x86_64-unknown-linux-gnu --features std --no-default-features
	cargo test --target x86_64-unknown-linux-gnu --features std --no-default-features

run:
	cargo run
