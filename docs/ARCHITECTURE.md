# Phoenix architecture

End-to-end view of how Phoenix turns "I lost access to my wallet" into a
recovered seed phrase. Read this if you're a contributor, an auditor, or a
journalist who wants to verify our claims.

## High-level data flow

```
                ┌─────────────────────────────────────────────────────┐
                │  User (lost wallet, on their own machine)           │
                └─────────────────────────────────────────────────────┘
                                       │
                  cognitive interview  │  partial-info uploads
                                       ▼
        ┌────────────────────────────────────────────────────────────┐
        │                  L1  Cognitive Excavation                  │
        │  Local LLM (Llama 3.3 / Qwen 3) runs Fisher–Geiselman       │
        │  protocol. Multi-agent debate (3 personas) cross-questions │
        │  the user. Output: ranked memory hints.                    │
        └────────────────────────────────────────────────────────────┘
                                       │
                                       ▼
        ┌────────────────────────────────────────────────────────────┐
        │                L2  Digital Forensic Excavator              │
        │  Bitwarden CSV · Chrome History SQLite · mbox archives ·   │
        │  any plain text. (KeePass kdbx + Tesseract OCR roadmap.)   │
        │  Each extractor produces typed MemoryNodes.                │
        └────────────────────────────────────────────────────────────┘
                                       │
                                       ▼
        ┌────────────────────────────────────────────────────────────┐
        │             L3  Constraint Propagation & Inference         │
        │           ROADMAP — Bayesian + CSP + HMM + PassGPT         │
        │  Today: pure scoring + sorting from MemoryState.           │
        └────────────────────────────────────────────────────────────┘
                                       │
                                       ▼
        ┌────────────────────────────────────────────────────────────┐
        │                  L4  Distributed Cracking                  │
        │  Pure-Rust brute force in Rayon parallel. For loads beyond │
        │  CPU budget, emits exact Hashcat command strings to paste  │
        │  on a GPU rig. Phoenix never executes Hashcat itself.      │
        └────────────────────────────────────────────────────────────┘
                                       │
                                       ▼
        ┌────────────────────────────────────────────────────────────┐
        │              L5  Verification & Restoration                │
        │  BIP-39 → BIP-32 → secp256k1 / ed25519                     │
        │  BTC P2WPKH bech32 · ETH keccak256 · Solana base58         │
        │  Match candidate-derived address against user-supplied     │
        │  target. Recovery succeeds = found exact match.            │
        └────────────────────────────────────────────────────────────┘
                                       │
                                       ▼
                            Recovered seed phrase
                            (printed to user's terminal)
```

## Module layout

```
crates/phoenix-core/src/
├── lib.rs               Module re-exports
├── config.rs            TOML loader for $XDG_CONFIG_HOME/phoenix.toml
├── logging.rs           tracing-subscriber with non-blocking file appender
├── telemetry.rs         Opt-in anonymous event emitter (default: off)
├── llm/
│   ├── mod.rs           LlmClient async trait
│   └── ollama.rs        HTTP client for the local Ollama daemon
├── interview/           Layer 1
│   ├── mod.rs
│   ├── questions.rs     50-question bank loader (questions.json embedded)
│   ├── state.rs         MemoryState graph + typed MemoryNode
│   ├── session.rs       JSON-on-disk persistence with atomic writes
│   ├── interviewer.rs   LLM-driven hint extractor + JSON parsing
│   ├── debate.rs        Multi-agent cross-questioning loop
│   └── candidate.rs     Pure scoring + sorting
├── forensic/            Layer 2
│   ├── mod.rs           BIP-39 word list helper
│   ├── registry.rs      Extension-dispatched ExtractorRegistry
│   ├── bip39_text.rs    Universal text scanner
│   ├── bitwarden_csv.rs Bitwarden CSV exports
│   ├── chrome_history.rs Chrome / Edge / Brave History SQLite
│   └── mbox.rs          RFC-822 mbox archives (Gmail Takeout / IMAP)
└── crypto/              Layer 5
    ├── mod.rs           Re-exports + the AddressKind enum
    ├── mnemonic.rs      BIP-39 wrapper, generate_fresh_mnemonic
    ├── derive.rs        BIP-32 hierarchical derivation (secp256k1)
    ├── address.rs       BTC P2WPKH bech32, ETH keccak256
    ├── solana.rs        ed25519 + SLIP-10 across 3 wallet paths
    ├── reconstruct.rs   1- and 2-word brute force, passphrase brute force
    └── hashcat.rs       Command builder for GPU offload (Layer 4)

crates/phoenix-cli/src/main.rs
  Subcommands: doctor, ollama-check, reconstruct, solana-show, wallet-create
  Post-recovery donation prompt with all three chain addresses

src-tauri/src/
  Tauri 2 desktop shell exposing the same recovery API to a React frontend
  Currently: app_info command. Future: Tauri commands for interview / forensic / reconstruct

src-ui/src/
  React + Vite + TS strict frontend, Vitest for tests
  Routes: /interview, /candidates/:id, /reconstruct, /import/:id, /settings
```

## The local-only invariant

Phoenix's core promise is that user seeds, private keys, partial mnemonics,
and recovery candidates never leave the user's machine. The invariant is
maintained by these structural rules:

1. **Cracking is pure CPU work.** No HTTP client is invoked inside the
   reconstruct pipeline. The only file `reqwest` is imported in is `llm/ollama.rs`,
   which talks to `localhost:11434`, never out to the public internet.
2. **The LLM runs on the user's hardware.** Ollama is a separate daemon the
   user installs. Phoenix calls it over HTTP loopback, not a cloud API.
3. **Telemetry is off by default.** When enabled, only enum tags are sent —
   never user content. See `telemetry.rs`.
4. **No Tauri command exfiltrates seed data.** Reconstruct returns the
   recovered mnemonic to the same process that has the keys; the React
   frontend displays it locally. There is no Tauri-to-cloud bridge.
5. **Open-source verifies all of the above.** Anyone can grep the source for
   `reqwest::`, `tokio::net::`, `std::net::`, etc. and confirm.

If you find a code path that breaks the invariant, please open a security
disclosure per [`SECURITY.md`](../SECURITY.md).

## Cross-verification

Phoenix's cryptographic outputs are independently verified against external
references:

| Function | External reference | Cross-check script |
| :--- | :--- | :--- |
| BIP-39 mnemonic-to-seed (PBKDF2-HMAC-SHA512) | BIP-39 spec test vectors | `crates/phoenix-core/tests/crypto_test.rs::test_vector_seed_no_passphrase` |
| ETH address derivation | EIP-55 ground-truth | `crates/phoenix-core/tests/crypto_test.rs::derives_known_eth_address` |
| BTC native segwit address | Trezor docs ground-truth | `crates/phoenix-core/tests/crypto_test.rs::derives_known_btc_segwit_address` |
| Solana derivation (3 paths) | bip39 + ed25519-hd-key + tweetnacl + bs58 (Phantom's libs) | `tools/verify/solana-cross-check.js` and `solana_test::locked_vectors_match_phantom_libs` |

If any of these locked-vector tests start failing, the cryptography has
diverged from the wider ecosystem and a regression has landed.

## What is deliberately simple

Phoenix avoids these patterns even when they would be easier:

- **No async in the cryptographic hot path.** Reconstruction is CPU-bound
  blocking work; async would add overhead with no benefit.
- **No microservices.** Single binary; user runs it on their machine.
- **No background daemon.** Phoenix exits when recovery completes.
- **No persistent local database in v0.8.** Sessions are JSON files.
  SQLite migration is a v1.0+ decision, gated by real user feedback.

## Threat model

See [`docs/threat-model-v1.md`](threat-model-v1.md) for the full attack-surface
analysis. Headline mitigations:

- Stolen-wallet filter at success-fee tier (Chainalysis / TRM Labs)
- Reproducible builds (verify binary matches source)
- Reproducible cross-verification (run the JS script, get the same address)
- Open-source from Day 1
- No telemetry by default
- Branch protection on `master` (no force-push, no deletion)

## Performance characteristics

| Operation | Time (release mode, 8-core CPU) |
| :--- | :---: |
| 1-word recovery, ETH | 7–17 ms |
| 1-word recovery, BTC | 5–10 ms |
| 1-word recovery, Solana (3 paths) | 5–10 ms |
| 2-word recovery, ETH | 3–10 seconds |
| Passphrase brute force (50 candidates) | <50 ms |
| Cognitive interview round-trip (LLM) | 200 ms – 2 s (model-dependent) |
| BIP-39 mnemonic generation | <1 ms |

Detailed benchmark methodology and hardware breakdown:
[`docs/BENCHMARKS.md`](BENCHMARKS.md).
