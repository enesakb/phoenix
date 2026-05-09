use rayon::prelude::*;
use thiserror::Error;

use super::address::{btc_p2wpkh_address, eth_address, AddressKind};
use super::derive::{derive_btc_segwit_key, derive_eth_key};
use super::mnemonic::{is_valid_mnemonic, mnemonic_to_seed};
use crate::forensic::bip39_wordlist;

#[derive(Debug, Error)]
pub enum ReconstructError {
    #[error("expected 12 tokens (with one '?' wildcard); got {0}")]
    BadInput(usize),
    #[error("missing-word position must be 0..=11; got {0}")]
    BadPosition(usize),
    #[error("address index search range exceeded")]
    NoMatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReconstructResult {
    pub recovered_word: String,
    pub recovered_mnemonic: String,
    pub address_index: u32,
}

/// Brute-force the single missing word of a 12-word BIP-39 mnemonic.
///
/// Inputs:
/// - `template`: 12 space-separated tokens; the missing position is `?`.
/// - `target_address`: lowercase address (BTC bech32 or ETH 0x...).
/// - `kind`: which derivation + address algorithm to test.
/// - `passphrase`: optional BIP-39 passphrase (empty if none).
/// - `address_index_range`: derivation indexes to scan per candidate (0..N).
pub fn reconstruct_missing_word(
    template: &str,
    target_address: &str,
    kind: AddressKind,
    passphrase: &str,
    address_index_range: u32,
) -> Result<ReconstructResult, ReconstructError> {
    let tokens: Vec<&str> = template.split_whitespace().collect();
    if tokens.len() != 12 {
        return Err(ReconstructError::BadInput(tokens.len()));
    }
    let missing_pos = tokens
        .iter()
        .position(|t| *t == "?")
        .ok_or(ReconstructError::BadInput(tokens.len()))?;
    if missing_pos >= 12 {
        return Err(ReconstructError::BadPosition(missing_pos));
    }

    let target = target_address.trim().to_lowercase();
    let words = bip39_wordlist();

    let hit = words.par_iter().find_map_any(|candidate| {
        let mut filled: Vec<&str> = tokens.to_vec();
        filled[missing_pos] = candidate;
        let mnemonic_text = filled.join(" ");
        if !is_valid_mnemonic(&mnemonic_text) {
            return None;
        }
        let seed = mnemonic_to_seed(&mnemonic_text, passphrase).ok()?;
        for i in 0..address_index_range {
            let derived = match kind {
                AddressKind::BtcSegwit => derive_btc_segwit_key(&seed, i).ok(),
                AddressKind::Eth => derive_eth_key(&seed, i).ok(),
            };
            let Some((_, pk)) = derived else { continue };
            let derived_addr = match kind {
                AddressKind::BtcSegwit => btc_p2wpkh_address(&pk),
                AddressKind::Eth => eth_address(&pk),
            };
            if derived_addr.to_lowercase() == target {
                return Some(ReconstructResult {
                    recovered_word: (*candidate).to_string(),
                    recovered_mnemonic: mnemonic_text.clone(),
                    address_index: i,
                });
            }
        }
        None
    });

    hit.ok_or(ReconstructError::NoMatch)
}
