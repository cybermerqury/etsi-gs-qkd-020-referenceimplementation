#! /bin/bash

curl                                              \
  -i                                              \
  --url "http://localhost:8080/kmapi/v1/ext_keys" \
  --header "Content-Type: application/json"       \
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
