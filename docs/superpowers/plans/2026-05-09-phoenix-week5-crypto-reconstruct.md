# Phoenix Week 5 — Cryptographic Core (BIP-39 reconstruction + address verification)

**Goal:** Turn the candidate fragments produced by Layers 1–2 into an *actual recovery attempt*. Given a partial 12-word BIP-39 mnemonic and a target on-chain address, Phoenix derives candidate seeds, computes addresses across BTC/ETH derivation paths, and verifies against the target. This is the layer that makes Phoenix recover wallets, not just collect hints.

**Architecture:** A new `phoenix-core::crypto` module with three sub-modules:

1. `mnemonic.rs` — Wraps the `bip39` crate. Provides mnemonic ↔ seed helpers and BIP-39 checksum validation.
2. `derive.rs` — Hierarchical derivation. Seed → master `XPrv` via `bip32`, then BIP-44 paths for BTC native segwit (P2WPKH, m/84'/0'/0'/0/i) and Ethereum (m/44'/60'/0'/0/i).
3. `address.rs` — secp256k1 public key → BTC P2WPKH (bech32) and ETH (keccak256 last-20-bytes) addresses.
4. `reconstruct.rs` — Single-missing-word brute force. Given 11 known words + position, iterates the BIP-39 wordlist (filtered to checksum-valid candidates) and verifies derived address against the target.

**Tech Stack:** bip39 (already), bip32, secp256k1, bitcoin_hashes (sha256+ripemd160), bech32, tiny-keccak, hex.

**Scope cuts:**
- Single missing word only (multiple missing → Week 6)
- BTC native segwit + ETH only (SOL/BSC/etc. → later)
- Standard BIP-44 paths only (custom passphrase or non-standard derivation → Week 6)
- No Hashcat integration yet (single-word brute is fast in pure Rust)
