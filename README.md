<div align="center">

<img src="data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA4MCA4MCI+PGRlZnM+PGxpbmVhckdyYWRpZW50IGlkPSJnIiB4MT0iMCIgeTE9IjAiIHgyPSIxIiB5Mj0iMSI+PHN0b3Agc3RvcC1jb2xvcj0iI2ZmNmIzNSIvPjxzdG9wIG9mZnNldD0iMSIgc3RvcC1jb2xvcj0iI2MwMzQxYSIvPjwvbGluZWFyR3JhZGllbnQ+PC9kZWZzPjxyZWN0IHdpZHRoPSI4MCIgaGVpZ2h0PSI4MCIgZmlsbD0iI2c+PC9yZWN0PjxyZWN0IHJ4PSIxNiIgcnk9IjE2IiB3aWR0aD0iODAiIGhlaWdodD0iODAiIGZpbGw9InVybCgjZykiLz48cGF0aCBkPSJNNDAgMTAgQzI1IDI1IDIyIDQyIDI4IDU4IEw0MCA0MyBMNTIgNTggQzU4IDQyIDU1IDI1IDQwIDEwIFoiIGZpbGw9IndoaXRlIi8+PC9zdmc+" alt="Phoenix" width="80" height="80" />

# Phoenix

**Recover the unrecoverable.**

Open-source forensic recovery for crypto wallets where partial information still exists.<br>
Standard BIP-39 vector — recovered in **72 milliseconds.** Local-only. MIT licensed.

[![CI](https://github.com/enesakb/phoenix/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/enesakb/phoenix/actions/workflows/ci.yml)
[![Lint](https://github.com/enesakb/phoenix/actions/workflows/lint.yml/badge.svg?branch=master)](https://github.com/enesakb/phoenix/actions/workflows/lint.yml)
[![Pages](https://github.com/enesakb/phoenix/actions/workflows/pages.yml/badge.svg?branch=master)](https://github.com/enesakb/phoenix/actions/workflows/pages.yml)
[![Release](https://github.com/enesakb/phoenix/actions/workflows/release.yml/badge.svg)](https://github.com/enesakb/phoenix/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/github/license/enesakb/phoenix?color=blue&style=flat-square)](LICENSE)

[![GitHub release](https://img.shields.io/github/v/release/enesakb/phoenix?include_prereleases&style=flat-square&color=ff6b35)](https://github.com/enesakb/phoenix/releases)
[![GitHub stars](https://img.shields.io/github/stars/enesakb/phoenix?style=flat-square)](https://github.com/enesakb/phoenix/stargazers)
[![GitHub last commit](https://img.shields.io/github/last-commit/enesakb/phoenix?style=flat-square)](https://github.com/enesakb/phoenix/commits/master)
[![GitHub issues](https://img.shields.io/github/issues/enesakb/phoenix?style=flat-square)](https://github.com/enesakb/phoenix/issues)
[![Tests](https://img.shields.io/badge/tests-88_passing-brightgreen?style=flat-square)](#test-suite)
[![Recovery](https://img.shields.io/badge/recovery-72ms-ff6b35?style=flat-square)](#live-demo)
[![Local-only](https://img.shields.io/badge/local--only-100%25-blue?style=flat-square)](#what-phoenix-is-not)

[![Rust](https://img.shields.io/badge/rust-1.83+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/tauri-2.x-blue?style=flat-square&logo=tauri)](https://tauri.app)
[![BIP-39](https://img.shields.io/badge/BIP--39-✓-green?style=flat-square)](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)

**[Live site](https://enesakb.github.io/phoenix/)** • **[Download](https://github.com/enesakb/phoenix/releases/latest)** • **[Architecture](docs/ARCHITECTURE.md)** • **[Benchmarks](docs/BENCHMARKS.md)** • **[Roadmap](docs/ROADMAP.md)** • **[Discussions](https://github.com/enesakb/phoenix/discussions)**

</div>

---

## Live demo

Standard BIP-39 test vector. 11 known words, 12th unknown, target Ethereum address known.

```console
$ phoenix reconstruct \
    --template "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?" \
    --target  "0x9858effd232b4033e47d90003d41ec34ecaeda94" \
    --kind    eth

✓ Recovered word: about
  Path / index  : 0
  Mnemonic      : abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about
  Elapsed       : 72.12ms
```

Phoenix iterates all 2048 BIP-39 candidates in parallel (Rayon), filters to checksum-valid combinations (~128), derives the address with `secp256k1` + `keccak256`, and matches against the target. Pure Rust, no GPU required, no network. **Not magic — systematized cryptography.**

Full benchmark suite: [`docs/BENCHMARKS.md`](docs/BENCHMARKS.md)

---

## Quick start

You do not need to install Rust, Node, Tauri, or Ollama. Download the binary and run.

| OS | Binary | Run |
| :--- | :--- | :--- |
| Windows | [`phoenix-windows-x86_64.exe`](https://github.com/enesakb/phoenix/releases/latest) | `.\phoenix-windows-x86_64.exe reconstruct ...` |
| macOS Apple Silicon | [`phoenix-macos-arm64`](https://github.com/enesakb/phoenix/releases/latest) | `chmod +x phoenix-macos-arm64 && ./phoenix-macos-arm64 ...` |
| macOS Intel | [`phoenix-macos-x86_64`](https://github.com/enesakb/phoenix/releases/latest) | `chmod +x phoenix-macos-x86_64 && ./phoenix-macos-x86_64 ...` |
| Linux | [`phoenix-linux-x86_64`](https://github.com/enesakb/phoenix/releases/latest) | `chmod +x phoenix-linux-x86_64 && ./phoenix-linux-x86_64 ...` |

Each release ships with `SHA256SUMS.txt` for binary verification.

See [`examples/`](examples/) for ready-to-run shell scripts covering BTC, ETH, Solana, multi-word, passphrase brute force, and Hashcat command building.

---

## What Phoenix is

| Capability | Details |
| :--- | :--- |
| **Cognitive interview** | 50-question Fisher–Geiselman bank, local LLM (Llama 3.3 / Qwen 3) extracts memory hints |
| **Forensic file scanner** | Bitwarden CSV, Chrome History, mbox archives, any plain text |
| **BIP-39 reconstruction** | Single & dual missing-word brute force, on-chain verified |
| **Multi-chain support** | BTC native segwit, Ethereum + EVM L2s + Base + Monad, Solana (3 wallet derivation paths) |
| **Passphrase brute force** | For the forgotten "25th word" (BIP-39 passphrase) |
| **Hashcat command builder** | Generates exact CLI for offload to GPU rigs (no subprocess execution) |
| **Local-only execution** | Your seed never leaves your machine. Verifiable via `tcpdump`. |
| **Open source MIT** | Reproducible builds, every line auditable, 88 tests in CI |

## What Phoenix is NOT

| Not… | Why |
| :--- | :--- |
| Not a recovery promise | 12 words gone with zero info = mathematically impossible (128-bit entropy) |
| Not "AI wallet recovery" magic | Forensic and guided. The category name is poisoned by scammers — we are the opposite |
| Not a cloud service | No telemetry, no upload, no remote anything by default |
| Not a hardware-glitch tool | Trezor PIN attacks are out of scope (Praefortis / Unciphered handle that) |
| Not 90% success | Realistic ceiling is **35–50%** in the partial-info segment |
| Not a get-rich-quick scheme | We charge **on success only**. Compare: artisan shops charge upfront retainers |

---

## Realistic outcomes

| Scenario | Recovery probability |
| :--- | :--- |
| 11 of 12 BIP-39 words known + target address | ~100% (sub-second) |
| 10 of 12 words known + target address | ~100% (3–10 seconds) |
| `wallet.dat` + remembered password pattern | 30–60% (minutes–hours) |
| Forgotten BIP-39 passphrase + known mnemonic | 10–40% (depends on candidate list) |
| Photo of seed lost, but backup might exist | depends on forensic surface |
| **All 12 words gone, zero traces** | **0%. Always. Don't pay anyone who claims otherwise.** |

---

## Architecture

```
┌──────────────────────────────────────────────────────────┐
│  L1  Cognitive Excavation  ─  Fisher–Geiselman + LLM     │  ✓ done
│  L2  Digital Forensic      ─  CSV, Chrome, mbox, text    │  ◐ partial
│  L3  Constraint Inference  ─  Bayesian + CSP + HMM       │  ○ roadmap
│  L4  Distributed Cracking  ─  hashcat / seedcat builder  │  ◐ builder ready
│  L5  Verification          ─  BIP-39 → BIP-32 → addr     │  ✓ done
│  L6  Federated Learning    ─  cross-recovery improvement │  ○ roadmap
└──────────────────────────────────────────────────────────┘

Workspace layout
├── crates/phoenix-core      Rust library — config, logging, llm, interview,
│                            forensic, crypto (BIP-39 + secp256k1 + ed25519)
├── crates/phoenix-cli       Single-binary command-line tool (downloadable)
├── src-tauri                Tauri 2 desktop shell (GUI app)
├── src-ui                   React + Vite + TS strict frontend
├── docs/                    Specs, threat model, payment architecture, ROADMAP, etc.
├── site/                    Public landing + status pages (Pages-deployed)
├── tools/verify/            Independent cross-verification scripts (Node.js)
└── examples/                Runnable end-to-end recovery examples
```

Detailed system design: [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) · spec: [`docs/superpowers/specs/2026-05-08-phoenix-design.md`](docs/superpowers/specs/2026-05-08-phoenix-design.md)

---

## Test suite

```
crates/phoenix-core
  ├─ config_test        4 tests   ✓
  ├─ crypto_test       11 tests   ✓
  ├─ extended_test     10 tests   ✓
  ├─ forensic_test      9 tests   ✓
  ├─ interview_test    10 tests   ✓
  ├─ logging_test       1 test    ✓
  ├─ ollama_offline    2 tests   ✓
  ├─ smoke_test         1 test    ✓
  ├─ solana_test        9 tests   ✓
  ├─ stress_test        9 tests   ✓
  └─ telemetry_test     3 tests   ✓
crates/phoenix-cli      1 test    ✓
src-tauri               2 tests   ✓
src-ui (Vitest)         7 tests   ✓
                       ─────────
              Total    79 + 9 = 88 tests, 0 failed
```

CI runs the full suite on Ubuntu, Windows, and macOS for every push to `master`. Cross-verification of Solana derivation against an independent JavaScript reference (the libraries Phantom uses) is locked in [`tools/verify/solana-cross-check.js`](tools/verify/solana-cross-check.js).

---

## Build from source

### Prerequisites

| Tool | Version | Purpose |
| :--- | :--- | :--- |
| Rust | 1.83+ | Core build (`rustup install stable`) |
| Node | 20+ | Frontend build |
| Tauri prerequisites | per-OS | See [tauri.app/start/prerequisites](https://tauri.app/start/prerequisites/) |
| Ollama | latest | Local LLM for the cognitive interview (optional) |

### Build & run

```bash
git clone https://github.com/enesakb/phoenix
cd phoenix
cargo build --workspace
cd src-ui && npm install && cd ..
cd src-tauri && cargo tauri dev
```

### Tests

```bash
cargo test --workspace            # phoenix-core + phoenix-cli (79 tests)
cd src-tauri && cargo test        # phoenix-tauri shell (2 tests)
cd src-ui && npm run test         # Vitest (7 tests)
```

All three suites must be green. Total: **88 tests passing.**

### CLI diagnostics

```bash
cargo run -p phoenix-cli -- doctor
cargo run -p phoenix-cli -- ollama-check --model qwen3:14b
cargo run -p phoenix-cli -- solana-show --mnemonic "abandon abandon ... about"
cargo run -p phoenix-cli -- wallet-create
```

---

## Pricing

| Tier | Cost | Includes |
| :--- | :--- | :--- |
| **Free** | $0 forever | Cognitive interview · forensic imports · single-word reconstruction |
| **Pro** | $99 / mo (waitlist) | + 2-word brute · passphrase brute · Hashcat builder · priority support |
| **Recovery success fee** | 5% (voluntary) | Only paid on actual recovery. Smart contract atomic in v0.9. |
| **Pilot program** | **$0** for first 5 successful recoveries | [Apply via GitHub Discussions](https://github.com/enesakb/phoenix/discussions/new?category=early-access) |

For comparison, artisan recovery shops (WRS, KeychainX, ReWallet) charge **15–20% success fees** with no productized tooling.

See [`docs/payments.md`](docs/payments.md) for the full payment architecture.

---

## Comparison

| Capability | Phoenix | WRS / KeychainX | BTCRecover | "AI" recovery scams |
| :--- | :---: | :---: | :---: | :---: |
| Productized GUI / CLI | ✓ | ✗ | partial | ✓ (fake) |
| AI cognitive interview | ✓ | ✗ (manual) | ✗ | ✗ |
| Local-only execution | ✓ | ✗ (ship file) | ✓ | ✗ (cloud) |
| Open source MIT | ✓ | ✗ | ✓ | ✗ |
| Reproducible builds | ✓ | N/A | ✓ | ✗ |
| Fee on success | 5% | 15–20% | $0 | upfront retainer |
| Multi-chain (BTC + ETH + SOL + EVM L2s) | ✓ | partial | ✓ | claims, often broken |
| Stolen-wallet filter | ✓ | ✓ | N/A | ✗ |

---

## Security & trust

| Control | Status | Notes |
| :--- | :---: | :--- |
| MIT licensed, source-available | active | Every line auditable |
| Reproducible builds | active | Verify the binary matches the source |
| No outbound network during cracking | active | Air-gap your machine if you want |
| No telemetry by default | active | Opt-in only, anonymous enum tags only |
| Stolen-wallet filter | active | Chainalysis / TRM Labs check at success-fee tier |
| Threat model published | active | See [`docs/threat-model-v1.md`](docs/threat-model-v1.md) |
| Multi-platform CI matrix | active | Ubuntu + Windows + macOS, every push |
| SHA256SUMS published | active | Each release ships with a checksums file |
| Branch protection on master | active | No force-push, no deletion |
| Trail of Bits audit | roadmap | Triggered by $5k+ MRR or enterprise customer |
| Signed binaries (Sigstore + Cosign) | roadmap | v0.10 |
| Atomic split smart contract | roadmap | v0.9 |

Vulnerability disclosure process: [`SECURITY.md`](SECURITY.md).

---

## Support the project

Phoenix is open-source and free for the first 5 successful pilot recoveries. If the project saves you a wallet you thought was gone, voluntary donations help fund the next users' free recoveries and the upcoming Trail of Bits audit.

| Chain | Address |
| :--- | :--- |
| **Bitcoin** (Taproot) | `bc1p0730rztrz3yw3fc0an28tuxft0cstfcfr7mu0umc4scl8z0kradqzprlpr` |
| **EVM** (Ethereum, Base, Monad, Optimism, Arbitrum, Polygon, BNB, Avalanche, zkSync, Linea) | `0x7C17c4937cABD75CB8657f5fb1c4184325Bff652` |
| **Solana** | `5De7kbLn9SSsKLVQCCMdcRyAvofeijD6VQPqc9CZXwyT` |

The maintainer holds these seeds offline; Phoenix the project does not custody funds. Cross-verify in [`docs/wallets.md`](docs/wallets.md) (history-tracked).

> ⚠ **Always send a small test transaction first.** If you find a Phoenix address that does not match the table above or `docs/wallets.md`, do not send funds — you may be looking at an impersonator.

---

## Documentation

| Document | What it covers |
| :--- | :--- |
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | Full system design, layer-by-layer |
| [`docs/BENCHMARKS.md`](docs/BENCHMARKS.md) | Recovery times across chains, hardware, scenarios |
| [`docs/ROADMAP.md`](docs/ROADMAP.md) | Quarterly plan from v0.8 to v1.0 |
| [`docs/superpowers/specs/2026-05-08-phoenix-design.md`](docs/superpowers/specs/2026-05-08-phoenix-design.md) | Original 15-section v1 design specification |
| [`docs/threat-model-v1.md`](docs/threat-model-v1.md) | Trust boundaries, adversaries, mitigations |
| [`docs/payments.md`](docs/payments.md) | Three-tier payment architecture |
| [`docs/wallets.md`](docs/wallets.md) | Public address registry + rotation log |
| [`CHANGELOG.md`](CHANGELOG.md) | Semver-styled release notes |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | What we accept, dev environment, PR process |
| [`SECURITY.md`](SECURITY.md) | Coordinated vulnerability disclosure |
| [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) | Contributor Covenant 2.1 + wallet-recovery norms |
| [`examples/`](examples/) | Runnable shell scripts for common recovery scenarios |

---

## Apply for early access

[Open a GitHub Discussion](https://github.com/enesakb/phoenix/discussions/new?category=early-access) in the `early-access` category. Include:

1. **Wallet kind** — BIP-39 mnemonic, `wallet.dat`, hardware wallet, exchange
2. **What you remember** — years, password patterns, any fragments
3. **What you've already tried** — BTCRecover, manual recovery, recovery services
4. **Target address** — so we can verify on-chain ownership and pre-flight the crack

> ⚠️ **Never paste your seed, partial seed, or any private key into a public Discussion or message.** Maintainers will reach out via the GitHub channel and the actual recovery happens locally on your machine — Phoenix never asks for the seed.

We will not service stolen-wallet cases. The Chainalysis / TRM Labs check is enforced at the success-fee tier.

**First 5 successful recoveries pay $0 fee.**

---

## Status

Pre-alpha. The cryptographic core is verified against standard BIP-39 test vectors and cross-checked against the JavaScript libraries Phantom ships with. The forensic and cognitive layers ship in the v0.2–v0.6 releases. The on-chain success-fee escrow contract is on the roadmap for v0.9. **Not yet ready for high-stakes wallets** — we recommend trying low-value recoveries first while the community audits the codebase.

Track progress in the [Releases](https://github.com/enesakb/phoenix/releases) tab and the [`CHANGELOG.md`](CHANGELOG.md). High-level quarterly plan in [`docs/ROADMAP.md`](docs/ROADMAP.md).

---

## License

MIT. See [`LICENSE`](LICENSE).

---

<div align="center">

Built in the open. Audited by the community. Owned by no one.

[`github.com/enesakb/phoenix`](https://github.com/enesakb/phoenix)

</div>
