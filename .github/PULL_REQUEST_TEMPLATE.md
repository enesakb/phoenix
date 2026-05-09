## Summary

<!-- One paragraph: what changes, why, and any non-obvious decisions. -->

## Type of change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that changes existing behavior)
- [ ] Documentation update
- [ ] Refactor / chore

## Test plan

<!-- Tell the reviewer how to verify this works. -->

- [ ] `cargo test --workspace` green
- [ ] `cargo test -p phoenix-tauri` green (if Tauri-side changes)
- [ ] `cd src-ui && npm run test` green (if UI changes)
- [ ] `cargo clippy --workspace -- -D warnings` clean
- [ ] `cargo fmt --all -- --check` clean
- [ ] Manual smoke: `cargo run -p phoenix-cli -- doctor`

## Invariant checklist

- [ ] No new outbound network calls in cracking pipeline (local-only preserved)
- [ ] No personal data, wallet addresses, or seed-shaped strings in commits or test fixtures
- [ ] No closed-source dependency added
- [ ] If cryptographic code changed: standard test vectors (BIP-39, BIP-32, etc.) still pass

## Related issues

<!-- e.g. "Closes #42", "Refs #17" -->
