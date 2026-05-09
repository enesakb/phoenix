use std::path::Path;

use rusqlite::Connection;

use super::registry::{ExtractError, Extractor};
use crate::interview::state::{MemoryNode, MemoryNodeKind};

/// Parser for Chrome / Edge / Brave History SQLite databases.
///
/// The user must copy `History` (no extension) out of the running browser's
/// User Data folder before running Phoenix; we do not unlock the live DB.
///
/// We pull:
/// - `urls.url` matching wallet/exchange/crypto domain keywords → ContextualLead
/// - `keyword_search_terms.term` containing wallet keywords → ContextualLead
pub struct ChromeHistoryExtractor;

const CRYPTO_DOMAINS: &[&str] = &[
    "metamask",
    "phantom",
    "ledger",
    "trezor",
    "blockchain",
    "binance",
    "coinbase",
    "kraken",
    "bitget",
    "okx",
    "uniswap",
    "pump.fun",
    "solscan",
    "etherscan",
    "blockchair",
    "btcrecover",
    "electrum",
    "mycelium",
    "exodus",
    "atomic",
    "trustwallet",
    "rabby",
    "okxweb3",
];

impl Extractor for ChromeHistoryExtractor {
    fn name(&self) -> &str {
        "chrome_history"
    }

    fn extensions(&self) -> &[&str] {
        // Chrome's file is literally named `History` with no extension.
        // Match both that and explicit .sqlite copies users might rename.
        &["", "sqlite", "db"]
    }

    fn extract(&self, path: &Path) -> Result<Vec<MemoryNode>, ExtractError> {
        let conn = Connection::open(path)?;
        let mut nodes = Vec::new();
        let label = format!("chrome_history:{}", path.display());

        // urls.url is the main visited-page table.
        let mut stmt = match conn.prepare("SELECT url, title FROM urls") {
            Ok(s) => s,
            Err(_) => return Ok(nodes), // Not a Chrome History DB; gracefully no-op.
        };
        let rows = stmt.query_map([], |row| {
            let url: String = row.get(0)?;
            let title: Option<String> = row.get(1).ok();
            Ok((url, title))
        })?;

        for row in rows.flatten() {
            let (url, title) = row;
            let lower = url.to_lowercase();
            if let Some(matched) = CRYPTO_DOMAINS.iter().find(|d| lower.contains(*d)) {
                let content = match &title {
                    Some(t) if !t.trim().is_empty() => format!("{matched}: {t} ({url})"),
                    _ => format!("{matched}: {url}"),
                };
                nodes.push(MemoryNode::new(
                    MemoryNodeKind::ContextualLead,
                    content,
                    0.45,
                    vec![label.clone()],
                ));
            }
        }

        Ok(nodes)
    }
}
