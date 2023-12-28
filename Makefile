default: run

run:
	cargo run

watch:
	cargo-watch -qc -x run -x clippy

test:
	cargo-watch -qc -x test

build:
	cargo build --release

clean:
	cargo clean
