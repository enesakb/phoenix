# Phoenix — Forensic Wallet Recovery Platform

**Version:** 1.0
**Date:** 2026-05-08
**Codename:** Phoenix (public name to be decided pre-launch; "AI wallet recovery" branding is not used)

---

## 1. Executive summary

Phoenix is an open-source, locally executed, AI-augmented forensic recovery platform for the **partial-information segment** of lost crypto wallets. It systematizes what artisan recovery shops (WRS, KeychainX, ReWallet) do manually today: a structured cognitive interview, deep digital exhaust forensics, Bayesian candidate ranking, and distributed cracking — orchestrated as a single consumer product.

**Bounded claims:**
- Full-information-loss seeds are **not recoverable** (128-bit entropy, physical limit)
- Hardware-glitch / firmware-fault attacks are **out of scope** (Praefortis / Unciphered territory)
- The "AI wallet recovery" buzzword is **not used** (scam-genre association)
- No seed or key **leaves the user's machine**

**Target success rate (within target segment):** 35–50% in v1, 50–70% after 12 months of federated-learning improvements.

**Core thesis:** First industrialization of the recovery industry. Today's artisan = tomorrow's product.

---

## 2. Problem and market

### 2.1 Market by the numbers
- Total lost BTC: ~3.7M coins (Chainalysis 2024) ≈ **$200B+**
- Pump.fun: 1.4M active wallets, continuous stream of new "lost dust positions"
- Reddit r/Bitcoin / r/Ethereum / r/CryptoCurrency "lost wallet" threads: 25–50 new posts per day
- Artisan recovery market: ~$50–100M annual estimate, +20% YoY growth

### 2.2 Existing services and their gaps

| Service | Model | Fee | Gap |
|---|---|---|---|
| Wallet Recovery Services (Dave Bitcoin, est. 2013) | Manual consultancy | 20% | Hours of phone calls, no scale, no AI |
| Crypto Asset Recovery (Chris Brooks) | Manual + viral YouTube | 20% | Family-run, capacity-bound |
| KeychainX (CH) | Manual + glitch | 10–20% | Enterprise-focused, no retail UX |
| ReWallet (DE/CH) | Air-gapped HPC | 20% | German-first, language-constrained |
| Praefortis / Unciphered (US) | Hardware glitch | Per-case | Expensive, hardware-only |
| BTCRecover (open-source CLI) | DIY | Free | 95% of users lack the technical bar |
| seedcat / WalletGen | DIY GPU brute | Free | CLI, Python required |

**Missing middle:** between artisan and CLI, no productized layer exists.

### 2.3 Why it is buildable in 2026 and was not before

1. **LLM-driven cognitive interview** — Opus 4.7 / Llama 4 class models can now run a structured memory-recovery interview reliably; this was not feasible two years ago
2. **Local LLM economics** — Llama 3.3 70B runs in 32 GB VRAM, so user data never has to leave the machine
3. **GPU is commodity** — RTX 4090 does ~10⁹ BIP-39 attempts/second; rented cloud GPU is ~$1.50/hour
4. **Tauri / Rust** — desktop app packaging is now both safe and fast
5. **MPC maturity** — atomic success-fee threshold ECDSA is production-ready

---

## 3. Non-goals (what Phoenix explicitly does not do)

- Recover seeds when no information is left (mathematically impossible — 128-bit entropy)
- Hardware-glitch / firmware-fault attacks (Praefortis / Unciphered domain)
- Recover stolen / phishing-drained funds (AssetReality, Aegis domain)
- Pre-protection (multi-sig, social-recovery wallet) — Argent / Safe / Daimo domain
- Quantum decryption (when Q-Day arrives, the protocol will already be post-quantum)
- Custodian dormant-account recovery (handled by Coinbase / Binance directly)
- Make claims like "guaranteed recovery" or "90% success rate" — that is the scam dialect

---

## 4. Target user segment

### 4.1 Tractability map

| Case | Frequency | Tractability | Phoenix v1? |
|---|---|---|---|
| **A.** 11/12 words remembered | High | 🟢 minutes | ✅ |
| **B.** All words remembered, wrong order or typo | Very high | 🟢 hours | ✅ |
| **C.** Encrypted wallet.dat + remembered password pattern | High | 🟡 days | ✅ |
| **D.** Old device or backup exists, no access | Medium | 🟡 forensic + cracking | ✅ |
| **E.** Seed photographed, photo deleted | Medium | 🟡 thumbnail recovery + OCR | ✅ |
| **F.** Hardware-wallet PIN forgotten (Trezor) | Low frequency, high value | 🟡 glitch attack | ❌ (out of scope) |
| **G.** All 12 words gone, zero traces | Medium | 🔴 impossible | ❌ |

**Phoenix v1 segment:** A + B + C + D + E. Estimated coverage: **50–60% of all lost-wallet cases**.

### 4.2 Personas

**Primary — "Frustrated Self-Custodian":**
- 30–50 years old, technically literate but not a CLI user
- Bought BTC/ETH between 2017–2021, set up a wallet
- Now $5k–$500k stuck in a forgotten wallet
- Tried BTCRecover and gave up, or balked at WRS pricing
- Has posted "I lost my crypto" on Reddit / Twitter

**Secondary — "Heir / Inheritor":**
- A relative passed away without leaving the seed/password
- Computers, phones, papers inherited
- Legal succession completed, ownership verifiable
- Phoenix is ideal for forensic extraction across the inherited devices

---

## 5. System architecture — six layers

```
┌──────────────────────────────────────────────────────────┐
│  Tauri Desktop Shell (Rust + React UI)                   │
└────────────┬─────────────────────────────────────────────┘
             │
┌────────────▼─────────────────────────────────────────────┐
│  Layer 1: Cognitive Excavation Engine                    │
│  - Local LLM (Llama 3.3 70B via Ollama)                  │
│  - Cognitive Interview Protocol (Fisher & Geiselman)     │
│  - RL interview policy (Bayesian Optimal Experimental)   │
│  - Multi-agent debate cross-questioning                  │
└────────────┬─────────────────────────────────────────────┘
             │
┌────────────▼─────────────────────────────────────────────┐
│  Layer 2: Digital Forensic Excavator                     │
│  - File carving (Foremost / Scalpel algorithms)          │
│  - OCR + VLM (Tesseract + Llama Vision)                  │
│  - Browser forensics (Chrome / Firefox)                  │
│  - Password manager dump parsing                         │
│  - Email backup mining                                   │
│  - Photo EXIF + perceptual hashing                       │
│  - iCloud / Drive backup parsing (with permission)       │
└────────────┬─────────────────────────────────────────────┘
             │
┌────────────▼─────────────────────────────────────────────┐
│  Layer 3: Constraint Propagation & Inference             │
│  - CSP (arc consistency, AC-3, backjumping)              │
│  - Bayesian Networks (Pearl 1988)                        │
│  - Hidden Markov Models for pattern modeling             │
│  - MCMC for low-prob candidate sampling                  │
│  - Damerau-Levenshtein for typo prediction               │
│  - PassGPT-style transformer (per-user fine-tune)        │
└────────────┬─────────────────────────────────────────────┘
             │
┌────────────▼─────────────────────────────────────────────┐
│  Layer 4: Distributed Cracking Engine                    │
│  - hashcat (modes 11300/12700/15700/18800)               │
│  - seedcat (BIP-39 GPU brute)                            │
│  - Custom CUDA kernels (Phantom/Solflare/Backpack)       │
│  - Bloom filter cache (skip tested candidates)           │
│  - Multi-tier dispatcher (local → optional cloud)        │
│  - Async work queue, resume-from-checkpoint              │
└────────────┬─────────────────────────────────────────────┘
             │
┌────────────▼─────────────────────────────────────────────┐
│  Layer 5: Verification & Wallet Restoration              │
│  - MPC threshold ECDSA (Gennaro-Goldfeder 2018)          │
│  - TEE attestation (Intel SGX / Apple Secure Enclave)    │
│  - Smart contract escrow (atomic success-fee)            │
│  - Onchain proof-of-recovery log                         │
└────────────┬─────────────────────────────────────────────┘
             │
┌────────────▼─────────────────────────────────────────────┐
│  Layer 6: Federated Learning Loop (moat)                 │
│  - Differential Privacy (Dwork 2006)                     │
│  - Federated Averaging (McMahan 2017)                    │
│  - Secure Aggregation                                    │
│  - Anonymous case telemetry → model improves weekly      │
└──────────────────────────────────────────────────────────┘
```

### 5.1 Layer details

#### Layer 1: Cognitive Excavation Engine
**Purpose:** Extract the richest possible set of hints from the user's memory.

**Flow:**
1. Onboarding: wallet kind, creation period, device
2. Structured interview (30–90 minutes):
   - Free recall (Fisher–Geiselman)
   - Context reinstatement (where the wallet was created, what mood)
   - Reverse-order recall
   - Change-perspective recall
3. RL policy picks highest-expected-information questions (Bayesian Optimal Experimental Design — Lindley 1956)
4. Multi-agent debate: three LLM agents cross-question the user's answers, surface contradictions

**Output:** ranked memory-hint list with per-hint confidence scores.

#### Layer 2: Digital Forensic Excavator
**Purpose:** Extract seed/passphrase candidates from the user's local machine and any approved backups.

**Modules:**
- **File carving:** Foremost / Scalpel byte-pattern matching for deleted seed/wallet recovery
- **OCR:** Tesseract + Llama Vision for handwritten seed photos
- **Browser:** Chrome / Firefox cache + history + autofill (BrowsingHistoryView, Hindsight)
- **Password managers:** KeePass / LastPass / 1Password export parsing
- **Email:** Gmail Takeout / iCloud Mail / IMAP backups (embedding search for seed-shaped strings)
- **Photos:** EXIF + perceptual hash for deleted seed-photo thumbnails
- **Backups:** iCloud / Google Drive / Dropbox local download + parse (with explicit user consent)

**Output:** candidate seed fragments, password fragments, historical usage patterns.

#### Layer 3: Constraint Propagation & Inference
**Purpose:** Combine Layer 1 + Layer 2 outputs into a ranked candidate list.

**Algorithms:**
- CSP (AIMA Chapter 6) — arc consistency, AC-3, conflict-directed backjumping
- Bayesian Networks (Pearl 1988) — fragments of evidence → candidate probabilities
- HMM — model the user's pattern-generation habits
- MCMC — sample low-probability but possible candidates
- Damerau-Levenshtein — typo prediction
- PassGPT-style transformer (Rando 2023) — per-user fine-tune on the user's pattern history

**Output:** priority queue of ranked candidates.

#### Layer 4: Distributed Cracking Engine
**Purpose:** Actually test the candidates.

**Components:**
- hashcat wrapper (modes 11300/12700/15700/18800)
- seedcat (BIP-39 GPU brute)
- Custom CUDA kernels: Phantom / Solflare / Backpack local storage formats
- Bloom-filter cache: skip already-tested candidates
- Multi-tier dispatcher:
  - High-probability candidates → user's local GPU
  - Low-probability candidates → user-approved cloud GPU
- Async work queue, fail-over, resume-from-checkpoint

**Output:** the working seed/password.

#### Layer 5: Verification & Wallet Restoration
**Purpose:** When the wallet opens, deliver atomically and securely.

**Mechanisms:**
- MPC threshold ECDSA (Gennaro-Goldfeder 2018) — split-key reveal; Phoenix never sees the key
- TEE attestation — cracking node attests output was not leaked
- Smart-contract escrow:
  - Free / Pro tier: paid out of subscription, recovery is free
  - Success-fee tier: 5% atomic (escrow + reveal in the same transaction)
- Onchain proof-of-recovery log → for anonymized marketing case studies

#### Layer 6: Federated Learning Loop (moat)
**Purpose:** Learn from every successful recovery; improve the model weekly.

**Mechanisms:**
- Differential Privacy (Dwork 2006) — learn without leaking user data
- Federated Averaging (McMahan et al. 2017)
- Secure Aggregation (Bonawitz et al. 2017)
- Anonymous telemetry: which interview patterns led to successful recovery?

**Why it's a moat:** Recovery shops do not share data. Phoenix's interview policy improves week-over-week; competitors restart from zero. Compounding moat.

---

## 6. Algorithm and literature reference

| Area | Reference |
|---|---|
| Cognitive interview | Fisher & Geiselman 1992 |
| Bayesian experimental design | Lindley 1956, Chaloner 1995 |
| CSP & search | Russell & Norvig (AIMA, 4th ed.) |
| Bayesian networks | Pearl 1988, Koller & Friedman 2009 |
| MCMC | Metropolis 1953, Hastings 1970 |
| Differential privacy | Dwork 2006, Abadi et al. 2016 |
| Federated learning | McMahan et al. 2017 |
| Secure aggregation | Bonawitz et al. 2017 |
| Threshold ECDSA | Gennaro-Goldfeder 2018, Lindell 2017 |
| Side-channel resistance | Kocher 1996 |
| File carving | Garfinkel 2007 |
| LLM password modeling | PassGPT (Rando 2023) |
| RL for interview policy | Sutton & Barto 2018 |
| Multi-agent LLM debate | Du et al. 2023 |
| File-system forensics | Carrier 2005 |

---

## 7. Trust & integrity model

### 7.1 Local-only execution proof
- All core algorithms run inside the Tauri sandbox
- Network isolation: during cracking the only outbound traffic is the signed update server + optional cloud GPU (with explicit user consent)
- Wireshark-grade audit log → user can review every byte
- Open-source: public GitHub, deterministic / reproducible builds

### 7.2 Trust mechanisms (rollout)
- **Day 1:** Code public, README + threat model
- **Day 30:** Trail of Bits engagement opened
- **Day 60:** Sigstore + Cosign signed binaries
- **Day 90:** Apple notarization + Microsoft signed
- **Day 120:** Independent reviewer program (top 10 crypto YouTubers get free copies + audit access)
- **Day 180:** Trail of Bits audit report published

### 7.3 Stolen-wallet filter
- Before recovery starts, target address → Chainalysis Reactor + TRM Labs API check
- If flagged stolen / sanctioned, Phoenix declines service (clear UX message)
- Ownership attestation: KYC doc + creation-date proof + 2 secondary references (success-fee tier)

### 7.4 Anti-scam-genre marketing
- Marketing copy avoids "AI"
- No success-rate claim above 70%
- The word "guaranteed" never appears
- Independent reviewer links on every page

---

## 8. Legal & compliance

### 8.1 Jurisdictions
- Primary registration: Switzerland or Estonia (low-friction crypto-recovery legal framework)
- US ops: no Money Transmitter License needed (no custody)
- EU: GDPR-compliant, local-only data processing

### 8.2 AML / KYC
- Free tier: no KYC, used as a local tool
- Pro tier ($99/mo): optional KYC
- Success-fee tier: KYC required + ownership attestation
- $5K+ recovered transactions: OFAC sanction-list check

### 8.3 Disclaimers
- "Phoenix is a tool, not a service. Outcomes depend on user-provided information."
- "We do not guarantee recovery."
- "By using Phoenix, you attest you are the rightful owner of the wallet under recovery."

---

## 9. Pricing

| Tier | Pricing | Target |
|---|---|---|
| **Free** | First 1M brute attempts + 30-min LLM interview | Tire-kickers, validation |
| **Pro** | $99/mo unlimited brute + extended interview + cloud GPU offload | Active recoverers |
| **Recovery success fee** | 5% (atomic via smart contract) | Successful cases |
| **Enterprise API** | $5k–$50k/year | Custodian / exchange dormant-account programs |
| **Premium human consult** | $500/hr | Complex cases, optional |

### 9.1 Revenue projection
- **Year 1:** 10 successful recoveries × $50k average = $25k success fee + $50k Pro sub = $75k
- **Year 2:** 100 recoveries × $30k = $150k success + $300k Pro = $450k
- **Year 3:** 500 recoveries × $25k = $625k success + $1M Pro + first Enterprise = $2M+
- **Year 4-5:** $5–15M ARR horizon

---

## 10. Naming & positioning

### 10.1 Naming
- Internal codename: **Phoenix**
- Public name to be decided pre-launch with the following criteria:
  - No "AI"
  - Forensic / archaeology imagery
  - .com / .io domain available
  - Trademark clean
  - Not used elsewhere in crypto

**Candidate names (workshop):** Lazarus, Excavate, Vault Hunter, Echo, Crypt, Phoenix, Reclaim, Foundling, Resurgo

### 10.2 Positioning
- ❌ "AI wallet recovery" (scam genre)
- ✅ "Open-source forensic recovery assistant for civilians"
- ✅ "BTCRecover with a brain"
- ✅ "The world's first systematized recovery platform"

### 10.3 Marketing channels
- Reddit lost-wallet threads — **service** them (provide value, no hard sell)
- YouTube case studies (Joe Grand model, but systematic)
- Crypto Twitter (BTCRecover, Bankr, Polymarket post-mortems)
- SEO: "I lost my wallet" long-tail
- Open-source community (HN, ProductHunt launch)

---

## 11. Competitive differentiation matrix

| Capability | WRS | KeychainX | BTCRecover | Phoenix |
|---|---|---|---|---|
| Productized | ❌ | ❌ | Partial | ✅ |
| AI cognitive interview | ❌ | ❌ | ❌ | ✅ |
| Digital exhaust parser | Partial (manual) | Partial | ❌ | ✅ |
| Local-only | ❌ (user ships file) | ❌ | ✅ | ✅ |
| Open-source | ❌ | ❌ | ✅ | ✅ |
| Federated learning | ❌ | ❌ | ❌ | ✅ |
| Atomic success fee | ❌ | ❌ | N/A | ✅ |
| GUI | ❌ | ❌ | ⚠️ (poor) | ✅ |
| Multilingual | ❌ | ❌ | ❌ | ✅ (planned) |

---

## 12. 8-week MVP scope

### Week 1 — Foundation
- Tauri + React + Rust workspace
- Llama 3.3 70B integration via Ollama
- Logging + telemetry framework (opt-in only)
- CI/CD baseline (GitHub Actions, signed builds)

### Week 2 — Cognitive Interview MVP
- 50 structured interview questions (manually tuned)
- LLM-as-interviewer with policy module
- User memory state representation (graph + embeddings)
- Output: ranked candidate text list

### Week 3 — Forensic Layer A
- Browser forensics (Chrome / Firefox cache + history + autofill)
- Password manager dump parser (KeePass, 1Password export)
- First photo OCR (Tesseract baseline)

### Week 4 — Forensic Layer B
- Email backup mining (Gmail Takeout, IMAP)
- iCloud / Google Drive local download + parse
- File carving (Foremost binding)

### Week 5 — Inference + Cracking
- Bayesian candidate ranker
- hashcat + seedcat wrapper
- First custom CUDA kernel (Phantom)

### Week 6 — Validation push
- Manual outreach: BTCRecover forum, r/Bitcoin lost-wallet threads, KeychainX rejects
- Goal: 10 paying pilots ($99 Pro tier)
- Daily user calls

### Week 7 — First recovery
- Refine based on pilot feedback
- Goal: 1 real successful recovery
- Anonymized case study draft

### Week 8 — Launch prep
- Trail of Bits engagement opened
- GitHub public open
- ProductHunt + HN draft
- Marketing site (Section 10 positioning)

---

## 13. Success criteria & validation gates

### 13.1 v1 GATE (end of Week 8)

**Continue:**
- ≥1 real recovery (user-verified, anonymized case study published)
- ≥10 paying pilots ($99 Pro)
- Trail of Bits engagement opened
- NPS >40 from pilot users

**Kill (if none of the above):**
- 0 recoveries
- ≤3 paying pilots
- Audit firm declined or did not engage

### 13.2 v2 GATE (end of Year 1)
- ≥10 successful recoveries
- $200k revenue
- Federated learning loop active
- Open-source community ≥500 GitHub stars

### 13.3 v3 GATE (end of Year 2)
- ≥100 successful recoveries
- $1M revenue
- First enterprise customer (Coinbase / Binance / Kraken / Crypto.com dormant program)

---

## 14. Open risks & questions

### 14.1 High-impact risks

1. **Naming-category poison** — "AI wallet recovery" Google results are scam-dominated; open-source + audit may not be enough. Mitigation: marketing copy never uses "AI"; only "forensic" / "guided." Independent reviewers from Day 1.
2. **AML legal exposure** — if a user accesses a stolen wallet via Phoenix, legal liability. Mitigation: Chainalysis / TRM check + ownership attestation + KYC at success-fee tier.
3. **Apple / Google app store rejection** — mitigation: desktop-only initially, web fallback, mobile only handles interview + report (cracking stays on desktop).
4. **Federated-learning privacy attack** — mitigation: differential privacy + secure aggregation + threat-model audit.

### 14.2 Medium-impact risks

5. Hashcat / seedcat upstream breakage → CI matrix tests, version pinning
6. LLM hallucination during cognitive interview → multi-agent debate cross-check, human-in-loop
7. iCloud / Google Drive API changes → fallback parsers, format detection

### 14.3 Open questions (to be resolved)

- v1 jurisdiction: Switzerland or Estonia? **Action: 30-min legal counsel call (Week 1)**
- Naming workshop: branding agency $5–15k vs DIY?
- How to recruit the first 10 pilot users? Outreach playbook detail
- Trail of Bits engagement cost: $50–200k expected; financing plan?
- Ollama vs llama.cpp vs MLX (Apple Silicon) performance comparison

---

## 15. Roadmap beyond v1

### Year 1 Q3-Q4
- Mobile app (iOS / Android) — interview + report only (cracking stays on desktop)
- Hardware-wallet seed extraction (software-only; not competing with Praefortis)
- Multilingual interview (Turkish, German, Spanish, Mandarin)

### Year 2
- Enterprise SaaS: exchange / custodian dormant-account API
- Heir / inheritance specific UX (estate executor partnership)
- Insurance partnership: pre-protection insurance + recovery rider

### Year 3
- Phoenix Foundation (open-source governance)
- Recovery network: federated GPU compute pool
- Token (optional) — Pro tier holders get premium access

---

## Appendix A: Design decisions & rationale

| Decision | Rationale |
|---|---|
| Tauri vs Electron | Bundle size, performance, Rust safety |
| Llama 3.3 vs GPT-5 API | Local-only principle; user data does not leave the machine |
| hashcat wrapper vs custom | Decade-tested, low regression risk |
| MPC threshold ECDSA vs trust | Phoenix never sees the key = trust moat |
| Open-source MIT vs proprietary | Mandatory to differentiate from the scam genre |
| 5% success fee vs 15% artisan | Lower fee × productized scale = higher absolute revenue |
| Federated learning vs centralized | Privacy + moat (competitors restart from zero) |

---

## Appendix B: Threat model (outline)

### Adversary models
1. **Passive observer:** monitors network traffic. Mitigation: local-only execution, network isolation.
2. **Malicious cloud GPU:** seed exfiltration on rented cloud. Mitigation: TEE attestation, only encrypted candidate testing.
3. **Stolen-wallet user:** uses Phoenix to open a wallet they don't own. Mitigation: Chainalysis check + KYC + attestation.
4. **Malicious Phoenix maintainer:** backdoor injection. Mitigation: open-source + reproducible builds + community audit.
5. **Federated learning poisoning:** adversary feeds fake telemetry to corrupt the model. Mitigation: secure aggregation + outlier detection.

Detailed threat model: written in Week 4 before security-architecture review.

---

## Appendix C: First 10 pilot outreach playbook (to be drafted before Week 6)

---

## End of spec

This document is committed as the **Phoenix v1 design specification** under the brainstorming skill. The implementation plan is maintained in a separate document under the writing-plans skill.
