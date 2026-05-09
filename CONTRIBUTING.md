# Contributing to Phoenix

Thanks for your interest in Phoenix. This document explains how the project is organized, how to set up a development environment, and how to submit changes that have a high chance of being merged.

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating you agree to its terms.

## What we accept

- Bug fixes with regression tests
- New forensic extractors (e.g. KeePass kdbx, 1Password export, Apple Keychain)
- Additional cryptographic chains (Solana ed25519, Cosmos, Polkadot) — must include canonical test vectors
- Performance improvements with benchmarks
- Documentation, typos, README polish
- Translations of the cognitive interview question bank

## What we do NOT accept

- Code that performs network calls inside the cracking pipeline. The local-only invariant is non-negotiable.
- Closed-source dependencies.
- Telemetry that ships anything other than enum tags.
- Branding changes that include "AI" in the user-facing copy.
- Recovery features for hardware-glitch attacks (out of scope; Praefortis / Unciphered handle those).

## Development environment

```bash
git clone https://github.com/enesakb/phoenix
cd phoenix
cargo build --workspace
cd src-ui && npm install && cd ..
```

### Running the test suite

```bash
cargo test --workspace             # phoenix-core + phoenix-cli
cd src-tauri && cargo test         # phoenix-tauri shell
cd src-ui && npm run test          # Vitest
```

All three suites must be green before you open a pull request.

### Running the desktop app locally

```bash
cd src-tauri
cargo tauri dev
```

Requires the [Tauri 2 prerequisites](https://tauri.app/start/prerequisites/) for your platform.

### Running the cognitive interview locally

You need an Ollama install with at least one model pulled:

```bash
ollama pull qwen3:14b   # ~9 GB
ollama serve
```

Then in another terminal:

```bash
cargo run -p phoenix-cli -- ollama-check --model qwen3:14b
```

## Style

- Rust: rustfmt default settings (`cargo fmt --all`); clippy clean (`cargo clippy --workspace -- -D warnings`)
- TypeScript: ESLint clean (`npm run lint` in `src-ui/`)
- Commit messages: Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`, `test:`)

## Pull request process

1. Fork the repo, branch from `master`
2. Make your change with tests
3. Run the full test suite locally — all green
4. Update relevant docs (`README.md`, `docs/`, this file if needed)
5. Open a PR with a clear description and link any related issue
6. CI must pass before review

## Security

If you find a security vulnerability, **do not open a public issue**. Read [`SECURITY.md`](SECURITY.md) for the disclosure process.

## License

By submitting a PR you agree that your contribution is licensed under MIT, the same as the rest of the project.
