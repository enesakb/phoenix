# Phoenix roadmap

This document tracks the quarterly direction of Phoenix from v0.8 (today) to
v1.0 (production-ready). Items move from "Planned" → "In progress" → "Shipped"
as work lands.

The order below reflects priority — earlier items unlock later ones.

## Shipped (v0.1 – v0.8)

- **v0.1 — Foundation** (`week-1-foundation`)
  - Tauri 2 + Rust workspace, React + TS frontend
  - Ollama LLM client, config / logging / telemetry frameworks
  - GitHub Actions CI for Linux + Windows + macOS
- **v0.2 — Cognitive Interview MVP** (`week-2-cognitive-interview`)
  - 50-question Fisher–Geiselman bank
  - LLM-driven memory-hint extractor with multi-agent debate
  - JSON-on-disk session persistence
- **v0.3 — Forensic Layer A** (`week-3-forensic-a`)
  - Bitwarden CSV, Chrome History SQLite, BIP-39 text scanners
  - File-extension dispatching extractor registry
- **v0.5 — Cryptographic Core** (`week-5-crypto-reconstruct`)
  - BIP-39 ↔ BIP-32 ↔ secp256k1 derivation chain
  - BTC P2WPKH (bech32) + ETH (keccak256) addresses
  - Rayon-parallel single-missing-word brute force
  - Standard BIP-39 test vectors locked
- **v0.6 — Extended Recovery** (`week-4-6-extended`)
  - Two-missing-word brute force (Rayon cartesian product)
  - BIP-39 passphrase brute force
  - Hashcat command builder for 7 wallet formats
  - mbox email scanner
  - Threat model v1
- **v0.7 — Solana** (`week-7-solana`)
  - ed25519 + SLIP-10 derivation across Phantom / Solflare / Sollet paths
  - Cross-verified against Phantom JS pipeline (bip39 + ed25519-hd-key)
  - Locked-vector regression test
- **v0.8 — Multi-platform binaries + payment infrastructure**
  - GitHub Actions release workflow producing Linux + Windows + macOS binaries
  - `phoenix wallet-create` CLI for fresh BIP-39 generation
  - Public donation addresses (BTC + EVM + Solana) registered
  - Post-recovery donation prompt in CLI
  - Pages-deployed landing site

## In progress

- **v0.8.x — UX polish**
  - More descriptive error messages with possible-cause lists ✓ shipped
  - All site CTAs link to real URLs (no fake anchors, no fake email) ✓ shipped
  - Examples directory with runnable shell scripts ✓ shipped
  - SHA256SUMS published with every release ✓ shipped

## v0.9 — Atomic split contract (Q3 2026)

Goal: Move the recovery success fee from voluntary CLI prompt to a
user-signed atomic on-chain split.

- Solidity contract `PhoenixSplit` with audited 95/5 split logic
- Deployed across Ethereum, Base, Optimism, Arbitrum, Monad
- Trail of Bits or Spearbit audit before mainnet deployment
- UI flow: post-recovery prompt presents pre-built tx; user signs in their
  wallet (MetaMask, Phantom for SOL); contract enforces atomic split
- Solana program (anchor-based) for SOL-side recoveries
- v0.9 ships only after the audit report is public

## v1.0 — Pilot validation (Q4 2026)

Goal: Demonstrate product-market fit through 5 real, anonymized recovery
case studies before opening the Pro tier.

- 5 verified successful pilot recoveries from real users
- Anonymized case studies published to landing page
- Trail of Bits audit report published
- Reproducible-build verification documented
- README and site updated with real performance data from production usage

## v1.1 — Pro tier launch (Q1 2027)

Goal: Recurring revenue tier for active recoverers.

- Legal entity registered (Switzerland or Estonia depending on counsel review)
- Stripe-based $99/mo subscription billing
- Terms of Service and Privacy Policy drafted by counsel
- AML/KYC vendor selected (Sumsub / Persona / Onfido)
- Pro tier features: 2-word brute, passphrase brute, Hashcat command builder
- 30-day free trial for waitlist members

## v1.2+ — Forensic Layer B (Q2 2027)

- KeePass kdbx parser with interactive master-password UX
- 1Password export parser
- Tesseract OCR for photographed seeds
- iCloud / Google Drive / Dropbox local-backup parsers
- Email backup parser beyond mbox (PST, OST, native client formats)
- Apple Notes parser

## v1.3+ — Constraint Inference (Q3 2027)

- Bayesian network combining cognitive + forensic hints
- Hidden Markov Model for per-user pattern fingerprinting
- PassGPT-style transformer fine-tuned on user pattern history
- AC-3 constraint propagation for multi-hint consolidation
- Markov chain Monte Carlo sampling for low-prob candidates

## v1.4+ — Federated Learning (Q4 2027)

- Differential-privacy aggregation across opted-in recoveries
- Cross-recovery model improvement (interview policy gets smarter weekly)
- Secure aggregation protocol (Bonawitz et al. 2017)
- Privacy budget displayed to user before opt-in

## v2.0 — Mobile + multi-chain expansion (2028)

- iOS app (interview + report only; cracking stays on desktop)
- Android app
- Multilingual interview (Turkish, German, Spanish, Mandarin, Arabic)
- Cosmos / Polkadot / Algorand / Cardano derivation paths
- Hardware-wallet pre-protection partnership (Ledger, Trezor, Coldcard)
- Insurance partnership: pre-protection insurance + recovery rider

## What's deliberately not on the roadmap

- **Custodial wallet hosting** — Phoenix never holds user keys.
- **Recovery-as-a-service** — recoveries always happen on the user's machine.
- **Hardware-glitch attacks** — Praefortis / Unciphered handle Trezor PIN, etc.
- **Quantum decryption** — when Q-Day arrives, the protocol will already be
  post-quantum. Phoenix is not a Q-Day vendor.

## How priorities shift

This roadmap is responsive to two signals:

1. **Pilot user feedback.** The first 5 successful recoveries will tell us
   which forensic surface matters most. If everyone needs KeePass kdbx
   support, that jumps to v0.9 priority. If no one ever asks, it drops out.
2. **Crypto landscape changes.** New chains, new wallet defaults, new
   cryptographic primitives — we will support what users actually have.

Roadmap discussion: [GitHub Discussions](https://github.com/enesakb/phoenix/discussions).
