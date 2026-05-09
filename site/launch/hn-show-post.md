# Hacker News Show HN post

**Title (≤80 chars):**

```
Show HN: Phoenix – Open-source crypto wallet recovery (BIP-39 reconstruction)
```

**URL:** https://github.com/enesakb/phoenix

**Body:**

```
Hi HN,

I built Phoenix because "AI wallet recovery" Google ads are 99% scams and the legit recovery shops (WRS, KeychainX, ReWallet) charge 15-20% success fees with no productized tooling. There's a missing middle.

Phoenix is a desktop application (Tauri + Rust + React) that handles the cases real recovery work already handles — partial-information BIP-39 mnemonics — and does it in milliseconds for the simple cases.

Live demo against the standard BIP-39 test vector ("abandon × 11 about" + 12th word missing):

    $ phoenix reconstruct \
        --template "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?" \
        --target  "0x9858effd232b4033e47d90003d41ec34ecaeda94" \
        --kind    eth

    ✓ Recovered word: about    Elapsed: 72.12ms

What it actually does:
- Local LLM-driven cognitive interview (Fisher-Geiselman protocol) using Ollama + Qwen 3 / Llama 3.3 to extract memory hints. Multi-agent debate (3 LLM perspectives, per Du et al. 2023) for cross-questioning.
- Forensic file scanner: Bitwarden CSV exports, Chrome History SQLite copies, mbox email archives. The mbox parser is dumb-by-design; we just walk RFC-822 messages and feed every body to the BIP-39 sequence detector.
- BIP-39 reconstruction: 1 missing word in milliseconds (Rayon parallel + checksum filter), 2 missing words in 6-10s. 3+ missing words is mathematically tractable on GPU but not on a single CPU — Phoenix builds you the Hashcat command instead of pretending to.
- Passphrase brute force for the forgotten "25th word" (BIP-39 passphrase). User supplies candidate passphrases (or uses a curated list of common patterns).
- Hashcat command builder for wallet.dat, Electrum, MEW V2, BIP-38, MultiBit, Blockchain.com formats. Phoenix never executes Hashcat itself — keeps the trust boundary clean.

What it does NOT do (deliberate):
- Recover a wallet with zero information. 128-bit entropy is mathematically uncrackable. Anyone selling otherwise is running a scam. Phoenix's marketing copy says this verbatim.
- Use the phrase "AI wallet recovery" — that category name is owned by scammers (see ai-seedfinder.com and walletrecovery.ai for prior art). We use "forensic recovery" instead.
- Touch your seed via cloud. The whole pipeline is local. Phoenix's only outbound traffic is the optional GitHub-releases update check (off by default).

Stack:
- Rust 1.83 workspace, Tauri 2.x, React 18 + TS strict
- bip32 + secp256k1 + bech32 (segwit) + tiny-keccak (ETH)
- Rusqlite (bundled SQLite) for browser history parsing
- bip39 crate (English wordlist embedded)
- Rayon for parallel brute, Reqwest+rustls for the Ollama HTTP client

Architecture:
- L1: Cognitive Excavation Engine (interview + LLM extractor)
- L2: Digital Forensic Excavator (file imports)
- L3: Constraint Inference (Bayesian + CSP — roadmap)
- L4: Distributed Cracking (hashcat orchestration — builder only today)
- L5: Verification + Restoration (the BIP-39/secp256k1/address core that does the actual recovery)
- L6: Federated Learning Loop (privacy-preserving cross-recovery model — roadmap)

What's left:
- KeePass kdbx parser (needs interactive master-password UX)
- Tesseract OCR for photographed seeds (native dep complexity)
- Solana ed25519 derivation (deferred until I find a canonical test vector I trust)
- Trail of Bits audit (deferred until revenue funds it; not blocking launch)

Pricing:
- Free tier: full cognitive interview + forensic imports + single-word reconstruction
- Pro $99/mo: 2-word brute, passphrase brute, Hashcat builder, priority support
- 5% success fee on actual recoveries (compare to 15-20% at artisan shops)
- Free for the first 5 real partial-info cases

Realistic ceiling: 35-50% success rate in the partial-info segment. We say this in the README. Anyone claiming higher is the scam genre.

Brutal feedback welcome. The cryptographic core is straightforward enough that mistakes show up loud — please look for them.

Repo: github.com/enesakb/phoenix
Threat model: docs/threat-model-v1.md
Spec: docs/superpowers/specs/2026-05-08-phoenix-design.md
```

---

## Comment-reply playbook (top expected questions)

### "How is this different from BTCRecover?"

```
BTCRecover (great tool, MIT-licensed, mature) is a CLI that requires the user to be comfortable with Python, regexes, and writing config files. It's the engine you'd want under a productized layer.

Phoenix is the productized layer: GUI for non-developers, cognitive interview to surface candidate words, forensic file parsers to find seeds in old data, and direct on-chain verification (so you stop the moment a candidate matches, instead of running through a wordlist blindly).

Where the cracking is the bottleneck (wallet.dat with strong password, etc.), Phoenix delegates to Hashcat — building the precise command — rather than reimplementing well-tuned crackers in Rust.
```

### "What if the recovery succeeds and you steal the user's wallet?"

```
Three layers of defense:
1. Cracking happens on the user's machine. Phoenix never sees the seed.
2. The "success fee" mechanism is opt-in and uses MPC threshold ECDSA — Phoenix sees a signature confirming match, not the key.
3. Open-source + reproducible builds — you can verify the binary you ran came from the source you read, and the source has zero outbound seed transmission.

If you don't trust any of that, air-gap your machine and run Phoenix offline. The recovery still works.
```

### "Why open-source if you want to charge?"

```
Two reasons:
1. The category name is poisoned. Every paid wallet-recovery service is presumed scammy. Open-source is the only credible counter-signal.
2. The product isn't the brute-force engine — it's the cognitive interview, the forensic parsers, the integrated UX, and the brand. Hashcat is also open-source and has a thriving paid services ecosystem on top of it.

The 5% success fee is for users who want a turnkey end-to-end solution. The free tier is a complete tool — no crippling.
```

### "How does the LLM not hallucinate seed words?"

```
The LLM never proposes seed words. Its job is to extract structured memory hints from a free-form interview answer (e.g. "user used dog names + 4-digit years as password roots").

The brute-force layer iterates the canonical BIP-39 wordlist (2048 words) and validates each candidate cryptographically against the target address. There's no path for an LLM to invent a non-wordlist token that gets accepted.

Multi-agent debate (3 LLMs cross-question the user) is for surfacing additional hints — it does not vote on candidate words.
```

### "Can you do Solana?"

```
Not yet. Solana uses ed25519 (not secp256k1) and SLIP-10-style HD derivation. The crates exist (slip10_ed25519, ed25519-dalek, bs58) but I haven't been able to find a canonical test vector I trust enough to ship without risk of producing wrong addresses for users — and "wrong address" in this domain means "user thinks their seed is gone when it's actually there".

If anyone here knows the canonical Solana derivation test vector, please drop it in this thread. I'll integrate within a week.
```

---

## Submission timing

- **Best:** Tuesday or Wednesday 09:00-10:30 ET (14:00-15:30 UTC)
- **Avoid:** Friday afternoon, weekends
- **Title length:** Under 80 chars (HN truncates)
- **First-comment:** Have your top defense ("how is this different from BTCRecover") ready to post within 5 minutes

## Anti-patterns (do NOT do)

- ❌ Don't ask for upvotes (instant flag)
- ❌ Don't post from multiple accounts
- ❌ Don't edit the title after posting unless requested by mods
- ❌ Don't reply with sales pitches; HN rewards technical depth
- ❌ Don't link to a paywall or signup; the GitHub link is the front door
