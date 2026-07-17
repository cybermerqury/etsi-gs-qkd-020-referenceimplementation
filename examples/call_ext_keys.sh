#!/usr/bin/env bash
# SPDX-FileCopyrightText: © 2026 Merqury Cybersecurity Ltd <info@merqury.eu>
# SPDX-License-Identifier: AGPL-3.0-only

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

curl \
  -i \
  --url "https://localhost:8888/kmapi/v1/ext_keys" \
  --cert "${SCRIPT_DIR}/../certificates/gateway_2.pem" \
  --key "${SCRIPT_DIR}/../certificates/gateway_2.key" \
  --cacert "${SCRIPT_DIR}/../certificates/root.pem" \
  --header "Content-Type: application/json" \
  --data-raw '{
  "keys": [
   {
     "key_id": "550e8400-e29b-41d4-a716-446655440000",
     "value": "wHHVxRwDJs3/bXd38GHP3oe4svTuRpZS0yCC7x4Ly+s="
   }
  ],
  "initiator_sae_id": "encryptor_1",
  "target_sae_ids": [
    "3rd_party_encryptor_1"
  ],
  "ack_callback_url": "https://localhost:8889/kmapi/v1/ext_keys/ack",
  "extension_mandatory": {}
}'
