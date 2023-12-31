default: run

run:
	cargo run --features=dotenv

watch:
	cargo-watch -qc -x "run --features=dotenv" -x clippy

test:
	cargo-watch -qc -x test

build:
	cargo build --release

clean:
	cargo clean
