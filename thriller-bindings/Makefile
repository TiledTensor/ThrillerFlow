TEST ?= test_bindings.py
TEST_DIR := tests

.PHONY: build test

build:
	@cargo build
	@maturin develop

test: build 
	@python3 $(TEST_DIR)/$(TEST)