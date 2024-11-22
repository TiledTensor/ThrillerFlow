TEST ?= pass/allocate_var.py
TEST_DIR := tests

.PHONY: build test

build:
	@cargo build
	@maturin develop

active:
	@source .env/bin/activate

test: build 
	@python3 $(TEST_DIR)/$(TEST)