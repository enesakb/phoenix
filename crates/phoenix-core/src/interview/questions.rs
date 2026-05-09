use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QuestionBankError {
    #[error("failed to read question bank: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse question bank: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("question bank is empty")]
    Empty,
}

/// A single Fisher–Geiselman-style question.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Question {
    pub id: String,
    pub category: QuestionCategory,
    pub text: String,
    /// Hints fed to the LLM so it can generate adaptive follow-ups.
    pub follow_up_hints: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum QuestionCategory {
    /// Free recall — open-ended "tell me everything you remember."
    FreeRecall,
    /// Context reinstatement — environment, time of day, mood.
    ContextReinstatement,
    /// Reverse order — events in reverse chronology.
    ReverseOrder,
    /// Change perspective — recall as if observing yourself.
    ChangePerspective,
    /// Wallet-specific — type, chain, software.
    WalletSpecific,
    /// Pattern identification — passwords, phrases used elsewhere.
    PatternIdentification,
    /// Physical artifacts — papers, devices, fragments.
    PhysicalArtifacts,
    /// Digital artifacts — backups, photos, password managers.
    DigitalArtifacts,
}

#[derive(Debug, Clone)]
pub struct QuestionBank {
    questions: Vec<Question>,
}

impl QuestionBank {
    pub fn from_path(path: &Path) -> Result<Self, QuestionBankError> {
        let raw = std::fs::read_to_string(path)?;
        let questions: Vec<Question> = serde_json::from_str(&raw)?;
        if questions.is_empty() {
            return Err(QuestionBankError::Empty);
        }
        Ok(Self { questions })
    }

    pub fn from_embedded() -> Result<Self, QuestionBankError> {
        let raw = include_str!("../../data/questions.json");
        let questions: Vec<Question> = serde_json::from_str(raw)?;
        if questions.is_empty() {
            return Err(QuestionBankError::Empty);
        }
        Ok(Self { questions })
    }

    pub fn len(&self) -> usize {
        self.questions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.questions.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Question> {
        self.questions.iter()
    }

    pub fn get(&self, id: &str) -> Option<&Question> {
        self.questions.iter().find(|q| q.id == id)
    }

    pub fn by_category(&self, cat: QuestionCategory) -> Vec<&Question> {
        self.questions
            .iter()
            .filter(|q| q.category == cat)
            .collect()
    }

    /// Default ordering: free recall first, then context, then specifics, then artifacts.
    pub fn ordered(&self) -> Vec<&Question> {
        let preferred = [
            QuestionCategory::FreeRecall,
            QuestionCategory::ContextReinstatement,
            QuestionCategory::WalletSpecific,
            QuestionCategory::PatternIdentification,
            QuestionCategory::ReverseOrder,
            QuestionCategory::ChangePerspective,
            QuestionCategory::PhysicalArtifacts,
            QuestionCategory::DigitalArtifacts,
        ];
        let mut out = Vec::with_capacity(self.questions.len());
        for cat in preferred {
            for q in &self.questions {
                if q.category == cat {
                    out.push(q);
                }
            }
        }
        out
    }
}
