EXAMPLE 	?= rf_block

.PHONY: build test clean example

build:
	@cargo build 

test:
	@cargo test -- --nocapture

example:
	@cargo run --example $(EXAMPLE)

clean:
	@cargo clean