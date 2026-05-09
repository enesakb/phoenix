use rayon::prelude::*;
use thiserror::Error;

use super::address::{btc_p2wpkh_address, eth_address, AddressKind};
use super::derive::{derive_btc_segwit_key, derive_eth_key};
use super::mnemonic::{is_valid_mnemonic, mnemonic_to_seed};
use crate::forensic::bip39_wordlist;

#[derive(Debug, Error)]
pub enum ReconstructError {
    #[error("expected 12 tokens (with one or more '?' wildcards); got {0}")]
    BadInput(usize),
    #[error("missing-word position must be 0..=11; got {0}")]
    BadPosition(usize),
    #[error("too many missing words: {0} (max 2 supported in pure-Rust mode)")]
    TooManyMissing(usize),
    #[error("address index search range exceeded")]
    NoMatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReconstructResult {
    pub recovered_word: String,
    pub recovered_mnemonic: String,
    pub address_index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiReconstructResult {
    pub recovered_words: Vec<String>,
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

/// Brute-force up to two missing words. With one missing it delegates; with
/// two missing it does a parallelized cartesian product (≈ 262k checksum-valid
/// candidates × N indexes; 6-10s on 8-core CPU).
pub fn reconstruct_multi(
    template: &str,
    target_address: &str,
    kind: AddressKind,
    passphrase: &str,
    address_index_range: u32,
) -> Result<MultiReconstructResult, ReconstructError> {
    let tokens: Vec<&str> = template.split_whitespace().collect();
    if tokens.len() != 12 {
        return Err(ReconstructError::BadInput(tokens.len()));
    }
    let missing: Vec<usize> = tokens
        .iter()
        .enumerate()
        .filter_map(|(i, t)| if *t == "?" { Some(i) } else { None })
        .collect();
    match missing.len() {
        0 => Err(ReconstructError::BadInput(0)),
        1 => {
            let single = reconstruct_missing_word(
                template,
                target_address,
                kind,
                passphrase,
                address_index_range,
            )?;
            Ok(MultiReconstructResult {
                recovered_words: vec![single.recovered_word],
                recovered_mnemonic: single.recovered_mnemonic,
                address_index: single.address_index,
            })
        }
        2 => {
            let target = target_address.trim().to_lowercase();
            let words = bip39_wordlist();
            let pos_a = missing[0];
            let pos_b = missing[1];

            let hit = words.par_iter().find_map_any(|w_a| {
                for w_b in words.iter() {
                    let mut filled: Vec<&str> = tokens.to_vec();
                    filled[pos_a] = w_a;
                    filled[pos_b] = w_b;
                    let mnemonic_text = filled.join(" ");
                    if !is_valid_mnemonic(&mnemonic_text) {
                        continue;
                    }
                    let Ok(seed) = mnemonic_to_seed(&mnemonic_text, passphrase) else {
                        continue;
                    };
                    for i in 0..address_index_range {
                        let derived = match kind {
                            AddressKind::BtcSegwit => derive_btc_segwit_key(&seed, i).ok(),
                            AddressKind::Eth => derive_eth_key(&seed, i).ok(),
                        };
                        let Some((_, pk)) = derived else { continue };
                        let addr = match kind {
                            AddressKind::BtcSegwit => btc_p2wpkh_address(&pk),
                            AddressKind::Eth => eth_address(&pk),
                        };
                        if addr.to_lowercase() == target {
                            return Some(MultiReconstructResult {
                                recovered_words: vec![(*w_a).to_string(), (*w_b).to_string()],
                                recovered_mnemonic: mnemonic_text.clone(),
                                address_index: i,
                            });
                        }
                    }
                }
                None
            });
            hit.ok_or(ReconstructError::NoMatch)
        }
        n => Err(ReconstructError::TooManyMissing(n)),
    }
}

/// Brute-force a forgotten BIP-39 passphrase ("25th word") given a complete
/// 12-word mnemonic and a target on-chain address. The user supplies the
/// candidate passphrases (typically derived from their patterns identified
/// in the cognitive interview).
pub fn brute_force_passphrase(
    mnemonic: &str,
    target_address: &str,
    kind: AddressKind,
    candidates: &[String],
    address_index_range: u32,
) -> Result<PassphraseResult, ReconstructError> {
    if !is_valid_mnemonic(mnemonic) {
        return Err(ReconstructError::BadInput(0));
    }
    let target = target_address.trim().to_lowercase();
    let mnemonic_owned = mnemonic.to_string();

    let hit = candidates.par_iter().find_map_any(|pp| {
        let seed = mnemonic_to_seed(&mnemonic_owned, pp).ok()?;
        for i in 0..address_index_range {
            let derived = match kind {
                AddressKind::BtcSegwit => derive_btc_segwit_key(&seed, i).ok(),
                AddressKind::Eth => derive_eth_key(&seed, i).ok(),
            };
            let Some((_, pk)) = derived else { continue };
            let addr = match kind {
                AddressKind::BtcSegwit => btc_p2wpkh_address(&pk),
                AddressKind::Eth => eth_address(&pk),
            };
            if addr.to_lowercase() == target {
                return Some(PassphraseResult {
                    passphrase: pp.clone(),
                    address_index: i,
                });
            }
        }
        None
    });

    hit.ok_or(ReconstructError::NoMatch)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassphraseResult {
    pub passphrase: String,
    pub address_index: u32,
}
