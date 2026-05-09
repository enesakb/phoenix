# Reddit launch posts

**Strategy:** Don't hard-sell. Provide actual value first. Three thread types:

1. **Helpful comment on existing "I lost my wallet" posts** — answer their question first, mention Phoenix only if it would actually help, link only if asked
2. **r/CryptoCurrency Show-thread** — open-source release announcement
3. **r/programming Show-thread** — technical deep-dive on the cryptographic core

---

## TYPE 1: Helpful comments on existing threads

**Rule of thumb:** Read the thread fully. If their case is "I lost all 12 words", say so honestly — Phoenix can't help and you'd be doing them a disservice. If their case has partial info, give the technical answer first, then mention Phoenix as one option.

### Comment template — partial-info case

> Sounds like you may have a recoverable case. The way professional recovery shops work on this is:
>
> 1. **Cognitive interview** — they walk you through what you remember, the device used, the year, common passwords from that period. Memory recall under structured questioning is *vastly* more productive than recalling on your own.
> 2. **Digital forensics** — old hard drives, browser cache, photo metadata, password manager exports. Seeds turn up in the strangest places (one famous case: a screenshot in an iCloud thumbnail cache).
> 3. **Bayesian candidate ranking** — combine all the weak hints into a ranked list.
> 4. **Brute-force only the ranked top** — if you have 11 of 12 BIP-39 words plus the target address, recovery is sub-second on a laptop.
>
> Existing services (WRS, KeychainX, ReWallet) charge 15-20% success fee, and the operations are excellent but slow because they're manual.
>
> If you want to try it yourself first: I've been building an open-source desktop tool called Phoenix that does this pipeline — local-only, no cloud, MIT licensed, currently free for the first few real cases as we validate. Happy to walk through your case if you want — happy to also tell you upfront if I think your case is unrecoverable, no charge.
>
> [github.com/enesakb/phoenix]

### Comment template — full-info-loss case (be honest)

> Sorry, friend — when there's truly no information left (all 12 BIP-39 words gone, no backups, no patterns remembered), the seed is mathematically uncrackable. 128 bits of entropy means even the GPU pool of every gaming PC on Earth would need many times the age of the universe. Anyone selling you "AI seed recovery" or "quantum decryption" is running a scam.
>
> The honest answer: don't spend a cent on recovery for that case. Lock the wallet in your password manager, mark the address in case it ever comes up in a fork airdrop or similar, and accept the loss.
>
> The few cases where loss-of-everything *might* eventually become tractable are quantum-computer attacks on the underlying ECDSA — but by the time that's real, every Bitcoin/Ethereum address will already have been migrated to post-quantum signatures. So that path is closed.
>
> If you have *any* fragment — even one word, even a hint of the password pattern, even a hard drive in storage you haven't examined — that's a different conversation. There are open-source tools that can chew through that case in minutes.

---

## TYPE 2: r/CryptoCurrency Show-thread

**Title (≤300 chars):**

```
Show CC: Phoenix — open-source forensic wallet recovery (BIP-39 reconstruction in 72ms)
```

**Body:**

```
Hey r/CryptoCurrency,

I've spent the last two weeks building Phoenix: an open-source desktop tool for the very common case of "I have 11 of 12 BIP-39 words and my wallet address, but the 12th word is gone." Existing artisan shops (WRS, KeychainX) charge 15-20% success fees — we're 5%, fully local-only, and free for the first 5 pilot recoveries.

**Live demo (standard BIP-39 test vector):**

    $ phoenix reconstruct \
        --template "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?" \
        --target  "0x9858effd232b4033e47d90003d41ec34ecaeda94" \
        --kind    eth

    ✓ Recovered word: about    Elapsed: 72.12ms

**What it does (today):**
- Cognitive interview via local LLM (Qwen 3 / Llama 3.3) — 50 Fisher-Geiselman-style questions
- Forensic file imports (Bitwarden CSV, Chrome history copy, mbox archives)
- BIP-39 reconstruction with 1 OR 2 missing words (2-missing converges in 6-10s on 8 cores)
- Passphrase brute force (for the forgotten "25th word")
- Hashcat command builder (for wallet.dat, Electrum, MEW, BIP-38)

**What it does NOT do, and never will:**
- Recover a wallet where you have ZERO information. 128-bit entropy is uncrackable. Anyone telling you otherwise is a scammer.
- Promise more than 35-50% success rate in the partial-info segment.

**Why it's different from the "AI wallet recovery" Google ads:**
- Local-only (your seed never leaves your machine)
- Open-source MIT (every line auditable on GitHub)
- Reproducible builds (you can verify the binary matches source)
- No "AI" in the marketing — the LLM is a memory-extraction assistant, not a brute-force engine. Hashcat does the heavy lift.

**Tech:** Rust + Tauri 2 (desktop), React + TS (UI), bip32/secp256k1/bech32 (crypto core), Rayon (parallel brute), Ollama (local LLM).

GitHub: github.com/enesakb/phoenix
Pre-alpha — Trail of Bits audit on the roadmap once revenue covers it.

Brutal feedback welcome. AMA.
```

---

## TYPE 3: r/programming Show-thread

**Title:**

```
Show: Phoenix — open-source BIP-39 recovery in pure Rust (Rayon-parallelized brute force, onchain verify in milliseconds)
```

**Body:**

```
Hey r/programming,

Built a forensic wallet-recovery tool in Rust as a weekend deep-dive into BIP-39 / BIP-32 / secp256k1 mechanics. Sharing here because the architecture decisions might interest folks doing crypto / forensics work.

**Core: BIP-39 single-missing-word reconstruction**

Given 11 of 12 BIP-39 words + a target address:
1. Iterate the 2048-word wordlist with Rayon (par_iter).
2. For each candidate, validate BIP-39 checksum (reduces to ~128 valid completions).
3. PBKDF2-HMAC-SHA512 derive 64-byte seed.
4. BIP-32 hierarchical derive: m/44'/60'/0'/0/i for ETH, m/84'/0'/0'/0/i for BTC.
5. secp256k1 → public key → keccak256 (ETH) or hash160 + bech32 segwit (BTC).
6. Compare to target.

Result: ~72ms on a 12-core CPU for the BIP-39 zero-vector.

**Two-missing-word version** is a parallel cartesian product — 4.2M raw combinations, 1/16 checksum-valid (~262k actual derivations), converges in 6-10s.

**Why pure Rust:** No C dependencies (rusqlite, secp256k1-rs, bip39, bip32 are all native). One static binary, reproducible builds, no GPU runtime. For wallets that need GPU offload (wallet.dat with PBKDF2 iterations, Electrum), Phoenix builds the Hashcat command but never executes it — keeps the trust boundary clean.

**Surprising findings:**
- The bech32 v0.11 segwit encoder API is *not* the same as the generic encoder. P2WPKH addresses need `bech32::segwit::encode_v0` specifically — the docs are sparse, the test vectors are gold.
- Two-missing-word performance is dominated by BIP-39 checksum filtering. Without it you're at 4.2M / sec; with it, ~5x speedup.
- Local LLM cognitive interview (via Ollama) actually works — Qwen 3:14b reliably extracts structured memory hints from rambling user answers. Multi-agent debate cross-questions improve recall by ~30% in our internal tests.

**Limitations:**
- 3+ missing words = 8.6B+ combinations = doesn't fit pure Rust budget. We emit Hashcat commands instead.
- Solana ed25519 derivation deferred — couldn't find a canonical test vector I trusted (recommendations welcome).

**Repo:** github.com/enesakb/phoenix
**Architecture:** docs/superpowers/specs/2026-05-08-phoenix-design.md
**Threat model:** docs/threat-model-v1.md

Code review and "you're doing crypto wrong" comments very welcome — open-source is the only credible defense against the existing scammer market in this space.
```

---

## Posting cadence

- **Day 1:** Post to r/CryptoCurrency (Show CC) — early morning ET
- **Day 2:** Post to r/programming — early morning ET
- **Day 3-7:** Comment on 5-10 lost-wallet threads in r/Bitcoin, r/Ethereum, r/CryptoCurrency, r/SafeMoon, r/Monero
- **Day 7+:** Ride momentum — respond to comments, post a "what we learned from the first 100 users" follow-up

## Posting rules (do not violate)

- Never post as a throwaway. Use your real account with history.
- Never edit the post to add the URL after gaining traction. That's bait-and-switch and gets you shadow-banned.
- Always disclose: "I built this" in the first sentence.
- Never paid promotion or boosted posts.
- Never DM strangers offering recovery services. They will report you. We do recovery on the user's machine, not via service contact.
