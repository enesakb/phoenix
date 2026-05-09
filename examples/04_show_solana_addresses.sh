#!/usr/bin/env bash
# Phoenix example 4 — see examples/README.md for context.
# This uses the public BIP-39 zero vector "abandon x 11 about" — zero funds.
# Replace with your own inputs to recover a real wallet.

set -e
./target/release/phoenix.exe solana-show \
    --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
