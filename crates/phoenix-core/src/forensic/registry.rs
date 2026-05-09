use std::path::Path;
use thiserror::Error;

use crate::interview::state::MemoryNode;

#[derive(Debug, Error)]
pub enum ExtractError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("csv: {0}")]
    Csv(#[from] csv::Error),
    #[error("no extractor handles file: {0}")]
    Unsupported(String),
    #[error("extractor failed: {0}")]
    Other(String),
}

pub trait Extractor: Send + Sync {
    fn name(&self) -> &str;
    /// Lowercase, no leading dot. e.g. "csv".
    fn extensions(&self) -> &[&str];
    fn extract(&self, path: &Path) -> Result<Vec<MemoryNode>, ExtractError>;
}

#[derive(Default)]
pub struct ExtractorRegistry {
    extractors: Vec<Box<dyn Extractor>>,
}

impl ExtractorRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<E: Extractor + 'static>(mut self, e: E) -> Self {
        self.extractors.push(Box::new(e));
        self
    }

    /// Run every extractor whose extension list matches the file. We deliberately
    /// allow multiple matches (e.g. a CSV may contain BIP-39 words too).
    pub fn dispatch(&self, path: &Path) -> Result<Vec<MemoryNode>, ExtractError> {
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        let matches: Vec<&dyn Extractor> = self
            .extractors
            .iter()
            .map(|b| b.as_ref())
            .filter(|e| e.extensions().is_empty() || e.extensions().contains(&ext.as_str()))
            .collect();

        if matches.is_empty() {
            return Err(ExtractError::Unsupported(path.display().to_string()));
        }

        let mut all = Vec::new();
        for e in matches {
            let mut nodes = e.extract(path)?;
            all.append(&mut nodes);
        }
        Ok(all)
    }

    pub fn len(&self) -> usize {
        self.extractors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.extractors.is_empty()
    }
}
