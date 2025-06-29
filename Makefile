default: run

run:
	cargo run --features=dotenv

watch:
	cargo-watch -qc -x "run --features=dotenv" -x clippy

test:
	cargo-watch -qc -x test

test-ci:
	cargo test

gemini-test:
	cargo test
	$(MAKE) test-docker

test-docker:
	./scripts/test_docker_build.sh

build:
	cargo build --release

clean:
	cargo clean
