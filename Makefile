build:
	cargo build --release

run-solo-alice:
	./target/release/gm-chain-node --chain solo-dev --alice --tmp

run-solo-bob:
	./target/release/gm-chain-node --chain solo-dev --bob --tmp --port 30334
