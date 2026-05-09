# Phoenix Week 4 + 6 — Extended recovery + forensic surface

**Goal:** Push everything that can be done autonomously. After this commit Phoenix supports:
1. Multi-word brute force (1 or 2 missing positions)
2. BIP-39 passphrase brute force (the forgotten "25th word")
3. Hashcat command builder (offload heavy crypt to GPU clusters when needed)
4. mbox email scanner (Gmail Takeout / IMAP backups)
5. Threat model v1 — covers all five new attack surfaces

**Deliberate residual scope cuts (need external resources, not code):**
- KeePass kdbx parsing — requires interactive master password; UX work in a separate sprint
- OCR — Tesseract native libs; deferred until pilot users actually need it
- Solana ed25519 derivation — no canonical test vector handy; defer until verifiable
- Hashcat *runner* — only the command builder, not subprocess orchestration; user runs hashcat themselves

What still cannot be autonomous:
- Real pilot users (Hafta 7)
- Trail of Bits engagement (Hafta 8)
- GitHub remote + ProductHunt launch (Hafta 8)
