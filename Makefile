prepare_env:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain stable -y
	rustup target add x86_64-unknown-linux-musl
	rustup target add wasm32-unknown-unknown
	cargo install cargo-machete
	cargo install cargo-tarpaul

audit:
	cargo audit

build:
	cargo build

format:
	cargo fmt

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	 cargo test --workspace -- --test-threads=1

coverage:
	cargo tarpaulin -- --test-threads=1

precommit:
	git pull
	make build
	make format
	make lint

build_server:
	docker build -t rusty-server -f rusty_server/Dockerfile .

build_agent:
	docker build -t rusty-agent -f rusty_agent/Dockerfile .

build_web:
	docker build -t rusty-web -f rusty_web/Dockerfile .
