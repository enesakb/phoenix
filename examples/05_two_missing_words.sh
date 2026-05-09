#!/usr/bin/env bash
# Phoenix example 5 — see examples/README.md for context.
# This uses the public BIP-39 zero vector "abandon x 11 about" — zero funds.
# Replace with your own inputs to recover a real wallet.

set -e
./target/release/phoenix.exe reconstruct \
    --template "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ? ?" \
    --target  "0x9858effd232b4033e47d90003d41ec34ecaeda94" \
    --kind    eth
