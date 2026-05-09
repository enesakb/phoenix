use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

use crate::llm::LlmClient;

use super::questions::Question;
use super::state::{MemoryNode, MemoryNodeKind};

/// LLM-driven extractor. Given a question and the user's free-form answer,
/// returns memory nodes the LLM considers extractable.
pub struct Interviewer<'a> {
    llm: &'a dyn LlmClient,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LlmExtraction {
    pub nodes: Vec<LlmNode>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LlmNode {
    pub kind: String,
    pub content: String,
    pub confidence: f32,
}

impl<'a> Interviewer<'a> {
    pub fn new(llm: &'a dyn LlmClient) -> Self {
        Self { llm }
    }

    pub async fn extract(&self, question: &Question, answer: &str) -> Result<Vec<MemoryNode>> {
        if answer.trim().is_empty() {
            return Ok(Vec::new());
        }
        let prompt = build_extraction_prompt(question, answer);
        let raw = self
            .llm
            .generate(&prompt)
            .await
            .context("LLM generate for extraction")?;
        let parsed = parse_extraction(&raw)
            .with_context(|| format!("parse extraction LLM response: {raw}"))?;
        let nodes = parsed
            .nodes
            .into_iter()
            .filter_map(|n| {
                let kind = parse_kind(&n.kind)?;
                Some(MemoryNode::new(
                    kind,
                    n.content,
                    n.confidence,
                    vec![question.id.clone()],
                ))
            })
            .collect();
        Ok(nodes)
    }
}

fn parse_kind(s: &str) -> Option<MemoryNodeKind> {
    match s {
        "fact" => Some(MemoryNodeKind::Fact),
        "password_pattern" => Some(MemoryNodeKind::PasswordPattern),
        "passphrase_fragment" => Some(MemoryNodeKind::PassphraseFragment),
        "seed_fragment" => Some(MemoryNodeKind::SeedFragment),
        "contextual_lead" => Some(MemoryNodeKind::ContextualLead),
        "artifact_pointer" => Some(MemoryNodeKind::ArtifactPointer),
        _ => None,
    }
}

pub fn build_extraction_prompt(question: &Question, answer: &str) -> String {
    format!(
        "You are a forensic interview assistant for cryptocurrency wallet recovery.\n\n\
        Extract atomic memory hints from the user's answer. Each hint is a JSON object with:\n\
        - kind: one of fact, password_pattern, passphrase_fragment, seed_fragment, contextual_lead, artifact_pointer\n\
        - content: short verbatim string, no commentary\n\
        - confidence: 0.0 (pure inference) to 1.0 (user-confirmed)\n\n\
        Reply with strict JSON: {{\"nodes\":[...]}}\n\
        If nothing extractable, reply {{\"nodes\":[]}}\n\n\
        Question: {question}\n\
        User answer: {answer}\n",
        question = question.text,
        answer = answer
    )
}

pub fn parse_extraction(raw: &str) -> Result<LlmExtraction> {
    // Tolerate code-fenced JSON.
    let trimmed = raw.trim();
    let json_str = if let Some(start) = trimmed.find('{') {
        if let Some(end) = trimmed.rfind('}') {
            &trimmed[start..=end]
        } else {
            return Err(anyhow!("no closing brace in LLM response"));
        }
    } else {
        return Err(anyhow!("no opening brace in LLM response"));
    };
    let parsed: LlmExtraction = serde_json::from_str(json_str)?;
    Ok(parsed)
}
