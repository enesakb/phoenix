# Phoenix

> **Recover the unrecoverable.** Open-source forensic recovery for crypto wallets where partial information still exists.

[![Status](https://img.shields.io/badge/status-pre--alpha-orange)](https://github.com/enesakb/phoenix)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-60_passing-brightgreen)](https://github.com/enesakb/phoenix)
[![Recovery](https://img.shields.io/badge/BIP--39_recovery-72ms-ff6b35)](#live-demo)
[![Local-only](https://img.shields.io/badge/local--only-100%25-blue)](#what-phoenix-is-not)

---

## Live demo — recovery in 72 milliseconds

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

This is the **standard BIP-39 test vector**. 11 known words, 12th unknown, target Ethereum address known. Phoenix tries all 2048 candidate words in parallel, filters checksum-valid combinations, derives the address with `secp256k1` + `keccak256`, matches against the target. **Not magic — just systematized cryptography.**

---

## What Phoenix is

- 🧠 **Cognitive interview** — 50-question Fisher-Geiselman bank, local LLM (Llama 3.3 / Qwen 3) extracts memory hints
- 📁 **Forensic file scanner** — Bitwarden CSV, Chrome History, mbox email archives, any text file
- 🔐 **BIP-39 reconstruction** — single & dual missing-word brute force, onchain-verified
- 🔑 **Passphrase brute force** — for the forgotten "25th word" (BIP-39 passphrase)
- ⚡ **Hashcat command builder** — for heavy lift offload to GPU rigs
- 🏠 **100% local** — your seed never leaves your machine
- 🔓 **Open source MIT** — reproducible builds, auditable

## What Phoenix is NOT

- ❌ **Not a recovery promise.** 12 words gone with zero info = mathematically impossible (128-bit entropy). Period.
- ❌ **Not "AI wallet recovery" magic.** Forensic and guided. The category name is poisoned by scammers — we're the opposite.
- ❌ **Not a cloud service.** No telemetry, no upload, no remote anything by default.
- ❌ **Not a hardware-glitch tool.** Trezor PIN attacks are out of scope (Praefortis / Unciphered handle that).
- ❌ **Not 90% success.** Realistic ceiling is **35–50%** in the partial-info segment.

## Realistic outcomes

| Scenario | Recovery probability |
|---|---|
| 11 of 12 BIP-39 words known + target address | ~100% (sub-second) |
| 10 of 12 words known + target address | ~100% (3-10 seconds) |
| Wallet.dat + remembered password pattern | 30-60% (minutes-hours) |
| Forgotten BIP-39 passphrase + known mnemonic | 10-40% (depends on candidate list) |
| Photo of seed lost, but backup might exist | depends on forensic surface |
| **All 12 words gone, zero traces** | **0%. Always. Don't pay anyone who claims otherwise.** |

---

## Architecture (six layers)

```
┌─ L1: Cognitive Excavation     (Fisher-Geiselman + LLM extractor)        ✓ done
├─ L2: Digital Forensic         (CSV, Chrome history, mbox, text)         ✓ partial
├─ L3: Constraint Inference     (Bayesian + CSP + HMM)                    ⏳ roadmap
├─ L4: Distributed Cracking     (hashcat / seedcat orchestration)         ✓ builder
├─ L5: Verification + Restore   (BIP-39 → BIP-32 → secp256k1 → addr)      ✓ done
└─ L6: Federated Learning       (cross-recovery model improvement)         ⏳ roadmap
```

---

## Build from source

### Prerequisites

- Rust 1.83+ (`rustup install stable`)
- Node 20+
- Tauri prerequisites for your OS — see https://tauri.app/start/prerequisites/
- Ollama with `qwen3:14b` or `llama3.3:70b` pulled locally (only needed for cognitive interview)

### Build

```bash
git clone https://github.com/enesakb/phoenix
cd phoenix
cargo build --workspace
cd src-ui && npm install && cd ..
cd src-tauri && cargo tauri dev
```

### Run tests (60 tests across the workspace)

```bash
cargo test --workspace
cd src-tauri && cargo test
cd src-ui && npm run test
```

### CLI diagnostics

```bash
cargo run -p phoenix-cli -- doctor
cargo run -p phoenix-cli -- ollama-check --model qwen3:14b
cargo run -p phoenix-cli -- reconstruct --template "..." --target "0x..." --kind eth
```

---

## Pricing

| Tier | Cost | Includes |
|---|---|---|
| **Free** | $0 | Interview, forensic imports, single-word reconstruction |
| **Pro** | $99/month | + 2-word brute, passphrase brute, Hashcat builder |
| **Recovery fee** | 5% of recovered | Only paid on success. Atomic via smart contract. |
| **Pilot** | **$0** for first 5 successful recoveries | Email `noreply@phoenix.local` |

Compare: artisan recovery shops (WRS, KeychainX, ReWallet) charge **15-20% success fees** with no productized tooling.

---

## Security & trust

- 🔓 **MIT licensed**, source-available — every line auditable
- 🏗️ **Reproducible builds** — verify the binary matches the source
- 🔍 **Trail of Bits audit** — on the roadmap once revenue funds it
- 🚫 **No telemetry by default** — opt-in only, anonymous events only
- ❄️ **No network during cracking** — air-gap your machine if you want
- 🛡️ **Threat model** — see [`docs/threat-model-v1.md`](docs/threat-model-v1.md)
- 🚨 **Stolen-wallet filter** — Chainalysis / TRM Labs check at success-fee tier

---

## Documentation

- **Design spec** — [`docs/superpowers/specs/2026-05-08-phoenix-design.md`](docs/superpowers/specs/2026-05-08-phoenix-design.md)
- **Threat model v1** — [`docs/threat-model-v1.md`](docs/threat-model-v1.md)
- **Sprint plans** — [`docs/superpowers/plans/`](docs/superpowers/plans/)
- **Internal status site** — open `site/status.html` in a browser
- **Public landing** — open `site/index.html` in a browser

---

## Apply for early access

Email **noreply@phoenix.local** with a one-paragraph description of your case:

- What kind of wallet (BIP-39 mnemonic, wallet.dat, hardware wallet, exchange)
- What you remember
- What you've already tried
- The target address (so we can verify on-chain)

No scammers. We will not service stolen-wallet cases. **First 5 successful recoveries pay $0 fee.**

---

## License

MIT. See [`LICENSE`](LICENSE).
