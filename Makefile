prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p payments --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/payment_processor_contract.wasm 
	wasm-strip target/wasm32-unknown-unknown/release/execute_payment.wasm 

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
