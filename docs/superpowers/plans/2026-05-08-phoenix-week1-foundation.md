# Phoenix Week 1 — Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Stand up a Tauri + Rust + React desktop application skeleton with logging, configuration, opt-in telemetry, CI/CD, and a verified Ollama / Llama 3.3 70B local LLM connection — committed in atomic, test-driven steps.

**Architecture:** Rust workspace with two crates — `phoenix-core` (domain logic, exposed via Tauri commands) and `phoenix-cli` (developer ergonomics). React + TypeScript frontend in `src-ui/`. Tauri 2.x as the desktop shell. Ollama runs as a sidecar process the user installs separately. Frontend talks to backend via Tauri IPC commands; backend talks to Ollama via HTTP. Tests use Rust's built-in `#[test]` framework + Vitest for React. CI runs on GitHub Actions (Linux + Windows + macOS matrix).

**Tech Stack:** Tauri 2.x, Rust 1.83+, React 18, TypeScript 5.x, Vite, Vitest, Ollama (Llama 3.3 70B), GitHub Actions, tracing (Rust logging), serde + toml (config).

**Repo path:** `C:\Users\enesa\phoenix`

**Subsequent plans:** Weeks 2-8 each get their own plan, written after this week's gate passes. Week 2 = Cognitive Interview MVP. Week 3 = Forensic Layer A (browser/password mgr/OCR). Week 4 = Forensic Layer B (email/cloud backup/file carving). Week 5 = Inference + Cracking. Weeks 6-8 = Validation + First Recovery + Launch Prep.

---

## File Structure

After Week 1 the repo will look like:

```
phoenix/
├── .github/
│   └── workflows/
│       ├── ci.yml                    # build + test on push
│       └── lint.yml                  # clippy + eslint
├── crates/
│   ├── phoenix-core/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs                # public API
│   │   │   ├── config.rs             # configuration loader
│   │   │   ├── logging.rs            # tracing setup
│   │   │   ├── telemetry.rs          # opt-in event tracker
│   │   │   └── llm/
│   │   │       ├── mod.rs            # LLM client trait
│   │   │       └── ollama.rs         # Ollama HTTP client
│   │   └── tests/
│   │       ├── config_test.rs
│   │       ├── telemetry_test.rs
│   │       └── ollama_test.rs        # integration test (requires Ollama)
│   └── phoenix-cli/
│       ├── Cargo.toml
│       └── src/main.rs               # `phoenix doctor` etc.
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   └── src/
│       ├── main.rs                   # Tauri entry point
│       └── commands.rs               # IPC commands
├── src-ui/
│   ├── package.json
│   ├── vite.config.ts
│   ├── tsconfig.json
│   ├── index.html
│   └── src/
│       ├── main.tsx
│       ├── App.tsx
│       ├── routes/
│       │   ├── Welcome.tsx
│       │   └── Settings.tsx
│       └── lib/
│           └── ipc.ts                # typed wrappers for invoke()
├── docs/
│   ├── superpowers/
│   │   ├── specs/2026-05-08-phoenix-design.md
│   │   └── plans/2026-05-08-phoenix-week1-foundation.md
│   └── threat-model-v0.md            # initial threat model
├── Cargo.toml                        # workspace root
├── .gitignore
├── LICENSE                           # MIT
└── README.md
```

Each Rust crate has a single responsibility. Each React component file is one screen. Tests live next to or in parallel with the code they exercise.

---

## Task 1: Initialize repository skeleton

**Files:**
- Create: `C:\Users\enesa\phoenix\.gitignore`
- Create: `C:\Users\enesa\phoenix\README.md`
- Create: `C:\Users\enesa\phoenix\LICENSE`

- [ ] **Step 1: Initialize git**

```bash
cd /c/Users/enesa/phoenix
git init
git config user.name "Phoenix Maintainer"
git config user.email "noreply@phoenix.local"
```

Expected: `Initialized empty Git repository in C:/Users/enesa/phoenix/.git/`

- [ ] **Step 2: Create .gitignore**

Write `C:\Users\enesa\phoenix\.gitignore`:

```gitignore
# Rust
target/
**/*.rs.bk
Cargo.lock.bak

# Node
node_modules/
dist/
.vite/
*.log

# Tauri
src-tauri/target/
src-tauri/gen/

# IDE
.vscode/
.idea/
*.swp
.DS_Store

# Phoenix runtime
phoenix-config.toml
phoenix.log
phoenix-data/
*.recovery
```

- [ ] **Step 3: Create LICENSE (MIT)**

Write `C:\Users\enesa\phoenix\LICENSE`:

```
MIT License

Copyright (c) 2026 Phoenix Maintainer

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

- [ ] **Step 4: Create initial README.md**

Write `C:\Users\enesa\phoenix\README.md`:

```markdown
# Phoenix

> Open-source forensic recovery assistant for partial-information lost crypto wallets.

**Status:** Pre-alpha (Week 1 — foundation). Not ready for use.

## What this is

Phoenix systematizes what artisan wallet-recovery shops do manually: a structured cognitive interview, deep digital exhaust forensics, Bayesian candidate ranking, and distributed cracking — orchestrated as a single open-source desktop application.

## What this is NOT

- Not a recovery promise. Wallets with zero memory and zero digital traces cannot be recovered. Period.
- Not a hardware-glitch tool. Hardware PIN attacks are out of scope.
- Not a cloud service. All recovery work happens locally on your machine.
- Not "AI wallet recovery." Phoenix is forensic and guided, not magic.

## Realistic outcomes

For wallets where partial information exists (forgotten 1-2 seed words, wallet.dat with remembered password pattern, lost backups with traceable digital exhaust): expected v1 recovery rate **35-50%**.

For wallets with no recoverable signal: **0%**. Always.

## Design

See [`docs/superpowers/specs/2026-05-08-phoenix-design.md`](docs/superpowers/specs/2026-05-08-phoenix-design.md).

## License

MIT. See [`LICENSE`](LICENSE).
```

- [ ] **Step 5: Stage and commit**

```bash
git add .gitignore README.md LICENSE
git commit -m "chore: initialize Phoenix repository"
```

Expected: one commit with three files.

---

## Task 2: Create Rust workspace + phoenix-core crate

**Files:**
- Create: `C:\Users\enesa\phoenix\Cargo.toml` (workspace root)
- Create: `C:\Users\enesa\phoenix\crates\phoenix-core\Cargo.toml`
- Create: `C:\Users\enesa\phoenix\crates\phoenix-core\src\lib.rs`
- Create: `C:\Users\enesa\phoenix\crates\phoenix-core\tests\smoke_test.rs`

- [ ] **Step 1: Write workspace root Cargo.toml**

Write `C:\Users\enesa\phoenix\Cargo.toml`:

```toml
[workspace]
resolver = "2"
members = ["crates/phoenix-core", "crates/phoenix-cli", "src-tauri"]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Phoenix Maintainer <noreply@phoenix.local>"]
license = "MIT"
repository = "https://github.com/enesakb/phoenix"
rust-version = "1.83"

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
anyhow = "1.0"
thiserror = "2.0"
reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false }
tokio = { version = "1", features = ["full"] }
```

- [ ] **Step 2: Write phoenix-core Cargo.toml**

Write `C:\Users\enesa\phoenix\crates\phoenix-core\Cargo.toml`:

```toml
[package]
name = "phoenix-core"
version.workspace = true
edition.workspace = true
license.workspace = true
authors.workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
toml.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
anyhow.workspace = true
thiserror.workspace = true
reqwest.workspace = true
tokio.workspace = true
```

- [ ] **Step 3: Write minimal lib.rs**

Write `C:\Users\enesa\phoenix\crates\phoenix-core\src\lib.rs`:

```rust
//! Phoenix core domain logic.
//!
//! All recovery primitives live here so they can be used both from the Tauri
//! shell and the CLI. Nothing in this crate touches the network without an
//! explicit user action.

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
```

- [ ] **Step 4: Write the failing smoke test**

Write `C:\Users\enesa\phoenix\crates\phoenix-core\tests\smoke_test.rs`:

```rust
use phoenix_core::version;

#[test]
fn version_matches_cargo_manifest() {
    assert_eq!(version(), "0.1.0");
}
```

- [ ] **Step 5: Run the test**

```bash
cd /c/Users/enesa/phoenix
cargo test -p phoenix-core
```

Expected: 1 passed; 0 failed (this should pass on first run because lib.rs is already correct — the test exists to lock the contract).

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml crates/phoenix-core/
git commit -m "feat: scaffold phoenix-core crate with version smoke test"
```

---

## Task 3: Add phoenix-cli crate with `doctor` command

**Files:**
- Create: `C:\Users\enesa\phoenix\crates\phoenix-cli\Cargo.toml`
- Create: `C:\Users\enesa\phoenix\crates\phoenix-cli\src\main.rs`

- [ ] **Step 1: Write the Cargo.toml**

Write `C:\Users\enesa\phoenix\crates\phoenix-cli\Cargo.toml`:

```toml
[package]
name = "phoenix-cli"
version.workspace = true
edition.workspace = true
license.workspace = true
authors.workspace = true

[[bin]]
name = "phoenix"
path = "src/main.rs"

[dependencies]
phoenix-core = { path = "../phoenix-core" }
clap = { version = "4.5", features = ["derive"] }
anyhow.workspace = true
tokio.workspace = true
```

- [ ] **Step 2: Write the failing doctor test inside main.rs**

Write `C:\Users\enesa\phoenix\crates\phoenix-cli\src\main.rs`:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "phoenix", version, about = "Phoenix recovery CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print version and environment info.
    Doctor,
}

fn doctor_output() -> String {
    format!("phoenix-core version: {}", phoenix_core::version())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Doctor => println!("{}", doctor_output()),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doctor_output_contains_version() {
        let output = doctor_output();
        assert!(output.contains("phoenix-core version: 0.1.0"));
    }
}
```

- [ ] **Step 3: Run the test**

```bash
cargo test -p phoenix-cli
```

Expected: 1 passed.

- [ ] **Step 4: Run the binary**

```bash
cargo run -p phoenix-cli -- doctor
```

Expected: `phoenix-core version: 0.1.0`

- [ ] **Step 5: Commit**

```bash
git add crates/phoenix-cli/
git commit -m "feat: add phoenix-cli with doctor subcommand"
```

---

## Task 4: Configuration loader with TOML

**Files:**
- Create: `C:\Users\enesa\phoenix\crates\phoenix-core\src\config.rs`
- Modify: `C:\Users\enesa\phoenix\crates\phoenix-core\src\lib.rs` (export config module)
- Create: `C:\Users\enesa\phoenix\crates\phoenix-core\tests\config_test.rs`

- [ ] **Step 1: Write the failing integration test**

Write `C:\Users\enesa\phoenix\crates\phoenix-core\tests\config_test.rs`:

```rust
use phoenix_core::config::{Config, TelemetryConfig};
use std::io::Write;

#[test]
fn loads_default_config_when_file_missing() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("phoenix-config.toml");

    let cfg = Config::load_or_default(&path).unwrap();

    assert_eq!(cfg.telemetry.enabled, false);
    assert_eq!(cfg.ollama.endpoint, "http://localhost:11434");
}

#[test]
fn parses_user_config_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("phoenix-config.toml");
    let mut file = std::fs::File::create(&path).unwrap();
    writeln!(
        file,
        r#"
[telemetry]
enabled = true

[ollama]
endpoint = "http://192.168.1.50:11434"
model = "llama3.3:70b"
"#
    )
    .unwrap();

    let cfg = Config::load_or_default(&path).unwrap();

    assert_eq!(cfg.telemetry.enabled, true);
    assert_eq!(cfg.ollama.endpoint, "http://192.168.1.50:11434");
    assert_eq!(cfg.ollama.model, "llama3.3:70b");
}

#[test]
fn rejects_malformed_toml() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("phoenix-config.toml");
    std::fs::write(&path, "this is :: not valid toml ===").unwrap();

    let result = Config::load_or_default(&path);

    assert!(result.is_err());
}

#[test]
fn telemetry_config_default_is_opt_out() {
    let cfg = TelemetryConfig::default();
    assert_eq!(cfg.enabled, false);
}
```

- [ ] **Step 2: Add tempfile to dev-dependencies**

Modify `C:\Users\enesa\phoenix\crates\phoenix-core\Cargo.toml` — append at end:

```toml
[dev-dependencies]
tempfile = "3.10"
```

- [ ] **Step 3: Run the test to verify failure**

```bash
cargo test -p phoenix-core --test config_test
```

Expected: compile error (`Config`, `TelemetryConfig` not found). This is the failing state.

- [ ] **Step 4: Implement config.rs**

Write `C:\Users\enesa\phoenix\crates\phoenix-core\src\config.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse config TOML: {0}")]
    Parse(#[from] toml::de::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub telemetry: TelemetryConfig,
    #[serde(default)]
    pub ollama: OllamaConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    #[serde(default)]
    pub enabled: bool,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self { enabled: false }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    #[serde(default = "default_endpoint")]
    pub endpoint: String,
    #[serde(default = "default_model")]
    pub model: String,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            endpoint: default_endpoint(),
            model: default_model(),
        }
    }
}

fn default_endpoint() -> String {
    "http://localhost:11434".to_string()
}

fn default_model() -> String {
    "llama3.3:70b".to_string()
}

impl Config {
    pub fn load_or_default(path: &Path) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Ok(Config::default());
        }
        let raw = std::fs::read_to_string(path)?;
        let cfg: Config = toml::from_str(&raw)?;
        Ok(cfg)
    }
}
```

- [ ] **Step 5: Export from lib.rs**

Modify `C:\Users\enesa\phoenix\crates\phoenix-core\src\lib.rs` to:

```rust
//! Phoenix core domain logic.

pub mod config;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
```

- [ ] **Step 6: Run tests**

```bash
cargo test -p phoenix-core
```

Expected: 5 passed (1 smoke + 4 config tests).

- [ ] **Step 7: Commit**

```bash
git add crates/phoenix-core/
git commit -m "feat(core): add Config loader with telemetry + ollama defaults"
```

---

## Task 5: Logging via `tracing` with file + stdout sinks

**Files:**
- Create: `C:\Users\enesa\phoenix\crates\phoenix-core\src\logging.rs`
- Modify: `C:\Users\enesa\phoenix\crates\phoenix-core\src\lib.rs`
- Create: `C:\Users\enesa\phoenix\crates\phoenix-core\tests\logging_test.rs`

- [ ] **Step 1: Add tracing-appender dep**

Modify `C:\Users\enesa\phoenix\crates\phoenix-core\Cargo.toml` dependencies — add:

```toml
tracing-appender = "0.2"
```

- [ ] **Step 2: Write failing logging test**

Write `C:\Users\enesa\phoenix\crates\phoenix-core\tests\logging_test.rs`:

```rust
use phoenix_core::logging::init_for_tests;
use tracing::info;

#[test]
fn logging_init_does_not_panic() {
    let _guard = init_for_tests();
    info!("phoenix logging initialized");
}
```

- [ ] **Step 3: Run test (expect compile error)**

```bash
cargo test -p phoenix-core --test logging_test
```

Expected: `phoenix_core::logging::init_for_tests` not found.

- [ ] **Step 4: Implement logging.rs**

Write `C:\Users\enesa\phoenix\crates\phoenix-core\src\logging.rs`:

```rust
use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initialize Phoenix's tracing subscriber.
///
/// Logs to both stdout and `<log_dir>/phoenix.log`. Returns a guard that must
/// remain in scope for the lifetime of the process — dropping it flushes
/// pending log lines.
pub fn init(log_dir: &Path) -> std::io::Result<WorkerGuard> {
    std::fs::create_dir_all(log_dir)?;
    let file_appender = tracing_appender::rolling::daily(log_dir, "phoenix.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let stdout_layer = fmt::layer().with_target(true).with_level(true);
    let file_layer = fmt::layer().with_writer(non_blocking).json();

    tracing_subscriber::registry()
        .with(filter)
        .with(stdout_layer)
        .with(file_layer)
        .init();

    Ok(guard)
}

/// Test-only initializer that uses a tempdir and ignores re-init failures.
pub fn init_for_tests() -> Option<WorkerGuard> {
    let tmp = std::env::temp_dir().join("phoenix-test-logs");
    init(&tmp).ok()
}
```

- [ ] **Step 5: Re-export from lib.rs**

Modify `C:\Users\enesa\phoenix\crates\phoenix-core\src\lib.rs`:

```rust
//! Phoenix core domain logic.

pub mod config;
pub mod logging;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
```

- [ ] **Step 6: Run tests**

```bash
cargo test -p phoenix-core
```

Expected: all pass (smoke + 4 config + 1 logging = 6 passed).

- [ ] **Step 7: Commit**

```bash
git add crates/phoenix-core/
git commit -m "feat(core): add tracing-based logging with file rolling"
```

---

## Task 6: Opt-in telemetry framework

**Files:**
- Create: `C:\Users\enesa\phoenix\crates\phoenix-core\src\telemetry.rs`
- Modify: `C:\Users\enesa\phoenix\crates\phoenix-core\src\lib.rs`
- Create: `C:\Users\enesa\phoenix\crates\phoenix-core\tests\telemetry_test.rs`

- [ ] **Step 1: Write failing telemetry test**

Write `C:\Users\enesa\phoenix\crates\phoenix-core\tests\telemetry_test.rs`:

```rust
use phoenix_core::telemetry::{Event, Telemetry, TestSink};

#[test]
fn telemetry_disabled_drops_events() {
    let sink = TestSink::new();
    let telem = Telemetry::new(false, sink.clone());

    telem.record(Event::AppStart);
    telem.record(Event::InterviewQuestionAsked);

    assert_eq!(sink.events(), Vec::<Event>::new());
}

#[test]
fn telemetry_enabled_forwards_events() {
    let sink = TestSink::new();
    let telem = Telemetry::new(true, sink.clone());

    telem.record(Event::AppStart);
    telem.record(Event::InterviewQuestionAsked);

    assert_eq!(
        sink.events(),
        vec![Event::AppStart, Event::InterviewQuestionAsked]
    );
}

#[test]
fn events_are_serializable() {
    let event = Event::AppStart;
    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains("AppStart"));
}
```

- [ ] **Step 2: Run test (expect compile error)**

```bash
cargo test -p phoenix-core --test telemetry_test
```

- [ ] **Step 3: Implement telemetry.rs**

Write `C:\Users\enesa\phoenix\crates\phoenix-core\src\telemetry.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Anonymous telemetry events. No PII, no wallet contents, no identifying info.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Event {
    AppStart,
    AppShutdown,
    InterviewQuestionAsked,
    InterviewSessionCompleted,
    CandidateGenerated,
    CrackingStarted,
    CrackingFinished,
    RecoverySucceeded,
}

pub trait TelemetrySink: Send + Sync {
    fn emit(&self, event: Event);
}

pub struct Telemetry {
    enabled: bool,
    sink: Arc<dyn TelemetrySink>,
}

impl Telemetry {
    pub fn new<S: TelemetrySink + 'static>(enabled: bool, sink: S) -> Self {
        Self {
            enabled,
            sink: Arc::new(sink),
        }
    }

    pub fn record(&self, event: Event) {
        if self.enabled {
            self.sink.emit(event);
        }
    }
}

/// In-memory sink for tests.
#[derive(Clone, Default)]
pub struct TestSink {
    inner: Arc<Mutex<Vec<Event>>>,
}

impl TestSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn events(&self) -> Vec<Event> {
        self.inner.lock().unwrap().clone()
    }
}

impl TelemetrySink for TestSink {
    fn emit(&self, event: Event) {
        self.inner.lock().unwrap().push(event);
    }
}
```

- [ ] **Step 4: Re-export from lib.rs**

Modify `C:\Users\enesa\phoenix\crates\phoenix-core\src\lib.rs`:

```rust
//! Phoenix core domain logic.

pub mod config;
pub mod logging;
pub mod telemetry;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
```

- [ ] **Step 5: Run tests**

```bash
cargo test -p phoenix-core
```

Expected: 9 passed (1 smoke + 4 config + 1 logging + 3 telemetry).

- [ ] **Step 6: Commit**

```bash
git add crates/phoenix-core/
git commit -m "feat(core): add opt-in anonymous telemetry framework"
```

---

## Task 7: Ollama HTTP client (offline test)

**Files:**
- Create: `C:\Users\enesa\phoenix\crates\phoenix-core\src\llm\mod.rs`
- Create: `C:\Users\enesa\phoenix\crates\phoenix-core\src\llm\ollama.rs`
- Modify: `C:\Users\enesa\phoenix\crates\phoenix-core\src\lib.rs`
- Create: `C:\Users\enesa\phoenix\crates\phoenix-core\tests\ollama_offline_test.rs`

The actual Ollama integration test (Task 8) requires a running Ollama daemon. This task only covers serialization/deserialization and URL construction so we have something CI can verify without external deps.

- [ ] **Step 1: Add wiremock to dev-deps**

Modify `C:\Users\enesa\phoenix\crates\phoenix-core\Cargo.toml`:

```toml
[dev-dependencies]
tempfile = "3.10"
wiremock = "0.6"
```

- [ ] **Step 2: Write failing offline test using wiremock**

Write `C:\Users\enesa\phoenix\crates\phoenix-core\tests\ollama_offline_test.rs`:

```rust
use phoenix_core::llm::{LlmClient, OllamaClient};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn generate_returns_response_text() {
    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/generate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "model": "llama3.3:70b",
            "response": "hello world",
            "done": true
        })))
        .mount(&mock)
        .await;

    let client = OllamaClient::new(&mock.uri(), "llama3.3:70b");
    let result = client.generate("say hi").await.unwrap();

    assert_eq!(result, "hello world");
}

#[tokio::test]
async fn generate_returns_error_on_5xx() {
    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/generate"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock)
        .await;

    let client = OllamaClient::new(&mock.uri(), "llama3.3:70b");
    let result = client.generate("say hi").await;

    assert!(result.is_err());
}
```

- [ ] **Step 3: Run test (expect compile error)**

```bash
cargo test -p phoenix-core --test ollama_offline_test
```

- [ ] **Step 4: Implement llm/mod.rs**

Write `C:\Users\enesa\phoenix\crates\phoenix-core\src\llm\mod.rs`:

```rust
//! LLM client abstraction. The default implementation is OllamaClient.

mod ollama;

pub use ollama::OllamaClient;

use async_trait::async_trait;

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate(&self, prompt: &str) -> anyhow::Result<String>;
}
```

- [ ] **Step 5: Add async-trait dep**

Modify `C:\Users\enesa\phoenix\crates\phoenix-core\Cargo.toml` deps:

```toml
async-trait = "0.1"
```

- [ ] **Step 6: Implement llm/ollama.rs**

Write `C:\Users\enesa\phoenix\crates\phoenix-core\src\llm\ollama.rs`:

```rust
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::LlmClient;

#[derive(Debug, Clone)]
pub struct OllamaClient {
    endpoint: String,
    model: String,
    http: reqwest::Client,
}

#[derive(Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Deserialize)]
struct GenerateResponse {
    response: String,
}

impl OllamaClient {
    pub fn new(endpoint: &str, model: &str) -> Self {
        Self {
            endpoint: endpoint.trim_end_matches('/').to_string(),
            model: model.to_string(),
            http: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmClient for OllamaClient {
    async fn generate(&self, prompt: &str) -> Result<String> {
        let url = format!("{}/api/generate", self.endpoint);
        let body = GenerateRequest {
            model: &self.model,
            prompt,
            stream: false,
        };

        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await
            .with_context(|| format!("POST {url}"))?;

        if !resp.status().is_success() {
            anyhow::bail!("ollama returned status {}", resp.status());
        }

        let parsed: GenerateResponse = resp
            .json()
            .await
            .context("decoding Ollama JSON response")?;

        Ok(parsed.response)
    }
}
```

- [ ] **Step 7: Re-export from lib.rs**

Modify `C:\Users\enesa\phoenix\crates\phoenix-core\src\lib.rs`:

```rust
//! Phoenix core domain logic.

pub mod config;
pub mod llm;
pub mod logging;
pub mod telemetry;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
```

- [ ] **Step 8: Run tests**

```bash
cargo test -p phoenix-core
```

Expected: 11 passed.

- [ ] **Step 9: Commit**

```bash
git add crates/phoenix-core/
git commit -m "feat(core): add Ollama HTTP client behind LlmClient trait"
```

---

## Task 8: Live Ollama healthcheck CLI

**Files:**
- Modify: `C:\Users\enesa\phoenix\crates\phoenix-cli\src\main.rs`

This task introduces an `ollama-check` subcommand that verifies the user has Ollama running locally with the configured model. It is not part of CI; it is a developer / user diagnostic.

- [ ] **Step 1: Add subcommand and impl**

Replace `C:\Users\enesa\phoenix\crates\phoenix-cli\src\main.rs` with:

```rust
use clap::{Parser, Subcommand};
use phoenix_core::llm::{LlmClient, OllamaClient};

#[derive(Parser)]
#[command(name = "phoenix", version, about = "Phoenix recovery CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print version and environment info.
    Doctor,
    /// Round-trip a "ping" prompt against the configured Ollama instance.
    OllamaCheck {
        #[arg(long, default_value = "http://localhost:11434")]
        endpoint: String,
        #[arg(long, default_value = "llama3.3:70b")]
        model: String,
    },
}

fn doctor_output() -> String {
    format!("phoenix-core version: {}", phoenix_core::version())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Doctor => println!("{}", doctor_output()),
        Command::OllamaCheck { endpoint, model } => {
            let client = OllamaClient::new(&endpoint, &model);
            let response = client.generate("respond with the single word: ok").await?;
            println!("ollama responded: {}", response.trim());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doctor_output_contains_version() {
        let output = doctor_output();
        assert!(output.contains("phoenix-core version: 0.1.0"));
    }
}
```

- [ ] **Step 2: Run cli unit tests**

```bash
cargo test -p phoenix-cli
```

Expected: 1 passed.

- [ ] **Step 3: Manual smoke test (requires Ollama running with llama3.3:70b)**

```bash
ollama pull llama3.3:70b   # one-time, ~40GB download
ollama serve               # in another terminal
cargo run -p phoenix-cli -- ollama-check
```

Expected output similar to: `ollama responded: ok`

If Ollama is not yet installed, skip this step and document in README that `ollama-check` requires a running daemon.

- [ ] **Step 4: Commit**

```bash
git add crates/phoenix-cli/
git commit -m "feat(cli): add ollama-check subcommand for live diagnostic"
```

---

## Task 9: Tauri shell + first IPC command

**Files:**
- Create: `C:\Users\enesa\phoenix\src-tauri\Cargo.toml`
- Create: `C:\Users\enesa\phoenix\src-tauri\tauri.conf.json`
- Create: `C:\Users\enesa\phoenix\src-tauri\build.rs`
- Create: `C:\Users\enesa\phoenix\src-tauri\src\main.rs`
- Create: `C:\Users\enesa\phoenix\src-tauri\src\commands.rs`
- Create: `C:\Users\enesa\phoenix\src-tauri\icons\` (placeholder icon)

- [ ] **Step 1: Install Tauri prerequisites (one-time)**

```bash
# WebView2 (Windows ships it on Win 11), Rust target, Node
rustup target add x86_64-pc-windows-msvc
node --version  # require >= 20
```

- [ ] **Step 2: Write src-tauri/Cargo.toml**

Write `C:\Users\enesa\phoenix\src-tauri\Cargo.toml`:

```toml
[package]
name = "phoenix-tauri"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = [] }
phoenix-core = { path = "../crates/phoenix-core" }
serde.workspace = true
serde_json.workspace = true
tokio.workspace = true
tracing.workspace = true

[features]
custom-protocol = ["tauri/custom-protocol"]
```

- [ ] **Step 3: Write tauri.conf.json**

Write `C:\Users\enesa\phoenix\src-tauri\tauri.conf.json`:

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "Phoenix",
  "version": "0.1.0",
  "identifier": "com.phoenix.recovery",
  "build": {
    "frontendDist": "../src-ui/dist",
    "devUrl": "http://localhost:5173",
    "beforeDevCommand": "npm --prefix ../src-ui run dev",
    "beforeBuildCommand": "npm --prefix ../src-ui run build"
  },
  "app": {
    "windows": [
      {
        "title": "Phoenix",
        "width": 1280,
        "height": 800,
        "minWidth": 960,
        "minHeight": 600,
        "resizable": true
      }
    ],
    "security": {
      "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; connect-src ipc: http://ipc.localhost"
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": ["icons/32x32.png", "icons/128x128.png", "icons/icon.ico"]
  }
}
```

- [ ] **Step 4: Write build.rs**

Write `C:\Users\enesa\phoenix\src-tauri\build.rs`:

```rust
fn main() {
    tauri_build::build();
}
```

- [ ] **Step 5: Write commands.rs (IPC layer)**

Write `C:\Users\enesa\phoenix\src-tauri\src\commands.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
}

#[tauri::command]
pub fn app_info() -> AppInfo {
    AppInfo {
        name: "Phoenix".to_string(),
        version: phoenix_core::version().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_info_returns_name_and_version() {
        let info = app_info();
        assert_eq!(info.name, "Phoenix");
        assert_eq!(info.version, "0.1.0");
    }
}
```

- [ ] **Step 6: Write src-tauri/src/main.rs**

Write `C:\Users\enesa\phoenix\src-tauri\src\main.rs`:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::app_info])
        .run(tauri::generate_context!())
        .expect("error while running Phoenix");
}
```

- [ ] **Step 7: Add placeholder icons**

Create directory `C:\Users\enesa\phoenix\src-tauri\icons\` and place any 32x32 + 128x128 + .ico placeholder files there. For Week 1, copy from a Tauri starter template:

```bash
mkdir -p src-tauri/icons
# copy the default Tauri sample icons or generate plain colored squares
# placeholder ok for week 1; replace with brand assets in week 8
```

- [ ] **Step 8: Run unit tests**

```bash
cargo test -p phoenix-tauri
```

Expected: 1 passed.

- [ ] **Step 9: Commit**

```bash
git add src-tauri/
git commit -m "feat(tauri): scaffold Tauri shell with app_info IPC command"
```

---

## Task 10: React + Vite frontend with Welcome + Settings routes

**Files:**
- Create: `C:\Users\enesa\phoenix\src-ui\package.json`
- Create: `C:\Users\enesa\phoenix\src-ui\vite.config.ts`
- Create: `C:\Users\enesa\phoenix\src-ui\tsconfig.json`
- Create: `C:\Users\enesa\phoenix\src-ui\index.html`
- Create: `C:\Users\enesa\phoenix\src-ui\src\main.tsx`
- Create: `C:\Users\enesa\phoenix\src-ui\src\App.tsx`
- Create: `C:\Users\enesa\phoenix\src-ui\src\routes\Welcome.tsx`
- Create: `C:\Users\enesa\phoenix\src-ui\src\routes\Settings.tsx`
- Create: `C:\Users\enesa\phoenix\src-ui\src\lib\ipc.ts`
- Create: `C:\Users\enesa\phoenix\src-ui\src\lib\ipc.test.ts`

- [ ] **Step 1: Write package.json**

Write `C:\Users\enesa\phoenix\src-ui\package.json`:

```json
{
  "name": "phoenix-ui",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "test": "vitest run",
    "lint": "eslint src --max-warnings 0"
  },
  "dependencies": {
    "@tauri-apps/api": "^2.1.1",
    "react": "^18.3.1",
    "react-dom": "^18.3.1",
    "react-router-dom": "^6.28.0"
  },
  "devDependencies": {
    "@testing-library/react": "^16.0.1",
    "@types/react": "^18.3.0",
    "@types/react-dom": "^18.3.0",
    "@vitejs/plugin-react": "^4.3.4",
    "eslint": "^9.0.0",
    "@typescript-eslint/eslint-plugin": "^8.0.0",
    "@typescript-eslint/parser": "^8.0.0",
    "jsdom": "^25.0.1",
    "typescript": "^5.6.3",
    "vite": "^5.4.10",
    "vitest": "^2.1.4"
  }
}
```

- [ ] **Step 2: Install deps**

```bash
cd src-ui
npm install
```

Expected: clean install, no audit errors at warn-level.

- [ ] **Step 3: Write vite.config.ts**

Write `C:\Users\enesa\phoenix\src-ui\vite.config.ts`:

```ts
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: { port: 5173, strictPort: true },
  test: { environment: "jsdom" },
});
```

- [ ] **Step 4: Write tsconfig.json**

Write `C:\Users\enesa\phoenix\src-ui\tsconfig.json`:

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "lib": ["ES2022", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "skipLibCheck": true,
    "esModuleInterop": true,
    "allowSyntheticDefaultImports": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "types": ["vitest/globals"]
  },
  "include": ["src"]
}
```

- [ ] **Step 5: Write index.html**

Write `C:\Users\enesa\phoenix\src-ui\index.html`:

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Phoenix</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
```

- [ ] **Step 6: Write the failing IPC wrapper test**

Write `C:\Users\enesa\phoenix\src-ui\src\lib\ipc.test.ts`:

```ts
import { describe, it, expect, vi } from "vitest";
import { fetchAppInfo } from "./ipc";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(async (cmd: string) => {
    if (cmd === "app_info") {
      return { name: "Phoenix", version: "0.1.0" };
    }
    throw new Error(`unknown command: ${cmd}`);
  }),
}));

describe("fetchAppInfo", () => {
  it("returns name and version from the IPC bridge", async () => {
    const info = await fetchAppInfo();
    expect(info.name).toBe("Phoenix");
    expect(info.version).toBe("0.1.0");
  });
});
```

- [ ] **Step 7: Run the test (expect failure)**

```bash
cd src-ui
npm run test
```

Expected: cannot find module `./ipc`.

- [ ] **Step 8: Implement ipc.ts**

Write `C:\Users\enesa\phoenix\src-ui\src\lib\ipc.ts`:

```ts
import { invoke } from "@tauri-apps/api/core";

export interface AppInfo {
  name: string;
  version: string;
}

export async function fetchAppInfo(): Promise<AppInfo> {
  return invoke<AppInfo>("app_info");
}
```

- [ ] **Step 9: Re-run the test**

```bash
npm run test
```

Expected: 1 passed.

- [ ] **Step 10: Write Welcome route**

Write `C:\Users\enesa\phoenix\src-ui\src\routes\Welcome.tsx`:

```tsx
import { useEffect, useState } from "react";
import { fetchAppInfo, AppInfo } from "../lib/ipc";

export function Welcome() {
  const [info, setInfo] = useState<AppInfo | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchAppInfo()
      .then(setInfo)
      .catch((e: Error) => setError(e.message));
  }, []);

  return (
    <main>
      <h1>Phoenix</h1>
      <p>Open-source forensic recovery assistant.</p>
      {error && <p style={{ color: "red" }}>IPC error: {error}</p>}
      {info && (
        <p>
          {info.name} v{info.version}
        </p>
      )}
    </main>
  );
}
```

- [ ] **Step 11: Write Settings route stub**

Write `C:\Users\enesa\phoenix\src-ui\src\routes\Settings.tsx`:

```tsx
export function Settings() {
  return (
    <main>
      <h1>Settings</h1>
      <p>Configuration UI is not yet implemented.</p>
    </main>
  );
}
```

- [ ] **Step 12: Wire up routing**

Write `C:\Users\enesa\phoenix\src-ui\src\App.tsx`:

```tsx
import { BrowserRouter, Routes, Route, Link } from "react-router-dom";
import { Welcome } from "./routes/Welcome";
import { Settings } from "./routes/Settings";

export function App() {
  return (
    <BrowserRouter>
      <nav>
        <Link to="/">Home</Link> | <Link to="/settings">Settings</Link>
      </nav>
      <Routes>
        <Route path="/" element={<Welcome />} />
        <Route path="/settings" element={<Settings />} />
      </Routes>
    </BrowserRouter>
  );
}
```

Write `C:\Users\enesa\phoenix\src-ui\src\main.tsx`:

```tsx
import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./App";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
```

- [ ] **Step 13: Run all tests + build**

```bash
cd src-ui
npm run test
npm run build
```

Expected: tests pass, build succeeds, `src-ui/dist/` produced.

- [ ] **Step 14: Commit**

```bash
git add src-ui/
git commit -m "feat(ui): add React + Vite frontend with Welcome and Settings routes"
```

---

## Task 11: GitHub Actions CI

**Files:**
- Create: `C:\Users\enesa\phoenix\.github\workflows\ci.yml`
- Create: `C:\Users\enesa\phoenix\.github\workflows\lint.yml`

- [ ] **Step 1: Write CI workflow**

Write `C:\Users\enesa\phoenix\.github\workflows\ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  rust:
    name: Rust ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Install system deps (Ubuntu)
        if: matrix.os == 'ubuntu-latest'
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev libssl-dev libgtk-3-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev
      - name: Cache cargo
        uses: Swatinem/rust-cache@v2
      - name: Cargo build
        run: cargo build --workspace --exclude phoenix-tauri
      - name: Cargo test
        run: cargo test --workspace --exclude phoenix-tauri

  ui:
    name: UI tests
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: src-ui
    steps:
      - uses: actions/checkout@v4
      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: npm
          cache-dependency-path: src-ui/package-lock.json
      - run: npm ci
      - run: npm run test
      - run: npm run build
```

We exclude `phoenix-tauri` from the matrix Rust test job because Tauri requires platform-specific build deps; building the bundle itself is left for a separate release workflow added in Week 8.

- [ ] **Step 2: Write lint workflow**

Write `C:\Users\enesa\phoenix\.github\workflows\lint.yml`:

```yaml
name: Lint

on:
  push:
    branches: [main]
  pull_request:

jobs:
  clippy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - name: Install system deps
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev libssl-dev libgtk-3-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev
      - uses: Swatinem/rust-cache@v2
      - name: cargo fmt
        run: cargo fmt --all -- --check
      - name: cargo clippy (workspace minus tauri shell)
        run: cargo clippy --workspace --exclude phoenix-tauri -- -D warnings

  eslint:
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: src-ui
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: npm
          cache-dependency-path: src-ui/package-lock.json
      - run: npm ci
      - run: npm run lint
```

- [ ] **Step 3: Commit**

```bash
git add .github/
git commit -m "ci: add GitHub Actions for Rust + UI tests and linting"
```

- [ ] **Step 4: Push to GitHub (manual, when remote exists)**

This step is deferred until the user creates a GitHub repository (planned for Task 13). For now, the workflow files are staged locally.

---

## Task 12: Initial threat model document

**Files:**
- Create: `C:\Users\enesa\phoenix\docs\threat-model-v0.md`

- [ ] **Step 1: Write threat-model-v0.md**

Write `C:\Users\enesa\phoenix\docs\threat-model-v0.md`:

```markdown
# Phoenix Threat Model — v0 (Week 1)

This is the initial scaffold. A full threat model (with adversary trees, mitigations, and security architecture review) is scheduled for Week 4. This v0 document captures the assumptions baked into the Week 1 foundation so future work can refer back to them.

## Trust boundaries

1. **User machine ↔ Phoenix process:** Phoenix runs as a desktop process owned by the user. No elevation required. No background daemon.
2. **Phoenix ↔ Ollama:** HTTP loopback. The user is responsible for trusting their local Ollama installation.
3. **Phoenix ↔ Internet:** Only outbound, only to:
   - GitHub releases (signed update manifest)
   - Optional opt-in telemetry endpoint (anonymous events)
   - Optional cloud GPU offload endpoint (only if user enables Pro tier with cloud cracking)

## Adversaries (initial set)

| Adversary | Capability | Phoenix v0 mitigation |
|---|---|---|
| Passive network observer | Reads all unencrypted traffic | All outbound HTTPS, no telemetry by default |
| Malicious Ollama image | User pulls a poisoned model | Out of scope for Phoenix; documented in README |
| Compromised dev dependency | Supply-chain attack via cargo / npm | Lockfiles committed; CI runs `cargo audit` (Week 2) |
| Phoenix maintainer | Inserts backdoor in upstream | Open-source from Day 1; deterministic builds Week 8 |
| User's own malware | Reads phoenix process memory | Out of scope (host compromise = game over) |

## Non-secrets handled in Week 1

- Configuration (telemetry on/off, ollama endpoint, model id)
- Application logs (timestamps, log levels, no user content)
- Telemetry events (enum tags only, no payloads)

## Secrets NOT yet handled in Week 1

- Wallet seeds (Layer 2-5 work in Weeks 2-5)
- Private keys (Layer 5)
- Recovery candidate lists (Layer 3)

These will get their own threat model entries as the corresponding layers ship.
```

- [ ] **Step 2: Commit**

```bash
git add docs/threat-model-v0.md
git commit -m "docs: add v0 threat model scaffold"
```

---

## Task 13: README polish + Week 1 completion

**Files:**
- Modify: `C:\Users\enesa\phoenix\README.md`

- [ ] **Step 1: Expand README**

Replace `C:\Users\enesa\phoenix\README.md` with:

```markdown
# Phoenix

> Open-source forensic recovery assistant for partial-information lost crypto wallets.

**Status:** Pre-alpha (Week 1 — foundation complete).

## What Phoenix is

Phoenix systematizes what artisan wallet-recovery shops do manually: a structured cognitive interview, deep digital exhaust forensics, Bayesian candidate ranking, and distributed cracking — orchestrated as a single open-source desktop application.

## What Phoenix is NOT

- Not a recovery promise. Wallets with zero memory and zero digital traces cannot be recovered. Period.
- Not a hardware-glitch tool. Hardware PIN attacks are out of scope.
- Not a cloud service. All recovery work happens locally on your machine.
- Not "AI wallet recovery." Phoenix is forensic and guided, not magic.

## Realistic outcomes

For wallets where partial information exists (forgotten 1-2 seed words, wallet.dat with remembered password pattern, lost backups with traceable digital exhaust): expected v1 recovery rate **35-50%**.

For wallets with no recoverable signal: **0%**. Always.

## Repository layout

```
phoenix/
├── crates/phoenix-core      # Domain logic (config, logging, telemetry, llm)
├── crates/phoenix-cli       # Developer ergonomics (`phoenix doctor`, `phoenix ollama-check`)
├── src-tauri                # Tauri desktop shell + IPC commands
├── src-ui                   # React + Vite frontend
├── docs/superpowers         # Spec + plan documents
└── docs/threat-model-v0.md  # Initial threat model
```

## Building from source

### Prerequisites

- Rust 1.83+ (`rustup install stable`)
- Node 20+
- Tauri prerequisites for your OS — see https://tauri.app/start/prerequisites/
- Ollama, with `llama3.3:70b` pulled locally:
  ```bash
  ollama pull llama3.3:70b
  ollama serve
  ```

### Build

```bash
git clone <this-repo>
cd phoenix
cargo build --workspace
cd src-ui && npm install && cd ..
cargo tauri dev   # launch the desktop app
```

### Run tests

```bash
cargo test --workspace --exclude phoenix-tauri
cd src-ui && npm run test
```

### Diagnostics

```bash
cargo run -p phoenix-cli -- doctor
cargo run -p phoenix-cli -- ollama-check
```

## Design

The full design specification is at [`docs/superpowers/specs/2026-05-08-phoenix-design.md`](docs/superpowers/specs/2026-05-08-phoenix-design.md).

The Week 1 implementation plan is at [`docs/superpowers/plans/2026-05-08-phoenix-week1-foundation.md`](docs/superpowers/plans/2026-05-08-phoenix-week1-foundation.md).

## License

MIT. See [`LICENSE`](LICENSE).
```

- [ ] **Step 2: Run full test suite one last time**

```bash
cd /c/Users/enesa/phoenix
cargo test --workspace --exclude phoenix-tauri
cargo test -p phoenix-tauri
cd src-ui && npm run test && npm run build && cd ..
```

Expected: every suite green.

- [ ] **Step 3: Tag Week 1 milestone**

```bash
git add README.md
git commit -m "docs: expand README with build + run instructions"
git tag week-1-foundation
```

---

## Week 1 Acceptance Criteria

The Week 1 plan is **done** when all of the following hold:

- [ ] `cargo test --workspace --exclude phoenix-tauri` is green on Linux + Windows + macOS in CI
- [ ] `cargo test -p phoenix-tauri` is green locally
- [ ] `npm run test && npm run build` is green in `src-ui/`
- [ ] `cargo run -p phoenix-cli -- doctor` prints the version
- [ ] `cargo run -p phoenix-cli -- ollama-check` succeeds against a local Ollama daemon
- [ ] `cargo tauri dev` launches the desktop window with the Welcome route showing `Phoenix v0.1.0`
- [ ] Every commit message follows the `<type>(<scope>): <description>` convention used above
- [ ] `git tag week-1-foundation` exists

If any criterion fails, fix it before starting Week 2.

---

## What comes next

After Week 1 acceptance:

1. **Week 2 plan** (Cognitive Interview MVP) — drafted at the start of Week 2, gated by Week 1 acceptance.
2. **Phoenix design spec review** — Enes reads the spec end-to-end and notes any changes; revisions land before Week 2 starts.
3. **Hukuki danışman call** — 30-minute jurisdiction question (CH vs EE), book in Week 2.
4. **Trail of Bits engagement letter** — initiate scoping in Week 8.

This plan is intentionally focused. No naming workshop, no marketing site, no GitHub remote setup, no audit firm outreach happens in Week 1. Those land in Week 8 (launch prep).
