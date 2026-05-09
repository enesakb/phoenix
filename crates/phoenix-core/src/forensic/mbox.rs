//! mbox email-archive scanner.
//!
//! `.mbox` files concatenate RFC-822 messages separated by lines starting
//! with "From " (capital F, space). We do not bother fully parsing MIME —
//! we just walk the body text and run the same BIP-39 sequence detector
//! that powers `Bip39TextExtractor`, plus a crypto-keyword extractor
//! that surfaces wallet-related messages as `ContextualLead`s.

use std::path::Path;

use super::bip39_text::scan as bip39_scan;
use super::registry::{ExtractError, Extractor};
use crate::interview::state::{MemoryNode, MemoryNodeKind};

const CRYPTO_SUBJECT_KEYWORDS: &[&str] = &[
    "wallet",
    "seed",
    "mnemonic",
    "passphrase",
    "metamask",
    "ledger",
    "trezor",
    "phantom",
    "coinbase",
    "binance",
    "kraken",
    "recovery",
    "backup",
    "bip39",
    "bitcoin",
    "ethereum",
    "solana",
];

pub struct MboxExtractor;

impl Extractor for MboxExtractor {
    fn name(&self) -> &str {
        "mbox"
    }

    fn extensions(&self) -> &[&str] {
        &["mbox"]
    }

    fn extract(&self, path: &Path) -> Result<Vec<MemoryNode>, ExtractError> {
        let raw = std::fs::read_to_string(path)?;
        Ok(scan_mbox(&raw, &path.display().to_string()))
    }
}

pub fn scan_mbox(text: &str, source_label: &str) -> Vec<MemoryNode> {
    let mut nodes = Vec::new();

    // Each message starts with a line beginning with `From ` at column 0.
    // We split on the marker pattern (newline followed by `From ` plus space).
    let messages: Vec<&str> = text
        .split("\nFrom ")
        .filter(|m| !m.trim().is_empty())
        .collect();

    for msg in messages {
        // Try to lift the Subject: line for triage.
        let subject = msg
            .lines()
            .find(|l| l.to_ascii_lowercase().starts_with("subject:"))
            .map(|l| l.trim_start_matches("Subject:").trim().to_string())
            .unwrap_or_default();

        let lower_subj = subject.to_lowercase();
        let matched_keyword = CRYPTO_SUBJECT_KEYWORDS
            .iter()
            .find(|k| lower_subj.contains(*k));

        if let Some(kw) = matched_keyword {
            nodes.push(MemoryNode::new(
                MemoryNodeKind::ContextualLead,
                format!("mbox subject contains '{kw}': {subject}"),
                0.4,
                vec![source_label.to_string()],
            ));
        }

        // Always run the BIP-39 sequence detector on the body — even spam
        // messages occasionally have stale seeds.
        nodes.extend(bip39_scan(msg, source_label));
    }

    nodes
}
