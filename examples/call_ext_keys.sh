#! /bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

curl \
  -i \
  --url "https://localhost:8080/kmapi/v1/ext_keys" \
  --cert "${SCRIPT_DIR}/../certificates/gateway_2.crt" \
  --key "${SCRIPT_DIR}/../certificates/gateway_2.key" \
  --cacert "${SCRIPT_DIR}/../certificates/root.crt" \
  --header "Content-Type: application/json" \
  --data-raw '{
  "keys": [
   {
     "key_id": "550e8400-e29b-41d4-a716-446655440000",
     "value": "wHHVxRwDJs3/bXd38GHP3oe4svTuRpZS0yCC7x4Ly+s="
   }
  ],
  "initiator_sae_id": "encryptor1",
  "target_sae_ids": [
    "encryptor2"
  ]
}'
