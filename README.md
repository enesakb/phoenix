<div align="center">

# Phoenix

**Open-source forensic recovery for crypto wallets where partial information still exists.**

Standard BIP-39 vector — recovered in **72 milliseconds**.<br>
Local-only. Verifiable. MIT licensed.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)
[![Status](https://img.shields.io/badge/status-pre--alpha-orange?style=flat-square)](#status)
[![Tests](https://img.shields.io/badge/tests-60_passing-brightgreen?style=flat-square)](#tests)
[![Recovery](https://img.shields.io/badge/recovery-72ms-ff6b35?style=flat-square)](#live-demo)
[![Local-only](https://img.shields.io/badge/local--only-100%25-blue?style=flat-square)](#what-phoenix-is)
[![Rust](https://img.shields.io/badge/rust-1.83+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/tauri-2.x-blue?style=flat-square&logo=tauri)](https://tauri.app)

[Live demo](#live-demo) · [How it works](#how-it-works) · [Architecture](#architecture) · [Pricing](#pricing) · [Security](#security--trust) · [FAQ](#faq)

</div>

---

## Live demo

Standard BIP-39 test vector. 11 known words, 12th unknown, target Ethereum address known.

```bash
$ phoenix reconstruct \
    --template "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?" \
    --target  "0x9858effd232b4033e47d90003d41ec34ecaeda94" \
    --kind    eth

✓ Recovered word: about
  Address index : 0
  Mnemonic      : abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about
  Elapsed       : 72.12ms
```

Phoenix iterates all 2048 BIP-39 candidates in parallel, filters checksum-valid combinations, derives the address with `secp256k1` + `keccak256`, matches against the target. Pure Rust, no GPU required, no network. **Not magic — systematized cryptography.**

---

## What Phoenix is

| Capability | Details |
| :--- | :--- |
| **Cognitive interview** | 50-question Fisher–Geiselman bank, local LLM (Llama 3.3 / Qwen 3) extracts memory hints |
| **Forensic file scanner** | Bitwarden CSV, Chrome History, mbox archives, any plain text |
| **BIP-39 reconstruction** | Single & dual missing-word brute force, on-chain verified |
| **Multi-chain support** | BTC native segwit, Ethereum, **Solana** (Phantom + Solflare + Sollet paths), all cryptographically cross-verified against reference implementations |
| **Passphrase brute force** | For the forgotten "25th word" (BIP-39 passphrase) |
| **Hashcat command builder** | Generates exact CLI for offload to GPU rigs (no subprocess execution) |
| **Local-only execution** | Your seed never leaves your machine. Verifiable via tcpdump. |
| **Open source MIT** | Reproducible builds, every line auditable |

## What Phoenix is NOT

| Not… | Why |
| :--- | :--- |
| Not a recovery promise | 12 words gone with zero info = mathematically impossible (128-bit entropy) |
| Not "AI wallet recovery" magic | Forensic and guided. The category name is poisoned by scammers — we are the opposite |
| Not a cloud service | No telemetry, no upload, no remote anything by default |
| Not a hardware-glitch tool | Trezor PIN attacks are out of scope (Praefortis / Unciphered handle that) |
| Not 90% success | Realistic ceiling is **35–50%** in the partial-info segment |
| Not a get-rich-quick scheme | We charge **on success only**. Compare: artisan shops charge upfront retainers |

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
L1  Cognitive Excavation     Fisher–Geiselman + LLM extractor               ✓ done
L2  Digital Forensic         CSV, Chrome history, mbox, text                ◐ partial
L3  Constraint Inference     Bayesian + CSP + HMM                           ○ roadmap
L4  Distributed Cracking     hashcat / seedcat orchestration                ◐ builder
L5  Verification + Restore   BIP-39 → BIP-32 → secp256k1 → addr             ✓ done
L6  Federated Learning       Cross-recovery model improvement (privacy)      ○ roadmap
```

Detailed design: [`docs/superpowers/specs/2026-05-08-phoenix-design.md`](docs/superpowers/specs/2026-05-08-phoenix-design.md)

---

## Quick start — for users with a lost wallet

You do not need to install Rust, Node, or anything else. Download the binary for your operating system from the [Releases page](https://github.com/enesakb/phoenix/releases/latest), make it executable, and run it.

### Windows

1. Download `phoenix-windows-x86_64.exe` from the [latest release](https://github.com/enesakb/phoenix/releases/latest).
2. Open a PowerShell window in the folder where the file is saved.
3. Run:
   ```powershell
   .\phoenix-windows-x86_64.exe reconstruct `
     --template "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?" `
     --target "0x9858effd232b4033e47d90003d41ec34ecaeda94" `
     --kind eth
   ```
4. Replace the template with your own 12 words (use `?` for the unknown one) and the target with your wallet address.

### macOS

1. Download `phoenix-macos-arm64` (Apple Silicon) or `phoenix-macos-x86_64` (Intel).
2. Open Terminal in the download folder.
3. Make it executable and run:
   ```bash
   chmod +x phoenix-macos-arm64
   ./phoenix-macos-arm64 reconstruct \
     --template "abandon abandon ... abandon ?" \
     --target "0x9858eff..." \
     --kind eth
   ```
4. macOS may show a Gatekeeper warning the first time — go to System Settings → Privacy & Security → "Allow Anyway".

### Linux

```bash
curl -L https://github.com/enesakb/phoenix/releases/latest/download/phoenix-linux-x86_64 -o phoenix
chmod +x phoenix
./phoenix reconstruct \
  --template "abandon abandon ... abandon ?" \
  --target "0x9858eff..." \
  --kind eth
```

### What `--kind` to pick

| If your wallet is on… | `--kind` value |
| :--- | :--- |
| Bitcoin (most modern wallets) | `btc` |
| Ethereum, Base, Optimism, Arbitrum, Polygon, BNB | `eth` |
| Solana (Phantom, Solflare, Backpack) | `sol` |

### Verifying the binary you downloaded

Each release includes a `SHA256SUMS.txt` file. After downloading the binary, run:

```bash
sha256sum -c SHA256SUMS.txt
```

(or `Get-FileHash` on Windows). If the checksum does not match, do not run the binary — you may have downloaded an impersonator's tampered file.

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
cargo test --workspace            # phoenix-core + phoenix-cli (51 tests)
cd src-tauri && cargo test        # phoenix-tauri shell (2 tests)
cd src-ui && npm run test         # Vitest (7 tests)
```

All three suites must be green. Total: **60 tests passing.**

### CLI diagnostics

```bash
cargo run -p phoenix-cli -- doctor
cargo run -p phoenix-cli -- ollama-check --model qwen3:14b
cargo run -p phoenix-cli -- reconstruct \
    --template "..." --target "0x..." --kind eth
```

---

## Pricing

| Tier | Cost | Includes |
| :--- | :--- | :--- |
| **Free** | $0 forever | Cognitive interview · forensic imports · single-word reconstruction |
| **Pro** | $99 / month (30-day trial) | + 2-word brute · passphrase brute · Hashcat builder · priority support |
| **Recovery fee** | 5% of recovered | Only paid on actual recovery. Atomic via smart contract. |
| **Pilot** | **$0** for the first 5 successful recoveries | [Open a GitHub Discussion](https://github.com/enesakb/phoenix/discussions/new?category=early-access) |

For comparison, artisan recovery shops (WRS, KeychainX, ReWallet) charge **15–20% success fees** with no productized tooling.

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
| Trail of Bits audit | roadmap | Triggered by $5k+ MRR or enterprise customer |
| Signed binaries | roadmap | Sigstore + Cosign in v0.7 |

Vulnerability disclosure process: [`SECURITY.md`](SECURITY.md).

---

## Documentation

| Document | What it covers |
| :--- | :--- |
| [`docs/superpowers/specs/2026-05-08-phoenix-design.md`](docs/superpowers/specs/2026-05-08-phoenix-design.md) | Full v1 design specification (15 sections) |
| [`docs/threat-model-v1.md`](docs/threat-model-v1.md) | Trust boundaries, adversaries, mitigations |
| [`docs/superpowers/plans/`](docs/superpowers/plans/) | Sprint-by-sprint implementation plans |
| [`CHANGELOG.md`](CHANGELOG.md) | Semver-styled release notes |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | What we accept, dev environment, PR process |
| [`SECURITY.md`](SECURITY.md) | Coordinated vulnerability disclosure |
| [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) | Contributor Covenant 2.1 |
| `site/index.html` | Public landing page (also published via Cloudflare Pages) |
| `site/status.html` | Internal operator dashboard |

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

Pre-alpha. The cryptographic core is verified against standard BIP-39 test vectors. The forensic and cognitive layers ship in the v0.2–v0.6 releases. The on-chain success-fee escrow contract is on the roadmap for v0.7. **Not yet ready for high-stakes wallets** — we recommend trying low-value recoveries first while the community audits the codebase.

Track progress in the [Releases](https://github.com/enesakb/phoenix/releases) tab and the [`CHANGELOG.md`](CHANGELOG.md).

---

## Support the project

Phoenix is open-source and free for the first 5 successful pilot recoveries. If the project saves you a wallet you thought was gone, voluntary donations help fund the next users' free recoveries and the upcoming Trail of Bits audit.

| Chain | Address |
| :--- | :--- |
| **Bitcoin** (Taproot) | `bc1p0730rztrz3yw3fc0an28tuxft0cstfcfr7mu0umc4scl8z0kradqzprlpr` |
| **EVM** (Ethereum, Base, Monad, Optimism, Arbitrum, Polygon, BNB, Avalanche, zkSync, Linea) | `0x7C17c4937cABD75CB8657f5fb1c4184325Bff652` |
| **Solana** | `5De7kbLn9SSsKLVQCCMdcRyAvofeijD6VQPqc9CZXwyT` |

The maintainer holds these seeds offline; Phoenix the project does not custody funds. Cross-verify in [`docs/wallets.md`](docs/wallets.md) (history-tracked). See [`docs/payments.md`](docs/payments.md) for the full payment architecture and the v0.8 atomic-split contract spec.

> ⚠ **Always send a small test transaction first.** If you find a Phoenix address that does not match the table above or `docs/wallets.md`, do not send funds — you may be looking at an impersonator.

---

## License

MIT. See [`LICENSE`](LICENSE).
