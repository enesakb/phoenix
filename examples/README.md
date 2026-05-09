# Phoenix examples

Runnable shell scripts demonstrating common recovery scenarios. Each script
uses the standard BIP-39 zero vector ("abandon × 11 about") so you can run
them without leaking a real seed. Replace the inputs with your own to recover
your wallet.

## Prerequisites

Either:

1. Build from source (`cargo build --release -p phoenix-cli`) — the
   examples assume `./target/release/phoenix.exe` is on disk
2. Download a release binary from the
   [Releases page](https://github.com/enesakb/phoenix/releases/latest) and
   replace `./target/release/phoenix.exe` in each script with the path to
   your downloaded binary.

## Examples

### `01_eth_one_missing_word.sh`

Recover the missing word of a 12-word BIP-39 mnemonic where the user knows
their Ethereum address. Most common case. Sub-second.

```bash
./examples/01_eth_one_missing_word.sh
```

### `02_btc_one_missing_word.sh`

Same as above but for Bitcoin native segwit (Taproot wallets need a
separate path; reach out via Discussions if you want it).

```bash
./examples/02_btc_one_missing_word.sh
```

### `03_solana_one_missing_word.sh`

Solana recovery across the three wallet derivation paths (Phantom,
Solflare, Sollet). Phoenix tries all three automatically.

```bash
./examples/03_solana_one_missing_word.sh
```

### `04_show_solana_addresses.sh`

Given a mnemonic, print the three candidate Solana addresses Phoenix
derives — useful for verifying which wallet a seed corresponds to.

```bash
./examples/04_show_solana_addresses.sh
```

### `05_two_missing_words.sh`

Two missing words at the end of an ETH mnemonic. Demonstrates the 4.2M
parallel cartesian product (with checksum filter). 3–10 seconds.

```bash
./examples/05_two_missing_words.sh
```

### `06_create_a_fresh_wallet.sh`

Generate a fresh BIP-39 mnemonic locally with OS-RNG entropy. Phoenix
prints the seed once; it is your responsibility to write it on paper.
Phoenix never persists or transmits the seed.

```bash
./examples/06_create_a_fresh_wallet.sh
```

### `07_doctor_and_health_check.sh`

Sanity check: prints the version, exits 0 if the binary works.

```bash
./examples/07_doctor_and_health_check.sh
```

## Replacing test inputs with your own

For a real recovery, edit the relevant `.sh` file:

- `--template` — your 12 words with `?` for the unknown one
- `--target` — your wallet's on-chain address
- `--kind` — `eth`, `btc`, or `sol` matching your wallet

For example, if your real ETH wallet is `0xabc...123` and you know all 12
words but cannot remember the order of words 7 and 8, replace `01_eth_one_missing_word.sh`
with:

```bash
./target/release/phoenix.exe reconstruct \
    --template "your-word-1 your-word-2 ... ? ? ... your-word-12" \
    --target  "0xabc...123" \
    --kind    eth
```

(Note: `?` for both unknown positions; Phoenix's 2-word brute force handles
this case in 3–10 seconds.)

## Privacy reminder

These examples use a publicly known test mnemonic that controls zero funds.
**When working with your own wallet:**

- Never paste your real mnemonic in a public chat, GitHub issue, or any
  cloud-synced file.
- Run Phoenix on an air-gapped machine if you are paranoid (the binary works
  with no network).
- Verify the binary's SHA-256 against `SHA256SUMS.txt` from the release
  before running.

See [`SECURITY.md`](../SECURITY.md) for the full security posture.
