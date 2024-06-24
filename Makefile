default: test

build:
	soroban contract build

build-optimized:
	soroban contract build
	cd target/wasm32-unknown-unknown/release/ && \
		for i in *.wasm ; do \
			soroban contract optimize --wasm "$$i" --wasm-out "$$i.tmp" && mv "$$i.tmp" "$$i"; \
			ls -l "$$i"; \
		done

test: build
	cargo test fca00c_fast -- --nocapture

test-budget:
	cargo test fca00c_budget -- --nocapture

clean:
	cargo clean

fmt:
	cargo fmt --all