# Phoenix launch playbook

Sequential checklist to get Phoenix from "code on Enes's laptop" to "real users finding it organically." All steps are **$0** unless explicitly noted.

---

## Day 0 — pre-launch sanity (you already did this)

- [x] All 60 tests green
- [x] CLI demo works (72ms recovery on test vector)
- [x] Tauri desktop app launches
- [x] README polished with badges + demo
- [x] Public landing site built (`site/index.html`)

## Day 1 — push public

- [ ] **Push to GitHub** — see [`push-to-github.md`](push-to-github.md)
  - 3 minutes, $0
  - Result: `github.com/enesakb/phoenix`
- [ ] **Deploy landing to Cloudflare Pages** — see [`deploy-cloudflare.md`](deploy-cloudflare.md)
  - 5 minutes, $0
  - Result: `phoenix-recovery.pages.dev`
- [ ] **Update README** with live landing URL
- [ ] **Create GitHub Releases** for `week-5-crypto-reconstruct` and `week-4-6-extended`
- [ ] **Add repo topics**: `wallet-recovery bip39 bitcoin ethereum rust tauri forensics`

## Day 2 — Reddit warm-up

- [ ] Read 5 threads on r/Bitcoin and r/Ethereum about lost wallets — DO NOT post yet, just observe
- [ ] Identify 3 threads where Phoenix would actually help → bookmark for Day 3

## Day 3 — Reddit posts

See [`reddit-posts.md`](reddit-posts.md) for templates.

- [ ] Post **Show CC** thread to r/CryptoCurrency
- [ ] Comment helpfully on 1 lost-wallet thread (use Type-1 template, partial-info case)
- [ ] Comment honestly on 1 lost-wallet thread (use Type-1 template, full-info-loss case — the goodwill from saying "your case is unrecoverable, don't pay anyone" pays back 10x)

## Day 4 — Twitter / X thread

See [`twitter-thread.md`](twitter-thread.md).

- [ ] Schedule master thread for 14:00 UTC
- [ ] Post reply-pinned scam-defense tweet 24h later
- [ ] Engage with replies, quote-tweets

## Day 5 — Hacker News

See [`hn-show-post.md`](hn-show-post.md).

- [ ] Submit Show HN at 09:30 ET (14:30 UTC) Tuesday or Wednesday
- [ ] Have first-comment defenses ready (BTCRecover comparison, "is this a scam", etc.)
- [ ] Reply to every top-level comment within 30 minutes for the first 3 hours

## Day 6-7 — engagement push

- [ ] Reply to all GitHub issues and discussions
- [ ] Reply to all DMs and emails (no scammers, no stolen-wallet cases)
- [ ] Comment on 5 more Reddit threads with the Type-1 helpful template
- [ ] Post r/programming Show-thread (different audience than r/CryptoCurrency)

## Week 2 — first pilot

If outreach worked, you should have 10-50 inquiries by end of week 1. Filter:

- ❌ Stolen wallets (Chainalysis check)
- ❌ Full-info-loss (be honest, save them money)
- ❌ Hardware wallet PIN (route to Praefortis / KeychainX)
- ✅ Partial info, ownership clear, target address known → run free pilot

Goal: 1 successful real-world recovery by end of week 2.

## Week 3-4 — case study

After first successful recovery:

- [ ] Get user's permission for an anonymized case study
- [ ] Record short video showing the actual UI flow (no PII)
- [ ] Post to:
  - Twitter/X follow-up tweet
  - r/CryptoCurrency follow-up post
  - GitHub README "Featured recovery"
  - HN follow-up "Show HN: First successful recovery using open-source tool"

Each successful real-world case is worth ~10x more outreach than any marketing copy.

---

## Metrics to track

Set up a simple Google Sheet with:

| Metric | Day 1 | Day 7 | Day 30 |
|---|---|---|---|
| GitHub stars | | | |
| GitHub forks | | | |
| Site visits (Cloudflare analytics) | | | |
| Email inquiries | | | |
| Pilot applications | | | |
| Successful recoveries | | | |
| Failed but charged-no-fee | | | |
| Stolen-wallet rejections | | | |

Targets for Week 1: ≥50 stars, ≥1000 visits, ≥5 inquiries.

If you're under target on Day 7, the issue is **either positioning** (rewrite the landing copy) **or distribution** (post to one more crypto subreddit). Almost never the product itself.

---

## Anti-patterns (do NOT do at any point)

- ❌ Pay for ads (Google Ads literally bans wallet-recovery; the existing ones are scams)
- ❌ DM strangers offering services (a few people will report; account dies)
- ❌ Hire a marketing agency before you have 10 paying users
- ❌ Run paid Twitter promotions
- ❌ Buy the domain `airecovery.io` or anything else with "AI" in it
- ❌ Promise success rates above 50%
- ❌ Take money before recovery succeeds (that's the scam playbook)
- ❌ Service stolen-wallet cases even at premium prices

---

## When to revisit Trail of Bits audit

Trigger: $5k+ MRR for 2 consecutive months, OR an enterprise customer asks for it. Cost: $50-200k. Bookmark `trailofbits.com/contact` for that day. Until then, your reproducible builds + open-source + community review serve the same trust function at $0.

---

## Closing the loop

Every week, take 30 minutes and ask:

1. Are real users using Phoenix? (count: GitHub stars + email inquiries)
2. Are they recovering wallets? (count: successful pilots)
3. Are they paying? (count: success-fee transactions)

If the answer to (1) is yes but (2) and (3) are no, the product needs another sprint of work — most likely on Layer 3 (Bayesian inference) which makes hard cases tractable.

If the answer to (1) is no, the issue is distribution. Pick one new channel and post.

If you're a year in and (1)(2)(3) are all yes — congratulations, you're now in the recovery industry. Time to consider the audit and a real legal entity.
