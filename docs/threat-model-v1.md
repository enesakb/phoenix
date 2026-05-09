# Phoenix Threat Model — v1 (Week 4)

Supersedes v0. Captures attack surfaces introduced by Layers 2 (forensic
imports), 5 (cryptographic reconstruction + passphrase brute force), and
the Hashcat command-builder helper. A v2 with formal attack trees is on
the roadmap before public launch.

## Trust boundaries (updated)

1. **User machine ↔ Phoenix process** — desktop process owned by the user.
   No elevation, no daemon, no persistent service.
2. **Phoenix ↔ Ollama** — HTTP loopback. The user is responsible for the
   integrity of their local Ollama install and any models they pulled.
3. **Phoenix ↔ User-imported files** — the user explicitly opens a file
   via the dialog plugin. Phoenix never walks the filesystem on its own.
4. **Phoenix ↔ Hashcat** — Phoenix only emits a *command string*; it does
   not run Hashcat. Execution responsibility stays with the user.
5. **Phoenix ↔ Internet** — limited to GitHub releases (signed updates)
   and opt-in telemetry. Both off by default.

## Adversaries

| Adversary | Capability | Phoenix v1 mitigation |
|---|---|---|
| Passive network observer | Reads cleartext traffic | All outbound HTTPS; no telemetry by default; cracking is local |
| Malicious Ollama image | User pulls a poisoned model | Out of scope; documented in README |
| Compromised Cargo / npm dependency | Supply-chain code injection | Lockfiles committed; `cargo audit` planned for CI in Week 8 |
| Phoenix maintainer inserts backdoor | Upstream compromise | Open-source from Day 1; reproducible builds in Week 8 |
| Stolen-wallet user | Tries Phoenix on a wallet they do not own | Chainalysis / TRM Labs API check at success-fee tier; ownership attestation gate (Week 7) |
| Malicious CSV / mbox / SQLite import | Crafted file triggers parser exploit | rusqlite is bundled SQLite (well-tested); csv crate is memory-safe; mbox parser walks UTF-8 strings only; depth-limited |
| ZIP-bomb / huge file import | OOM via large file | Path-based reads; no automatic decompression; user opens explicitly |
| Hashcat command-builder injection | User-supplied wordlist path interpolated unsafely | Builder uses `{:?}` debug formatting on `Path` (escapes); user copy-pastes the resulting command — no shell exec from Phoenix |
| Side-channel on cracking | Timing leaks expose candidate progress | Cracking is local-only; if user uses a shared GPU instance, their hosting provider is the trust boundary |

## New surfaces introduced this sprint

### 5.1 Forensic imports
- `Bip39TextExtractor` — UTF-8 read + tokenization. Worst case is large
  files; bounded by RAM. We do not yet stream — Week 8 task.
- `BitwardenCsvExtractor` — `csv` crate (well-fuzzed). Malformed rows are
  silently skipped. No PII leaves the machine.
- `ChromeHistoryExtractor` — bundled SQLite. Read-only opens. If the file
  is corrupt, we no-op (return empty) rather than panic.
- `MboxExtractor` — string split on `From ` markers; never executes any
  embedded HTML/JS. Subject lifting is purely cosmetic.

### 5.2 Multi-word brute force
- CPU exhaustion: 2-missing-word search is bounded at ~6-10s on 8 cores.
  3+ missing returns `TooManyMissing` rather than running unbounded.
- Output: only on match; no leak via timing.

### 5.3 Passphrase brute force
- User supplies the candidate list (or uses `common_passphrase_seeds()`).
  No network calls; nothing leaves the process.
- Discovered passphrase + recovered mnemonic are returned to the caller
  who is the same process that has the keys already; no extra exposure.

### 5.4 Hashcat command builder
- Phoenix never executes the command. The string is shown in the UI for
  the user to paste into their shell. We escape user-supplied paths via
  Rust's `Debug` formatting; we do not run shells.

## Local-only invariant (still holds)

> **Nothing sensitive leaves the user's machine without an explicit, named user action.**

Verifiable via:
- Network capture: only Ollama loopback + (optional) GitHub-releases
  manifest fetch. Telemetry off by default.
- Static audit: search the source for `reqwest`, `ureq`, `tcp_connect`,
  `http`. Only the Ollama client and update-fetch helpers exist.

## Outstanding pre-launch items

- `cargo audit` integration in CI (Week 8)
- Reproducible-build manifest (Week 8)
- Trail of Bits engagement (Week 8 — needs funding)
- Differential-fuzz harness for forensic parsers (Week 8 stretch)
