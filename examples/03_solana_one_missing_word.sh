#!/usr/bin/env bash
# Phoenix example 3 — see examples/README.md for context.
# This uses the public BIP-39 zero vector "abandon x 11 about" — zero funds.
# Replace with your own inputs to recover a real wallet.

set -e
./target/release/phoenix.exe reconstruct \
    --template "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?" \
    --target  "HAgk14JpMQLgt6rVgv7cBQFJWFto5Dqxi472uT3DKpqk" \
    --kind    sol
