# Phoenix Threat Model — v0 (Week 1)

This is the initial scaffold. A full threat model (with adversary trees, mitigations, and security architecture review) is scheduled for Week 4. This v0 document captures the assumptions baked into the Week 1 foundation so future work can refer back to them.

## Trust boundaries

1. **User machine ↔ Phoenix process:** Phoenix runs as a desktop process owned by the user. No elevation required. No background daemon.
2. **Phoenix ↔ Ollama:** HTTP loopback. The user is responsible for trusting their local Ollama installation.
3. **Phoenix ↔ Internet:** Only outbound, only to:
   - GitHub releases (signed update manifest)
   - Optional opt-in telemetry endpoint (anonymous events, never wallet content)
   - Optional cloud GPU offload endpoint (only if user enables Pro tier with cloud cracking)

## Adversaries (initial set)

| Adversary | Capability | Phoenix v0 mitigation |
|---|---|---|
| Passive network observer | Reads all unencrypted traffic | All outbound HTTPS, no telemetry by default |
| Malicious Ollama image | User pulls a poisoned model | Out of scope for Phoenix; documented in README |
| Compromised dev dependency | Supply-chain attack via cargo / npm | Lockfiles committed; `cargo audit` planned for Week 2 |
| Phoenix maintainer inserts backdoor | Upstream compromise | Open-source from Day 1; deterministic / reproducible builds in Week 8 |
| User's own malware | Reads phoenix process memory | Out of scope (host compromise = game over) |
| Stolen-wallet user | Tries to use Phoenix on a wallet they don't own | Chainalysis/TRM Labs check + ownership attestation gate at success-fee tier (Week 5) |

## Non-secrets handled in Week 1

- Configuration (telemetry on/off, ollama endpoint, model id)
- Application logs (timestamps, log levels, no user content)
- Telemetry events (enum tags only, no payloads)

## Secrets NOT yet handled in Week 1

- Wallet seeds (Layer 2-5 work in Weeks 2-5)
- Private keys (Layer 5)
- Recovery candidate lists (Layer 3)
- User cognitive interview transcripts (Layer 1; sensitive even though no seed)

These will get their own threat model entries as the corresponding layers ship.

## Local-only invariant (Week 1 commitment)

> **"Nothing sensitive leaves the user's machine without an explicit, named user action."**

Concretely:
- No telemetry until the user toggles `[telemetry] enabled = true`
- No cloud GPU offload until the user opts into Pro tier with explicit cloud cracking
- No background HTTP traffic at all in the foundation build

This invariant is enforced today by the absence of any HTTP client invocation outside the explicit Ollama generate call. Layer 2 (forensic excavation) will add file-system access; that addition triggers a threat-model revision before merging.
