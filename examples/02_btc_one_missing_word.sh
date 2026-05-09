#!/usr/bin/env bash
# Phoenix example 2 — see examples/README.md for context.
# This uses the public BIP-39 zero vector "abandon x 11 about" — zero funds.
# Replace with your own inputs to recover a real wallet.

set -e
./target/release/phoenix.exe reconstruct \
    --template "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?" \
    --target  "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu" \
    --kind    btc
