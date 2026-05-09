use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// What kind of memory hint a node represents.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MemoryNodeKind {
    /// A confirmed factual hint (year, wallet type, etc.).
    Fact,
    /// A pattern the user uses across passwords.
    PasswordPattern,
    /// A passphrase fragment or candidate word.
    PassphraseFragment,
    /// A possible seed word or partial seed.
    SeedFragment,
    /// A location, person, or context that may produce more leads later.
    ContextualLead,
    /// A pointer to a digital or physical artifact worth searching.
    ArtifactPointer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryNode {
    pub id: String,
    pub kind: MemoryNodeKind,
    pub content: String,
    /// 0.0 (pure speculation) to 1.0 (user-confirmed certainty).
    pub confidence: f32,
    /// Question IDs that contributed to extracting this node.
    pub source_question_ids: Vec<String>,
    pub created_at: DateTime<Utc>,
}

impl MemoryNode {
    pub fn new(
        kind: MemoryNodeKind,
        content: impl Into<String>,
        confidence: f32,
        sources: Vec<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            kind,
            content: content.into(),
            confidence: confidence.clamp(0.0, 1.0),
            source_question_ids: sources,
            created_at: Utc::now(),
        }
    }
}

/// Aggregate state across an interview — the union of all extracted hints.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryState {
    pub nodes: Vec<MemoryNode>,
}

impl MemoryState {
    pub fn add(&mut self, node: MemoryNode) {
        self.nodes.push(node);
    }

    pub fn nodes_of(&self, kind: MemoryNodeKind) -> Vec<&MemoryNode> {
        self.nodes.iter().filter(|n| n.kind == kind).collect()
    }

    pub fn high_confidence(&self, threshold: f32) -> Vec<&MemoryNode> {
        self.nodes
            .iter()
            .filter(|n| n.confidence >= threshold)
            .collect()
    }
}
