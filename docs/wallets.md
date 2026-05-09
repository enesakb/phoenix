# Phoenix public addresses

This file tracks the public receive addresses Phoenix uses for donations and
the v0.8+ success-fee program. **Only public addresses appear here. The seed
phrases for these wallets are stored offline by the maintainer and never
appear in this repository, in chat logs, or in any committed file.**

If you find an address here that does not match a release-signed announcement
on https://github.com/enesakb/phoenix/releases, do not send funds — you may
be looking at an impersonator's repository fork.

## Active addresses (current — 2026-05-10 rotation)

| Chain | Address | Format | Active from |
|---|---|---|---|
| **Bitcoin** | `bc1p0730rztrz3yw3fc0an28tuxft0cstfcfr7mu0umc4scl8z0kradqzprlpr` | Taproot (P2TR, segwit v1) | 2026-05-10 |
| **EVM (Ethereum + L2s)** | `0x7C17c4937cABD75CB8657f5fb1c4184325Bff652` | EIP-55 checksummed | 2026-05-10 |
| **Solana** | `5De7kbLn9SSsKLVQCCMdcRyAvofeijD6VQPqc9CZXwyT` | base58 ed25519 | 2026-05-10 |

### EVM-compatible chains served by the same Ethereum address

The single EIP-55 address above is valid on every EVM-compatible network
because they all share the secp256k1 + keccak256 address derivation. Send
any of the following to `0x7C17c4937cABD75CB8657f5fb1c4184325Bff652`:

| Network | Type | Notes |
|---|---|---|
| Ethereum mainnet | L1 | Native ETH |
| Base | L2 | Coinbase chain |
| Monad | L1 (EVM-compatible) | Same address derivation as Ethereum |
| Optimism | L2 | OP Stack |
| Arbitrum | L2 | Arbitrum Nitro |
| Polygon | sidechain | PoS chain |
| BNB Chain | L1 | BSC |
| Avalanche C-Chain | L1 | EVM compatible |
| zkSync Era | L2 | zk-rollup |
| Linea | L2 | ConsenSys zk-rollup |

> When sending from one of the above networks, double-check that **your
> wallet is set to the correct network before pressing Send.** The address
> is the same; the network setting determines which chain the transfer
> actually lands on. We recommend a small test amount on any new chain.

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
| `bc1p0730rztrz3yw3fc0an28tuxft0cstfcfr7mu0umc4scl8z0kradqzprlpr` | BTC | 2026-05-10 | _(active)_ | _(active)_ |
| `0x7C17c4937cABD75CB8657f5fb1c4184325Bff652` | EVM | 2026-05-10 | _(active)_ | _(active)_ |
| `5De7kbLn9SSsKLVQCCMdcRyAvofeijD6VQPqc9CZXwyT` | SOL | 2026-05-10 | _(active)_ | _(active)_ |
| `bc1pswqcehh9nlkvnepulg58mp5p0kyfgcv3cw2s4qdljrwd9gtf80ksh4q9cg` | BTC | 2026-05-10 (briefly) | 2026-05-10 | Replaced before any deposits — wallet upgraded by maintainer |
| `0x1341e0A851E9c5656C65e764fE3B841908514603` | EVM | 2026-05-10 (briefly) | 2026-05-10 | Replaced before any deposits — wallet upgraded by maintainer |
| `J88Epp5C6oDVsFNLJwZuN8eQXjYiwJhpy1kjtUZY6J7p` | SOL | 2026-05-10 (briefly) | 2026-05-10 | Replaced before any deposits — wallet upgraded by maintainer |

If you sent funds to any of the **retired** addresses above, please reach out
via [GitHub Discussions](https://github.com/enesakb/phoenix/discussions) so
the maintainer can confirm receipt — those addresses are still controlled by
the same offline seed and not lost, but the new addresses are the canonical
ones going forward.

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
- **EVM** addresses are EIP-55 checksummed; mixed case is part of the
  checksum and must be preserved when copy-pasting. The same address is
  valid on Ethereum, Base, Monad, and every other EVM-compatible network
  listed above — set your wallet's network correctly before sending.
- **Solana** is the standard 32-byte ed25519 public key encoded as base58.
  Phantom, Solflare, Backpack, Trust Wallet, and the Solana CLI all use this
  format identically.
