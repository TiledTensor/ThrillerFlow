.PHONY: build test clean

build:
	@cargo build 

test:
	@cargo test -- --nocapture

clean:
	@cargo clean