# Changelog

All notable changes to Phoenix are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.0] – 2026-05-09 — `week-7-solana`

### Added
- `crypto::solana` module — ed25519 + SLIP-10 derivation across the three canonical Solana wallet paths (Phantom/Backpack/Trust at `m/44'/501'/0'/0'`, Solflare at `m/44'/501'/0'`, Sollet legacy at `m/501'/0'/0'/0'`)
- `AddressKind::Solana` variant in the reconstruction pipeline; `match_seed_to_target` helper unifies BTC/ETH/Solana matching
- `phoenix solana-show` CLI subcommand prints the three candidate addresses for any mnemonic so users can compare against their installed wallet
- `tools/verify/solana-cross-check.js` — independent Node.js cross-verification using the exact libraries Phantom ships with (bip39 + ed25519-hd-key + tweetnacl + bs58)
- Locked-vector regression test (`solana_test::locked_vectors_match_phantom_libs`) asserting the exact addresses produced by the BIP-39 zero vector
- Public landing page with conversion-optimized layout (`site/index.html`)
- Internal status dashboard (`site/status.html`)
- Launch playbook (`site/launch/`) covering Reddit, Twitter, Hacker News post drafts and Cloudflare Pages deployment guide
- Professional repository boilerplate: `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, issue/PR templates, Dependabot config

### Verified
- Solana derivation matches the Phantom JavaScript pipeline byte-identical across all three paths for the BIP-39 zero vector. Cross-verified 2026-05-09.

### Test count
- **78 tests** total: 69 Rust workspace (51 prior + 9 Solana + 9 extended) + 2 Tauri + 7 Vitest. All green.

## [0.6.0] – 2026-05-09 — `week-4-6-extended`

### Added
- `crypto::reconstruct::reconstruct_multi` — supports 1 or 2 missing BIP-39 words. Two-missing case runs Rayon-parallel cartesian product (4.2M raw × checksum filter); converges in 6–10 seconds on 8-core CPU. 3+ missing returns `TooManyMissing` rather than running unbounded.
- `crypto::reconstruct::brute_force_passphrase` — given full mnemonic + target address + candidate passphrase list, finds the forgotten 25th word.
- `crypto::hashcat` — command builder for offload to GPU rigs. Supports modes 11300 (BTC wallet.dat), 12700 (Blockchain.com), 22500 (MultiBit), 21700/21800 (Electrum), 15700 (MEW V2), 27800 (BIP-38). No subprocess execution — Phoenix only emits the string.
- `forensic::mbox::MboxExtractor` — RFC-822 archive scanner (Gmail Takeout, IMAP). Lifts Subject lines for crypto keyword leads, runs BIP-39 sequence detector on every body.
- `docs/threat-model-v1.md` — supersedes v0; covers all new attack surfaces.

## [0.5.0] – 2026-05-09 — `week-5-crypto-reconstruct`

### Added
- `crypto::mnemonic` — bip39 wrapper, validation, mnemonic-to-seed.
- `crypto::derive` — bip32 hierarchical derivation; standard BIP-44 paths (m/84'/0'/0'/0/i for BTC native segwit, m/44'/60'/0'/0/i for ETH).
- `crypto::address` — secp256k1 → BTC P2WPKH (bech32 segwit v0) and ETH (keccak256 last-20-bytes).
- `crypto::reconstruct::reconstruct_missing_word` — Rayon-parallel single-missing-word brute force; checksum-valid candidate filter; address verification.
- `phoenix-cli reconstruct` subcommand.
- Standard BIP-39 test vector verification: 'abandon × 11 about' → seed matches `5eb00bbddcf069...e9e38e4`; ETH m/44'/60'/0'/0/0 → `0x9858effd232b4033e47d90003d41ec34ecaeda94`; recovery in **72 milliseconds**.

## [0.4.0] – 2026-05-09 — `week-4-6-extended` (intermediate)

Documented under the same release tag as 0.6.0; see above.

## [0.3.0] – 2026-05-09 — `week-3-forensic-a`

### Added
- `forensic::registry::ExtractorRegistry` — dispatcher mapping file extensions to extractors.
- `forensic::bip39_text::Bip39TextExtractor` — universal text scanner; finds 3+ consecutive BIP-39 word sequences with confidence scoring.
- `forensic::bitwarden_csv::BitwardenCsvExtractor` — CSV parser for Bitwarden exports.
- `forensic::chrome_history::ChromeHistoryExtractor` — SQLite reader for Chrome/Edge/Brave History DB copies; pulls crypto-domain visits.
- Tauri `import_file` command and dialog plugin integration.
- `/import/:sessionId` UI route with native file picker.

## [0.2.0] – 2026-05-08 — `week-2-cognitive-interview`

### Added
- `interview::questions::QuestionBank` — 50-question Fisher–Geiselman-style bank, embedded JSON.
- `interview::state::MemoryState` — graph of typed memory nodes with confidence scoring.
- `interview::session::SessionStore` — JSON-on-disk persistence (atomic write).
- `interview::interviewer::Interviewer` — LLM-driven memory-hint extractor with strict JSON parsing.
- `interview::debate::cross_question` — three-perspective LLM cross-questioning loop.
- `interview::candidate::extract` — pure scoring + sorting from MemoryState.
- Tauri commands `list_questions`, `start_interview`, `answer_question`, `get_candidates`, `get_memory`, `complete_interview`.
- `/interview` and `/candidates/:sessionId` UI routes.

## [0.1.0] – 2026-05-08 — `week-1-foundation`

### Added
- Cargo workspace with `phoenix-core`, `phoenix-cli`, `phoenix-tauri` crates.
- React 18 + Vite + TypeScript strict frontend in `src-ui/`.
- Tauri 2 desktop shell with `app_info` IPC command.
- `phoenix-core::config` (TOML), `logging` (tracing), `telemetry` (opt-in).
- `phoenix-core::llm::OllamaClient` with `LlmClient` async trait, validated with wiremock.
- `phoenix-cli` with `doctor` and `ollama-check` subcommands.
- GitHub Actions CI (Linux/Windows/macOS Rust matrix + clippy + eslint).
- Initial threat model (`docs/threat-model-v0.md`).
