# Twitter / X launch thread

## Master thread (10 tweets) — schedule for 14:00 UTC Tuesday

### Tweet 1 (hook)

```
Just shipped Phoenix: open-source forensic wallet recovery.

It just recovered a BIP-39 wallet in 72 milliseconds.

Here's how — and why every "AI wallet recovery" Google ad is a scam. 🧵
```

(attach: short MP4/GIF of the CLI command + recovery output)

### Tweet 2

```
First, the math:

A 12-word BIP-39 seed = 128 bits of entropy = 3.4 × 10³⁸ combinations.

Even with every GPU on Earth, brute-forcing all of them takes ~108 BILLION YEARS.

If you've truly lost ALL 12 words with zero information, your wallet is gone. Period.
```

### Tweet 3

```
But — and this is the part nobody talks about — most "lost wallets" aren't full-info loss.

They're:
• 11 of 12 words remembered
• Pattern of password remembered
• Photo of seed exists somewhere on an old device
• Wallet.dat with weak password
• Hardware wallet PIN forgotten

These cases are VERY recoverable.
```

### Tweet 4

```
Existing recovery shops (WRS, KeychainX, ReWallet) handle these cases — manually, by phone, charging 15-20% success fees.

That works at scale of 1 customer.

What's been missing: a productized version. Open-source. Local-only. 5% fee. Built for the partial-info segment specifically.
```

### Tweet 5 (the demo)

```
Here's the standard BIP-39 test vector. 11 known words, 12th unknown.

$ phoenix reconstruct \
    --template "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?" \
    --target  "0x9858effd232b4033e47d90003d41ec34ecaeda94" \
    --kind    eth

✓ Recovered word: about
  Elapsed: 72.12ms
```

(attach: terminal screenshot or short video of this exact command)

### Tweet 6

```
What's actually happening:

1. Iterate all 2048 BIP-39 words in parallel (Rayon)
2. Filter checksum-valid combinations (~128 of 2048 pass)
3. Derive seed (PBKDF2-HMAC-SHA512)
4. BIP-32 child key m/44'/60'/0'/0/0
5. secp256k1 → keccak256 → ETH address
6. Match target

Pure Rust. Zero magic.
```

### Tweet 7

```
Two-missing-word case: 4.2M combinations, ~262k checksum-valid, 6-10s on a regular laptop.

Three-missing-word case: 8.6B combos. Phoenix doesn't even try — it builds you the precise Hashcat command for offload to a GPU rig.

We refuse to misrepresent what's tractable.
```

### Tweet 8 (honesty)

```
What Phoenix is NOT:

❌ "AI wallet recovery" magic — that phrase is owned by scammers
❌ A cloud service — your seed NEVER leaves your machine
❌ A hardware-glitch tool — Trezor PIN attacks are out of scope
❌ 90% success — the realistic ceiling is 35-50% in the partial-info segment

Anyone who claims more is selling snake oil.
```

### Tweet 9 (open-source)

```
Why open-source matters here:

The "AI wallet recovery" Google ads are *literally* malware-adjacent — they ask for your seed, then drain your wallet. Or they "find" your seed, charge you upfront, then disappear.

Phoenix is MIT licensed. Every line is on GitHub. Reproducible builds. Trail of Bits audit on the roadmap once revenue funds it.

You can compile it yourself and verify the hash matches our release.
```

### Tweet 10 (CTA)

```
Pre-alpha. We're looking for the first 5 real partial-info cases — those will pay $0 fee.

If you have:
• 11/12 BIP-39 words + the target address, OR
• A wallet.dat with a remembered password pattern, OR
• Old backups you haven't searched yet

→ DM me or email noreply@phoenix.local

GitHub: github.com/enesakb/phoenix
```

---

## Reply-pinned tweet (post separately)

```
For people asking "is this a scam":

1. Repo is public: github.com/enesakb/phoenix
2. Build it yourself — `cargo tauri dev`
3. Air-gap your machine before running
4. Watch tcpdump during cracking — you'll see zero outbound traffic
5. Trust nothing, verify everything

That's the whole pitch.
```

---

## Quote-tweet hooks (for crypto Twitter accounts to engage)

When @aixbt or @0xngmi or similar quote a "lost wallet" tweet, reply:

```
For partial-info cases (11/12 words remembered, etc.) — we open-sourced a tool last week. Local-only, sub-second for single missing word: github.com/enesakb/phoenix

Full-info loss is mathematically gone, no tool can help. Anyone selling otherwise is a scammer. ⚠️
```

---

## DM templates (for cold reach — use sparingly, only to people who've publicly mentioned losing)

```
Hey, saw your tweet about losing access to your [wallet kind].

I just open-sourced a recovery tool that's free for the first 5 real cases — Phoenix. Local-only, MIT licensed. If you have any partial info (remembered words, patterns, old backups) it can probably help.

If you have ZERO info, I'd tell you that upfront for free — no point in either of us wasting time.

Want to walk through your case? No charge for the diagnosis.

GitHub: github.com/enesakb/phoenix
```

---

## Scheduling

- Tuesday 14:00 UTC: Master thread
- Wednesday 16:00 UTC: Reply-pinned scam-defense tweet
- Thursday-Friday: Engage with replies, quote-tweets, DMs
- Following Tuesday: Follow-up tweet with first-week metrics ("X downloads, Y recoveries attempted, Z bugs fixed")

## Anti-patterns (do NOT do)

- ❌ Never pay for promoted tweets
- ❌ Never use bot networks
- ❌ Never DM people who haven't publicly indicated they lost a wallet
- ❌ Never mention specific wallet addresses or amounts in public — even if a recovery succeeds, anonymize the case study
- ❌ Never reply with the link in threads where the user already gave up — that's funeral-spam
