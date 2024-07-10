prepare_env:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain stable -y
	rustup target add x86_64-unknown-linux-musl
	rustup component add llvm-tools-preview
	cargo install cargo-audit
	cargo install cargo-machete
	cargo install cargo-tarpaulin
	cargo install grcov

audit:
	cargo audit

build:
	cargo build

release:
	cargo build --release --all-features

format:
	cargo fmt

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --workspace -- --test-threads=1

coverage:
	export RUSTFLAGS="-Cinstrument-coverage"
	export LLVM_PROFILE_FILE="rusty_ops-%p-%m.profraw"
	cargo build
	cargo test --workspace -- --test-threads=1
	grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/coverage/
	grcov . --binary-path ./target/debug/ -s . -t lcov --branch --ignore-not-existing -o ./lcov.info

cleanup:
	find . -name '*.profraw' -type f -delete
	rm ./lcov.info
	rm -rf target

precommit:
	git pull
	make build
	make format
	make lint

docker_build_server:
	docker build -t rusty-server -f rusty_server/Dockerfile .

docker_build_agent:
	docker build -t rusty-agent -f rusty_agent/Dockerfile .

docker_build_init:
	docker build -t rusty-init -f rusty_init/Dockerfile .
