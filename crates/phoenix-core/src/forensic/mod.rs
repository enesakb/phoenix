//! Layer 2 — Digital Forensic Excavator (file-import surface).
//!
//! The user explicitly imports a file (browser history copy, password-manager
//! export, plain text). Phoenix dispatches to a registered Extractor based on
//! file extension/signature and produces MemoryNodes that feed the same
//! candidate pipeline as the cognitive interview.

pub mod bip39_text;
pub mod bitwarden_csv;
pub mod chrome_history;
pub mod registry;

pub use bip39_text::Bip39TextExtractor;
pub use bitwarden_csv::BitwardenCsvExtractor;
pub use chrome_history::ChromeHistoryExtractor;
pub use registry::{ExtractError, Extractor, ExtractorRegistry};

use std::collections::HashSet;
use std::sync::OnceLock;

/// Embedded BIP-39 English wordlist (2048 entries) accessed via the `bip39` crate.
pub fn bip39_wordlist() -> &'static [&'static str; 2048] {
    bip39::Language::English.word_list()
}

/// O(1) membership test for BIP-39 words.
pub fn bip39_word_set() -> &'static HashSet<&'static str> {
    static SET: OnceLock<HashSet<&'static str>> = OnceLock::new();
    SET.get_or_init(|| bip39_wordlist().iter().copied().collect())
}
