# Phoenix Week 2 — Cognitive Interview MVP (Layer 1)

**Goal:** Build the cognitive excavation engine that interviews users to extract memory hints about their lost wallets, producing a structured candidate list ready for Layer 2 (forensic) and Layer 3 (inference) consumption.

**Architecture:** A `phoenix-core::interview` module containing: a JSON-backed Question bank, a session state machine, an LLM-driven interviewer that adapts follow-ups based on prior answers, a 3-agent debate cross-questioning loop (Du et al. 2023), and a candidate extractor that turns memory state into ranked seed/passphrase fragments. Persistence is JSON-on-disk; SQLite deferred to Week 4. Tauri commands expose the engine to the React UI which provides the interview flow.

**Tech Stack:** Rust (chrono, uuid, sqlx-deferred), serde_json file-backed sessions, Ollama LLM via existing client, React UI, Vitest.

**Scope cuts (deliberate YAGNI):**
- No SQLite — JSON file per session
- No RL policy — questions ordered by Fisher-Geiselman protocol; adaptive follow-ups via LLM only
- No memory graph viz — flat list view
- No federated learning telemetry yet (Layer 6, Week 8+)
