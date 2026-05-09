use serde::{Deserialize, Serialize};

use super::state::{MemoryNode, MemoryNodeKind, MemoryState};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum CandidateKind {
    SeedWord,
    SeedPhraseFragment,
    Passphrase,
    PasswordCandidate,
    DerivationPath,
    WalletKindHint,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Candidate {
    pub kind: CandidateKind,
    pub value: String,
    /// 0.0–1.0; product of source-node confidences and structural priors.
    pub score: f32,
    pub supporting_node_ids: Vec<String>,
}

impl Candidate {
    pub fn new(
        kind: CandidateKind,
        value: impl Into<String>,
        score: f32,
        sources: Vec<String>,
    ) -> Self {
        Self {
            kind,
            value: value.into(),
            score: score.clamp(0.0, 1.0),
            supporting_node_ids: sources,
        }
    }
}

/// Build a sorted candidate list from the union of memory nodes.
///
/// Layer 1 produces seed/passphrase *fragments*. Layer 3 (constraint
/// propagation, Week 5) is responsible for combining them into full
/// candidate phrases — we deliberately keep this module simple.
pub fn extract(state: &MemoryState) -> Vec<Candidate> {
    let mut out = Vec::new();
    for node in &state.nodes {
        let kind = match node.kind {
            MemoryNodeKind::SeedFragment => CandidateKind::SeedPhraseFragment,
            MemoryNodeKind::PassphraseFragment => CandidateKind::Passphrase,
            MemoryNodeKind::PasswordPattern => CandidateKind::PasswordCandidate,
            MemoryNodeKind::Fact
            | MemoryNodeKind::ContextualLead
            | MemoryNodeKind::ArtifactPointer => {
                continue;
            }
        };
        out.push(Candidate::new(
            kind,
            &node.content,
            score_for(node),
            vec![node.id.clone()],
        ));
    }
    out.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    out
}

/// Combined score: confidence × kind-prior. Pure data; no I/O.
fn score_for(node: &MemoryNode) -> f32 {
    let prior = match node.kind {
        MemoryNodeKind::SeedFragment => 1.0,
        MemoryNodeKind::PassphraseFragment => 0.9,
        MemoryNodeKind::PasswordPattern => 0.7,
        _ => 0.0,
    };
    node.confidence * prior
}
