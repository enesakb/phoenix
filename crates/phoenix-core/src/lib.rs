//! Phoenix core domain logic.
//!
//! All recovery primitives live here so they can be used both from the Tauri
//! shell and the CLI. Nothing in this crate touches the network without an
//! explicit user action.

pub mod config;
pub mod interview;
pub mod llm;
pub mod logging;
pub mod telemetry;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
