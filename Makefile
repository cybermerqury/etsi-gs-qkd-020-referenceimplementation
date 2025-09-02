include ./.env

# Environment variables.
CURDIR=$(dir $(realpath $(lastword $(MAKEFILE_LIST))))
CERTS_DIR?=$(CURDIR)certs
export $(shell sed 's/=.*//' .env)

.PHONY: setup run_server run_server_release clean
.PHONY: run_tests run_ext_keys run_invalid_ext_keys
.PHONY: build build_release build_image build_clean
.SILENT:

# ------------------------------------------------------------------------------
# Environment setup.
# ------------------

setup:
	$(MAKE) -C certificates certs

run_server: build
	cargo run

run_server_release: build_release
	cargo run --release

clean:
	$(MAKE) -C certificates clean
	$(MAKE) build_clean

# ------------------------------------------------------------------------------
# Tests.
# ------
run_tests:
	cargo test

run_ext_keys:
	./examples/call_ext_keys.sh

run_invalid_ext_keys:
	./examples/call_invalid_ext_keys.sh

# ------------------------------------------------------------------------------
# Build resources.
# ----------------

build:
	cargo build --workspace

build_release:
	cargo build --release --workspace

build_image: build_release
	docker build -t merqury/etsi_020_ref_impl:1.0.0 -f Dockerfile .

build_clean:
	cargo clean

# ------------------------------------------------------------------------------
