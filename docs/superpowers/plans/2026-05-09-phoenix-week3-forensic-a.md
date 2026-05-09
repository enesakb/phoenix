# Phoenix Week 3 — Forensic Layer A (Digital Exhaust Excavator)

**Goal:** Add the first wave of concrete forensic extractors that turn local files (browser data, password manager exports, plain text) into memory nodes feeding the same candidate pipeline as the cognitive interview.

**Architecture:** A `phoenix-core::forensic` module with an `Extractor` trait and a registry-style dispatcher that maps file extensions to extractors. Three concrete extractors ship in this week:

1. `Bip39TextExtractor` — universal text scan for BIP-39 words. Highest-impact, lowest-effort: works on any text file and finds partial / scrambled seed phrases.
2. `ChromeHistoryExtractor` — SQLite read of a user-supplied `History` file. Pulls visits to wallet/exchange domains and any autofill text matching BIP-39 patterns.
3. `BitwardenCsvExtractor` — CSV parser for the most common password-manager export. Pulls passwords + URIs and feeds them as `PasswordPattern` nodes.

**Deliberate scope cuts:**
- No OCR (Tesseract bindings require native libs; defer to Week 4)
- No KeePass kdbx parser (separate native dep; Week 4)
- No automatic OS-level discovery — user explicitly imports files (avoids locking + permission complexity; OS-walk in Week 5+)
- No iCloud / Drive backup parsing (Week 4)
- No file carving (Week 4)

**Tech stack:** rusqlite (bundled SQLite for portability), csv crate, BIP-39 wordlist (embedded).
