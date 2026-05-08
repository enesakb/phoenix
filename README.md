# Phoenix

> Open-source forensic recovery assistant for partial-information lost crypto wallets.

**Status:** Pre-alpha (Week 1 — foundation). Not ready for use.

## What this is

Phoenix systematizes what artisan wallet-recovery shops do manually: a structured cognitive interview, deep digital exhaust forensics, Bayesian candidate ranking, and distributed cracking — orchestrated as a single open-source desktop application.

## What this is NOT

- Not a recovery promise. Wallets with zero memory and zero digital traces cannot be recovered. Period.
- Not a hardware-glitch tool. Hardware PIN attacks are out of scope.
- Not a cloud service. All recovery work happens locally on your machine.
- Not "AI wallet recovery." Phoenix is forensic and guided, not magic.

## Realistic outcomes

For wallets where partial information exists (forgotten 1-2 seed words, wallet.dat with remembered password pattern, lost backups with traceable digital exhaust): expected v1 recovery rate **35-50%**.

For wallets with no recoverable signal: **0%**. Always.

## Design

See [`docs/superpowers/specs/2026-05-08-phoenix-design.md`](docs/superpowers/specs/2026-05-08-phoenix-design.md).

## License

MIT. See [`LICENSE`](LICENSE).
