# Phoenix payment architecture

This document describes how money flows through Phoenix — both the on-chain
recovery success fee and the optional Pro subscription. It is the reference
that grounds the implementation work in v0.8 and beyond.

## Three-tier model

### Tier 1 — Donations (live today)

Phoenix publishes three public receive addresses (BTC, ETH, Solana) for users
who want to support development without needing a recovery. This is voluntary
and triggers no KYC/AML obligation because:

- It is a gift, not a payment for service
- The maintainer treats it as casual support, not commercial revenue
- Public addresses cannot be refused on chain

**To set this up safely:**

1. Create a fresh wallet using `phoenix wallet-create` OR a hardware wallet
   (Ledger, Trezor) OR an established consumer wallet (Phantom, MetaMask,
   Sparrow). **Never** generate a wallet inside a chat transcript or commit
   the seed phrase to git.
2. Write the seed on paper. Lock the paper somewhere safe.
3. Verify the public addresses by sending a small test amount and checking
   that the wallet UI shows the balance.
4. Add the public addresses to `README.md` and `site/index.html`. Public
   addresses are safe to share.

### Tier 2 — Recovery success fee (v0.8 — spec below)

When Phoenix recovers a wallet, the user is offered a one-click 5% donation
to Phoenix's success-fee address. This is opt-in, never coerced.

The implementation has two paths:

**Path A — Manual transfer (v0.7, no contract).**

After a successful recovery the UI displays:

> Recovery succeeded! If Phoenix helped, the project asks for a 5% donation
> to keep it free for the next user. **You can decline; no penalty.**
>
> [Send 5% via Phoenix wallet]   [Skip]

The "Send 5%" button opens the user's existing wallet (MetaMask / Phantom /
hardware) with a pre-filled transfer to Phoenix's address. The user signs
manually. No on-chain contract needed; no Phoenix custody at any point.

**Path B — Atomic split contract (v0.9, audited).**

A smart contract on Ethereum (and a Solana program for SOL chain recoveries)
that takes a recovered wallet's funds and atomically splits them:

- 95% to a user-provided "destination" address
- 5% to Phoenix's fee address

The user authorizes once at the moment of recovery. The contract executes
atomically — either both transfers happen or neither. Phoenix never holds
custody.

Pseudocode of the Solidity contract:

```solidity
contract PhoenixSplit {
    address public immutable PHOENIX_FEE_ADDR;
    uint256 public constant FEE_BPS = 500; // 5.00%

    constructor(address feeAddr) {
        PHOENIX_FEE_ADDR = feeAddr;
    }

    function split(address payable destination)
        external payable
    {
        uint256 fee = (msg.value * FEE_BPS) / 10000;
        uint256 user = msg.value - fee;
        (bool ok1,) = destination.call{value: user}("");
        (bool ok2,) = PHOENIX_FEE_ADDR.call{value: fee}("");
        require(ok1 && ok2, "split failed");
    }
}
```

Pre-launch checklist:
- [ ] Trail of Bits or Spearbit audit
- [ ] Deploy across Ethereum, Base, Optimism, Arbitrum
- [ ] User sees fee + final amount BEFORE signing
- [ ] Contract address pinned to the Phoenix release tag

**Why opt-in over coercion:** the wallet was the user's the whole time. If
Phoenix provided meaningful help, most users will tip without coercion (the
artisan recovery shop industry runs on this). Coercive contracts that hold
funds hostage attract regulatory scrutiny we want to avoid.

### Tier 3 — Pro subscription ($99/month, after legal entity exists)

Subscriptions need a legal entity, a bank account, and a payment processor.
Phoenix will not accept subscription payments before all three exist:

- [ ] Legal entity (DE GmbH, Estonia OÜ, US Wyoming LLC, or similar)
- [ ] Business bank account
- [ ] Stripe account linked to the entity (or Crypto.com Pay, Coinbase Commerce
      for crypto-native subscriptions)
- [ ] Terms of Service drafted by counsel
- [ ] Privacy policy drafted by counsel
- [ ] Tax registration in the relevant jurisdiction(s)
- [ ] AML / KYC vendor selection (only triggered at the success-fee tier
      where ownership attestation matters)

This list is sequential, not parallel. The right time to start is **after the
first 3-5 successful pilot recoveries demonstrate product-market fit** and the
Pro tier shows clear demand. Building the legal stack before there is revenue
to support it is a common founder mistake.

## What Phoenix does NOT do

- We do not store user seeds or keys. The recovery happens on the user's
  machine. Phoenix never has the cryptographic ability to move user funds
  without the user signing.
- We do not custody user funds. There is no "Phoenix wallet that holds your
  money." The success fee is a transfer from the user to a public address;
  Phoenix's wallet is no different from any other recipient.
- We do not collect more than the user explicitly authorizes. The 5% fee is
  computed and shown before the user signs. We do not have the ability to
  take more.
- We do not service stolen wallets. Chainalysis / TRM Labs check at the
  success-fee tier blocks known-stolen target addresses before recovery
  begins.

## Setting up the receive wallet — recommended flow

1. Buy a **Ledger Nano X** ($79) or **Trezor Safe 3** ($69). Hardware wallet
   is the gold standard.
2. Initialize the device. Write the 24-word seed on the included recovery
   sheet. Store the sheet in a fireproof safe or split across two locations.
3. Create three accounts on the device — BTC, ETH, Solana.
4. Copy the three public receive addresses. Paste into a draft of
   `README.md` and `site/index.html` under a "Donations" section.
5. Send a $5 test transaction to each address. Verify the wallet shows the
   balance. Cancel any auto-send schedules.
6. Open a pull request. Maintainer merges. Addresses are now public.

**Alternative if hardware wallet is not available:**

- **MetaMask** (browser extension) — for ETH receive address. Backed by
  password + seed; less secure than hardware but acceptable for low balances.
- **Phantom** — for Solana receive address. Same trade-offs.
- **Sparrow** — for BTC, desktop, supports air-gap with a hardware wallet.
- **`phoenix wallet-create` CLI** — generates a fresh seed locally. Use only
  if you understand that an unencrypted machine is the threat model. Move
  the seed to paper / hardware wallet immediately afterward.

## Operational hygiene

- Donation address rotation: once a year, generate a new wallet and migrate.
  Rotation contains breaches and limits taint propagation if any donor turns
  out to be sanctioned.
- Public address audit log: maintain a `docs/wallets.md` (this file's sibling)
  recording every active address, the date it became active, and the date it
  was retired. Visible in git history.
- Multi-sig migration: once Phoenix passes $50k in cumulative donations or
  fees, migrate to a 2-of-3 multi-sig (Safe.global on EVM, Squads on Solana).
  Single-sig is a single-key risk.
