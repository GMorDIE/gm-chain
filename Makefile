build:
	cargo build --release

run-solo-alice:
	./target/release/gm-chain-node --chain solo-dev --alice --tmp

run-solo-bob:
	./target/release/gm-chain-node --chain solo-dev --bob --tmp --port 30334

run-solo: ; printf "run-solo-alice\nrun-solo-bob" | parallel -u make


.PHONY: setup-testing purge-testing download-relay generate-relay-raw-chainspec build generate-both copy-collator-to-testing

generate-genesis-wasm:
	./target/release/gm-chain-node export-genesis-wasm > testing/genesis-wasm

generate-genesis-state:
	./target/release/gm-chain-node export-genesis-state > testing/genesis-state

generate-both: generate-genesis-state generate-genesis-wasm

download-relay:
	wget -O testing/polkadot "https://github.com/paritytech/polkadot/releases/download/v0.9.22/polkadot" && \
	chmod +x testing/polkadot

generate-relay-raw-chainspec:
	./testing/polkadot build-spec --chain rococo-local --disable-default-bootnode --raw > ./testing/rococo-chainspec-raw.json

run-relay-alice:
	./testing/polkadot --chain ./testing/rococo-chainspec-raw.json --alice --tmp

run-relay-bob:
	./testing/polkadot --chain ./testing/rococo-chainspec-raw.json --bob --tmp --port 30334

copy-collator-to-testing:
	cp ./target/release/gm-chain-node ./testing/

# Safely purge testing directory by only removing the files we use
purge-testing:
	mkdir -p ./testing && \
	rm -f ./testing/rococo-chainspec-raw.json \
				./testing/polkadot \
				./testing/gm-chain-node \
				./testing/genesis-state \
				./testing/genesis-wasm

run-parachain-collator:
	./testing/gm-chain-node \
		--collator \
		--alice \
		--force-authoring \
		--tmp \
		--port 40335 \
		--ws-port 8844 \
		-- \
		--execution wasm \
		--chain ./testing/rococo-chainspec-raw.json \
		--port 30335

setup-testing: | purge-testing download-relay generate-relay-raw-chainspec build generate-both copy-collator-to-testing
	$(info Setup finished, here's how to proceed with testing:)
	$(info Open 3 terminals, all on $(CURDIR))
	$(info Terminal 1: make run-relay-alice)
	$(info Terminal 2: make run-relay-bob)
	$(info Terminal 3: make run-parachain-collator)
