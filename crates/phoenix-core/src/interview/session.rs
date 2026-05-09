use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

use super::candidate::Candidate;
use super::state::MemoryState;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("session not found: {0}")]
    NotFound(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub question_id: String,
    pub content: String,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterviewSession {
    pub id: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub answers: Vec<Answer>,
    pub memory: MemoryState,
    pub candidates: Vec<Candidate>,
}

impl InterviewSession {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            started_at: Utc::now(),
            completed_at: None,
            answers: Vec::new(),
            memory: MemoryState::default(),
            candidates: Vec::new(),
        }
    }

    pub fn record_answer(&mut self, question_id: impl Into<String>, content: impl Into<String>) {
        self.answers.push(Answer {
            question_id: question_id.into(),
            content: content.into(),
            recorded_at: Utc::now(),
        });
    }

    pub fn complete(&mut self) {
        self.completed_at = Some(Utc::now());
    }
}

impl Default for InterviewSession {
    fn default() -> Self {
        Self::new()
    }
}

/// JSON-on-disk persistence. SQLite migration deferred to Week 4.
#[derive(Debug, Clone)]
pub struct SessionStore {
    root: PathBuf,
}

impl SessionStore {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self, SessionError> {
        let root: PathBuf = root.into();
        std::fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    fn path_for(&self, id: &str) -> PathBuf {
        self.root.join(format!("{id}.json"))
    }

    pub fn save(&self, session: &InterviewSession) -> Result<(), SessionError> {
        let path = self.path_for(&session.id);
        let raw = serde_json::to_string_pretty(session)?;
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, raw)?;
        std::fs::rename(tmp, path)?;
        Ok(())
    }

    pub fn load(&self, id: &str) -> Result<InterviewSession, SessionError> {
        let path = self.path_for(id);
        if !path.exists() {
            return Err(SessionError::NotFound(id.to_string()));
        }
        let raw = std::fs::read_to_string(path)?;
        let session: InterviewSession = serde_json::from_str(&raw)?;
        Ok(session)
    }

    pub fn list(&self) -> Result<Vec<String>, SessionError> {
        let mut ids = Vec::new();
        for entry in std::fs::read_dir(&self.root)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    ids.push(stem.to_string());
                }
            }
        }
        Ok(ids)
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}
