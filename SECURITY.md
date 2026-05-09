# Security policy

## Scope

Phoenix is a forensic recovery tool for crypto wallets. Security issues we care about include:

- Any path where a user's seed, private key, or passphrase could be exfiltrated to a third party
- Any pipeline regression that breaks the local-only invariant
- Any cryptographic mistake (wrong derivation, wrong checksum, address mismatch)
- Any supply-chain risk in our build pipeline (compromised dependency, unsigned binary)
- Any vulnerability in the forensic file parsers (CSV, mbox, SQLite reads of attacker-controlled files)
- Any side-channel leak during cracking

## Reporting a vulnerability

**Please do not open a public GitHub issue for security vulnerabilities.**

Open a private vulnerability report via GitHub's [Private Vulnerability Reporting](https://github.com/enesakb/phoenix/security/advisories/new). Include:

- A description of the vulnerability
- Steps to reproduce
- The affected component (`phoenix-core`, `phoenix-cli`, `phoenix-tauri`, `src-ui`)
- A suggested fix if you have one

We aim to acknowledge reports within **72 hours** and ship a fix within **14 days** for high-severity issues.

## Coordinated disclosure

We follow a 90-day coordinated disclosure window:

- Day 0: report received, acknowledged
- Day 1–14: triage, patch developed, tested
- Day 15–60: patch released, users notified to upgrade
- Day 90: full disclosure published

If the issue is already being exploited in the wild, we may shorten the window.

## Out of scope

- Wallets where all 12 BIP-39 words are lost with zero recoverable information — by design, no tool can recover this. If a "recovery" tool claims it can, that tool is the threat. Phoenix does not claim it.
- Hardware-glitch attacks on Trezor / Ledger PINs — out of scope for this project.
- Vulnerabilities in upstream dependencies (rustsec / crates.io / npm registry) — please report those upstream first.
- Stolen-wallet recovery — Phoenix declines to service these cases at the success-fee tier; the Chainalysis / TRM Labs filter is enforced there. We will not negotiate around this filter.

## Acknowledgements

Researchers who report verified vulnerabilities will be credited in the [SECURITY-HALL-OF-FAME.md](SECURITY-HALL-OF-FAME.md) file (created on first credit) and in the release notes for the patched version, unless they request otherwise.
