use std::path::Path;

use super::registry::{ExtractError, Extractor};
use crate::interview::state::{MemoryNode, MemoryNodeKind};

/// Parser for Bitwarden CSV exports.
///
/// Bitwarden's standard export columns include: `name`, `login_uri`, `login_username`,
/// `login_password`, `notes`. We feed both passwords and notes as PasswordPattern
/// nodes (low confidence; Layer 3 will re-score based on co-occurrence with the
/// wallet creation timeframe).
pub struct BitwardenCsvExtractor;

impl Extractor for BitwardenCsvExtractor {
    fn name(&self) -> &str {
        "bitwarden_csv"
    }

    fn extensions(&self) -> &[&str] {
        &["csv"]
    }

    fn extract(&self, path: &Path) -> Result<Vec<MemoryNode>, ExtractError> {
        let mut reader = csv::Reader::from_path(path)?;
        let mut nodes = Vec::new();
        for result in reader.deserialize::<BitwardenRow>() {
            let row = match result {
                Ok(r) => r,
                Err(_) => continue,
            };
            let label = format!("bitwarden_csv:{}", path.display());
            if let Some(pwd) = row.login_password.as_deref() {
                if !pwd.trim().is_empty() {
                    nodes.push(MemoryNode::new(
                        MemoryNodeKind::PasswordPattern,
                        pwd,
                        0.5,
                        vec![label.clone()],
                    ));
                }
            }
            if let Some(notes) = row.notes.as_deref() {
                if !notes.trim().is_empty() {
                    nodes.push(MemoryNode::new(
                        MemoryNodeKind::ContextualLead,
                        notes,
                        0.4,
                        vec![label],
                    ));
                }
            }
        }
        Ok(nodes)
    }
}

#[derive(Debug, serde::Deserialize)]
struct BitwardenRow {
    #[serde(default)]
    login_password: Option<String>,
    #[serde(default)]
    notes: Option<String>,
}
