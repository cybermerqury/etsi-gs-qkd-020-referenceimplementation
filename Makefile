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

run_server: run_server_a

run_server_a: build
	echo Starting Alice server.
	ETSI_020_REF_IMPL_ROOT_CERT=$(A_ETSI_020_REF_IMPL_ROOT_CERT) \
	ETSI_020_REF_IMPL_PRIVATE_KEY=$(A_ETSI_020_REF_IMPL_PRIVATE_KEY) \
	ETSI_020_REF_IMPL_PUBLIC_CERT=$(A_ETSI_020_REF_IMPL_PUBLIC_CERT) \
	ETSI_020_REF_IMPL_PORT_NUM=$(A_ETSI_020_REF_IMPL_PORT_NUM) \
	ETSI_020_REF_IMPL_IP_ADDR=$(A_ETSI_020_REF_IMPL_IP_ADDR) \
	ETSI_020_REF_IMPL_INTRA_NETWORK_SAE_IDS=$(A_ETSI_020_REF_IMPL_INTRA_NETWORK_SAE_IDS) \
	ETSI_020_REF_IMPL_THIRD_PARTY_SAE_IDS=$(A_ETSI_020_REF_IMPL_THIRD_PARTY_SAE_IDS) \
	    cargo run

run_server_b: build
	echo Starting Bob server.
	ETSI_020_REF_IMPL_ROOT_CERT=$(B_ETSI_020_REF_IMPL_ROOT_CERT) \
	ETSI_020_REF_IMPL_PRIVATE_KEY=$(B_ETSI_020_REF_IMPL_PRIVATE_KEY) \
	ETSI_020_REF_IMPL_PUBLIC_CERT=$(B_ETSI_020_REF_IMPL_PUBLIC_CERT) \
	ETSI_020_REF_IMPL_PORT_NUM=$(B_ETSI_020_REF_IMPL_PORT_NUM) \
	ETSI_020_REF_IMPL_IP_ADDR=$(B_ETSI_020_REF_IMPL_IP_ADDR) \
	ETSI_020_REF_IMPL_INTRA_NETWORK_SAE_IDS=$(B_ETSI_020_REF_IMPL_INTRA_NETWORK_SAE_IDS) \
	ETSI_020_REF_IMPL_THIRD_PARTY_SAE_IDS=$(B_ETSI_020_REF_IMPL_THIRD_PARTY_SAE_IDS) \
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
# Run examples
# ------------

example_call_ext_keys_async:
	cargo run --bin call_ext_keys_async
