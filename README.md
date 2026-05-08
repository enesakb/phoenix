# Phoenix

> Open-source forensic recovery assistant for partial-information lost crypto wallets.

**Status:** Pre-alpha (Week 1 foundation complete, 2026-05-08).

## What Phoenix is

Phoenix systematizes what artisan wallet-recovery shops do manually: a structured cognitive interview, deep digital exhaust forensics, Bayesian candidate ranking, and distributed cracking — orchestrated as a single open-source desktop application.

## What Phoenix is NOT

- Not a recovery promise. Wallets with zero memory and zero digital traces cannot be recovered. Period.
- Not a hardware-glitch tool. Hardware PIN attacks are out of scope (Praefortis / Unciphered own that space).
- Not a cloud service. All recovery work happens locally on your machine.
- Not "AI wallet recovery." Phoenix is forensic and guided, not magic.

## Realistic outcomes

For wallets where partial information exists (forgotten 1-2 seed words, wallet.dat with remembered password pattern, lost backups with traceable digital exhaust): expected v1 recovery rate **35-50%**.

For wallets with no recoverable signal: **0%**. Always.

## Repository layout

```
phoenix/
├── crates/phoenix-core      # Domain logic (config, logging, telemetry, llm)
├── crates/phoenix-cli       # Developer ergonomics (`phoenix doctor`, `phoenix ollama-check`)
├── src-tauri                # Tauri 2 desktop shell + IPC commands
├── src-ui                   # React + Vite frontend
├── docs/superpowers         # Spec + plan documents
└── docs/threat-model-v0.md  # Initial threat model
```

## Building from source

### Prerequisites

- Rust 1.83+ (`rustup install stable`)
- Node 20+
- Tauri prerequisites for your OS — see https://tauri.app/start/prerequisites/
- Ollama, with `llama3.3:70b` pulled locally (only required for `ollama-check` and Layer 1 work):
  ```bash
  ollama pull llama3.3:70b
  ollama serve
  ```

### Build

```bash
git clone <this-repo>
cd phoenix
cargo build --workspace
cd src-ui && npm install && cd ..
cd src-tauri && cargo tauri dev   # launch the desktop app
```

### Run tests

```bash
cargo test --workspace            # phoenix-core + phoenix-cli (12 tests)
cd src-tauri && cargo test        # phoenix-tauri (1 test)
cd src-ui && npm run test         # vitest (1 test)
```

### Diagnostics

```bash
cargo run -p phoenix-cli -- doctor
cargo run -p phoenix-cli -- ollama-check     # requires running Ollama
```

## Design

The full design specification is at [`docs/superpowers/specs/2026-05-08-phoenix-design.md`](docs/superpowers/specs/2026-05-08-phoenix-design.md).

The Week 1 implementation plan is at [`docs/superpowers/plans/2026-05-08-phoenix-week1-foundation.md`](docs/superpowers/plans/2026-05-08-phoenix-week1-foundation.md).

The initial threat model is at [`docs/threat-model-v0.md`](docs/threat-model-v0.md).

## Roadmap

- **Week 1 (DONE 2026-05-08)** — Foundation: workspace, Tauri shell, React UI, Ollama client, CI.
- **Week 2** — Layer 1: Cognitive Interview MVP (50 structured questions, RL policy, multi-agent debate).
- **Week 3** — Layer 2 part A: Browser forensics, password manager dump parser, photo OCR.
- **Week 4** — Layer 2 part B: Email backup mining, iCloud/Drive parsers, file carving + threat model v1.
- **Week 5** — Layer 3+4: Bayesian candidate ranker, hashcat/seedcat wrappers, custom CUDA for Phantom.
- **Week 6** — Validation: 10 paying pilots, daily user calls.
- **Week 7** — First successful recovery + anonymous case study.
- **Week 8** — Trail of Bits engagement, public GitHub launch, ProductHunt + HN.

## License

MIT. See [`LICENSE`](LICENSE).
