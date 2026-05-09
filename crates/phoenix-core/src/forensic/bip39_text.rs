use std::path::Path;

use super::bip39_word_set;
use super::registry::{ExtractError, Extractor};
use crate::interview::state::{MemoryNode, MemoryNodeKind};

/// Universal text-file scanner. Reads any text file and extracts:
/// - sequences of consecutive BIP-39 words (length ≥ 3) → SeedFragment
/// - solitary BIP-39 words appearing repeatedly → SeedFragment with lower confidence
///
/// Confidence scoring:
/// - Sequence of 11+ BIP-39 words: 0.95 (likely a real seed)
/// - Sequence of 3–10:            0.6
/// - Single recurring word:       0.2
pub struct Bip39TextExtractor;

impl Extractor for Bip39TextExtractor {
    fn name(&self) -> &str {
        "bip39_text"
    }

    fn extensions(&self) -> &[&str] {
        // Empty list means "match any extension" — we run on every imported file.
        &[]
    }

    fn extract(&self, path: &Path) -> Result<Vec<MemoryNode>, ExtractError> {
        let raw = std::fs::read_to_string(path)?;
        Ok(scan(&raw, &path.display().to_string()))
    }
}

pub fn scan(text: &str, source_label: &str) -> Vec<MemoryNode> {
    let words = bip39_word_set();
    let tokens: Vec<&str> = text
        .split(|c: char| !c.is_alphabetic())
        .filter(|t| !t.is_empty())
        .collect();

    let mut sequences: Vec<Vec<&str>> = Vec::new();
    let mut current: Vec<&str> = Vec::new();

    for tok in &tokens {
        let lower = tok.to_lowercase();
        if words.contains(lower.as_str()) {
            current.push(tok);
        } else if current.len() >= 3 {
            sequences.push(std::mem::take(&mut current));
        } else {
            current.clear();
        }
    }
    if current.len() >= 3 {
        sequences.push(current);
    }

    let mut nodes = Vec::new();
    for seq in sequences {
        let confidence = if seq.len() >= 11 {
            0.95
        } else if seq.len() >= 3 {
            0.6
        } else {
            0.2
        };
        let phrase = seq
            .iter()
            .map(|s| s.to_lowercase())
            .collect::<Vec<_>>()
            .join(" ");
        nodes.push(MemoryNode::new(
            MemoryNodeKind::SeedFragment,
            phrase,
            confidence,
            vec![source_label.to_string()],
        ));
    }
    nodes
}
