# Phoenix benchmarks

Reproducible performance numbers for the recovery pipeline across chains,
hardware, and scenarios.

All numbers below are from running the release-mode binary on the maintainer's
Windows 11 machine (8-core CPU, 32 GB RAM). Your hardware will differ; treat
these as ratios more than absolutes. Reproduce on your own machine with the
shell scripts in [`examples/`](../examples/).

## Single missing word recovery

The headline use case: 11 of 12 BIP-39 words known plus the target on-chain
address. Phoenix iterates all 2048 candidate words in parallel, filters
checksum-valid combinations (~128 of 2048 pass), derives an address, and
matches.

| Chain | Derivation path | Time |
| :--- | :--- | ---: |
| Ethereum (and EVM L2s) | `m/44'/60'/0'/0/0` | **6–17 ms** |
| Bitcoin (native segwit) | `m/84'/0'/0'/0/0` | **5–10 ms** |
| Solana (Phantom path) | `m/44'/501'/0'/0'` | **5 ms** |
| Solana (Solflare path) | `m/44'/501'/0'` | <10 ms |
| Solana (Sollet legacy) | `m/501'/0'/0'/0'` | <10 ms |

The variability comes from CPU contention. Idle laptop = bottom of range;
laptop on battery doing other work = top of range.

## Two missing word recovery

Hardest case Phoenix supports without GPU offload. 2048² = 4.2 million raw
combinations × checksum filter (~262k valid) × address derivation.

| Chain | Time (8 cores, release) |
| :--- | ---: |
| Ethereum | 3–10 seconds |
| Bitcoin | 2–8 seconds |
| Solana | 1–5 seconds (no checksum filter; faster derivation) |

3+ missing words yields 8.6 billion+ combinations and is intentionally
rejected — Phoenix builds a Hashcat command for offload to a GPU rig
instead. See `crypto::hashcat::build_command`.

## Passphrase brute force

Given a complete mnemonic + target address + candidate passphrase list,
Phoenix tries each candidate in parallel.

| Candidate count | Time |
| :--- | ---: |
| 50 (curated common BIP-39 patterns) | <50 ms |
| 1,000 | 0.5–2 seconds |
| 100,000 (a real user-supplied dictionary) | 30–90 seconds |

The bottleneck is `mnemonic_to_seed` (PBKDF2-HMAC-SHA512), which is
intentionally slow per design of BIP-39 (2048 iterations).

## Cognitive interview round-trip

Time for a single user-answer to be processed by the local LLM and produce
extracted memory nodes.

| Model | First-question latency | Per-question latency |
| :--- | ---: | ---: |
| Qwen 3 14B | 2–5 seconds (cold) | 200–800 ms |
| Llama 3.3 70B | 10–30 seconds (cold) | 1–3 seconds |
| Llama 3.1 8B | 1–3 seconds (cold) | 100–400 ms |

Quality scales with size; speed scales inversely. We recommend Qwen 3 14B
as the default — fits in 9 GB of VRAM, fast enough for a flowing interview.

## Forensic file extraction

| Source | Typical file size | Time |
| :--- | ---: | ---: |
| Bitwarden CSV (10k passwords) | 5 MB | 50–200 ms |
| Chrome History SQLite (yearly) | 50 MB | 200–800 ms |
| mbox archive (10k messages) | 500 MB | 5–15 seconds |
| Plain text file | varies | 50 ms / MB |

The BIP-39 word scanner runs at roughly 50 MB/s of pure text on a single
core. Larger files would benefit from chunking; we have not yet seen a
real case that needs it.

## Methodology

To reproduce the single-missing-word numbers above:

```bash
# Build the release binary
cargo build --release -p phoenix-cli

# Run the benchmark for ETH
time ./target/release/phoenix.exe reconstruct \
    --template "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?" \
    --target  "0x9858effd232b4033e47d90003d41ec34ecaeda94" \
    --kind    eth

# Same for BTC
time ./target/release/phoenix.exe reconstruct \
    --template "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?" \
    --target  "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu" \
    --kind    btc

# Same for Solana
time ./target/release/phoenix.exe reconstruct \
    --template "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?" \
    --target  "HAgk14JpMQLgt6rVgv7cBQFJWFto5Dqxi472uT3DKpqk" \
    --kind    sol
```

The `examples/` directory contains shell scripts that automate this.

## Hardware impact

These numbers are linear in CPU core count for parallel work (1- and 2-word
brute force) and linear in clock speed for serial work (forensic file scans
and LLM round-trips).

| Hardware tier | Single-word ETH (typical) |
| :--- | ---: |
| Laptop (4 cores, 2.5 GHz) | 30–60 ms |
| Desktop (8 cores, 4.0 GHz) | 7–17 ms |
| Workstation (16 cores, 4.5 GHz) | 3–8 ms |
| Server (64 cores, 3.5 GHz) | <2 ms (CPU-coordination cost dominates) |

For the 2-word case, performance is roughly inversely proportional to total
core count. A 16-core machine does it in 1.5–5 seconds; a 64-core machine
in 0.5–2 seconds.

## Why we measure this way

We publish wall-clock times instead of cycles or instructions because:

1. The user-perceived experience is wall-clock. "Did the wallet open in a
   second or in a minute?" is the only question that matters.
2. Phoenix is single-binary, so cycles-per-instruction varies meaningfully
   between Linux/Windows/macOS due to scheduling.
3. The ratio of CPU time to BIP-39 PBKDF2 time is roughly fixed across
   hardware, so wall-clock at one scale predicts wall-clock at another.

## CI-side numbers

The GitHub Actions CI runs the test suite on Ubuntu, Windows, and macOS.
Workspace test wall-clock times (debug mode):

| Job | Tests | Time |
| :--- | ---: | ---: |
| Rust ubuntu-latest | 79 | 1m 12s |
| Rust windows-latest | 79 | 1m 43s |
| Rust macos-latest | 79 | 39s |
| UI tests (Vitest) | 7 | 15s |
| clippy | — | <30s |
| eslint | — | <30s |

These dominate the build time of every PR. We optimize for "passes in under
3 minutes total" across all jobs in parallel.
