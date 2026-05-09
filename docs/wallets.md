# Phoenix public addresses

This file tracks the public receive addresses Phoenix uses for donations and
the v0.8+ success-fee program. **Only public addresses appear here. The seed
phrases for these wallets are stored offline by the maintainer and never
appear in this repository, in chat logs, or in any committed file.**

If you find an address here that does not match a release-signed announcement
on https://github.com/enesakb/phoenix/releases, do not send funds — you may
be looking at an impersonator's repository fork.

## Active addresses (v0.8 launch — 2026-05-10)

| Chain | Address | Format | Active from |
|---|---|---|---|
| **Bitcoin** | `bc1pswqcehh9nlkvnepulg58mp5p0kyfgcv3cw2s4qdljrwd9gtf80ksh4q9cg` | Taproot (P2TR, segwit v1) | 2026-05-10 |
| **Ethereum** | `0x1341e0A851E9c5656C65e764fE3B841908514603` | EIP-55 checksummed | 2026-05-10 |
| **Solana** | `J88Epp5C6oDVsFNLJwZuN8eQXjYiwJhpy1kjtUZY6J7p` | base58 ed25519 | 2026-05-10 |

These addresses receive:

- **Voluntary donations** from anyone who finds Phoenix useful
- **Recovery success-fee tips** in v0.7 (manual user-signed transfers)
- **Atomic 5% splits** in v0.8+ (smart-contract enforced; opt-in per recovery)

The maintainer holds the seed phrase for these wallets offline. Phoenix the
project does not custody, manage, or have programmatic access to any of these
funds beyond what the seed-holder personally signs.

## Address rotation log

| Address | Chain | Active from | Retired on | Reason |
|---|---|---|---|---|
| `bc1pswqcehh9nlkvnepulg58mp5p0kyfgcv3cw2s4qdljrwd9gtf80ksh4q9cg` | BTC | 2026-05-10 | _(active)_ | _(active)_ |
| `0x1341e0A851E9c5656C65e764fE3B841908514603` | ETH | 2026-05-10 | _(active)_ | _(active)_ |
| `J88Epp5C6oDVsFNLJwZuN8eQXjYiwJhpy1kjtUZY6J7p` | SOL | 2026-05-10 | _(active)_ | _(active)_ |

## Verification

To verify a published address actually belongs to the Phoenix project:

1. Check this file's git history. Any address change must appear as a commit
   on `master` of `https://github.com/enesakb/phoenix`.
2. Cross-reference with the release notes for the corresponding version on
   https://github.com/enesakb/phoenix/releases.
3. Send a $1 test before any larger transfer.

## Reporting suspected fraud

If you encounter an address being passed off as Phoenix's that does not appear
in this file, please open a GitHub Discussion in the `security` category so
we can warn other users.

## Address format notes

- **Bitcoin** uses Taproot (P2TR, native segwit v1, `bc1p…`). Modern BTC
  wallets (Sparrow, Wasabi, BlueWallet, hardware wallets) all support Taproot.
  Older wallets (Bitcoin Core <22, Electrum <4.2) may not — those should
  either upgrade or coordinate a non-Taproot transfer with the maintainer.
- **Ethereum** is EIP-55 checksummed; mixed case is part of the checksum and
  must be preserved when copy-pasting. EVM-compatible chains (Base, Optimism,
  Arbitrum, Polygon, BNB Chain) accept the same address.
- **Solana** is the standard 32-byte ed25519 public key encoded as base58.
  Phantom, Solflare, Backpack, Trust Wallet, and the Solana CLI all use this
  format identically.
