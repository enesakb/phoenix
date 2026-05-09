use anyhow::Result;
use futures::future::join_all;
use serde::{Deserialize, Serialize};

use crate::llm::LlmClient;

use super::questions::Question;

/// Three perspectives that cross-question the user's answer (Du et al. 2023).
const PERSPECTIVES: &[&str] = &[
    "skeptical analyst — flag inconsistencies and ambiguities",
    "empathetic listener — surface emotional anchors that may aid recall",
    "technical forensicist — map answers to specific recoverable data sources",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebateOutput {
    pub follow_ups: Vec<String>,
}

pub async fn cross_question(
    llm: &dyn LlmClient,
    question: &Question,
    answer: &str,
) -> Result<DebateOutput> {
    if answer.trim().is_empty() {
        return Ok(DebateOutput {
            follow_ups: Vec::new(),
        });
    }

    let futs = PERSPECTIVES
        .iter()
        .map(|persona| {
            let prompt = build_debate_prompt(persona, question, answer);
            async move { llm.generate(&prompt).await }
        })
        .collect::<Vec<_>>();

    let raws = join_all(futs).await;

    let mut follow_ups = Vec::new();
    for raw in raws.into_iter().flatten() {
        for line in raw.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix('-') {
                let q = rest.trim().trim_end_matches('.');
                if !q.is_empty() {
                    follow_ups.push(q.to_string());
                }
            }
        }
    }

    follow_ups.sort();
    follow_ups.dedup();
    follow_ups.truncate(6);

    Ok(DebateOutput { follow_ups })
}

pub fn build_debate_prompt(persona: &str, question: &Question, answer: &str) -> String {
    format!(
        "You are a {persona}. The user is being interviewed to recover access to a lost crypto wallet.\n\
        Original question: {question}\n\
        User answer: {answer}\n\n\
        Suggest at most TWO sharp follow-up questions that would extract more recoverable detail from this user.\n\
        Reply as plain bullet list, one question per line, starting with '- '. No commentary.\n",
        persona = persona,
        question = question.text,
        answer = answer
    )
}
