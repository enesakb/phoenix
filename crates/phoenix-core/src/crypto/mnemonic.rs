use bip39::{Language, Mnemonic};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MnemonicError {
    #[error("invalid mnemonic: {0}")]
    Invalid(String),
}

pub fn is_valid_mnemonic(words: &str) -> bool {
    Mnemonic::parse_in(Language::English, words).is_ok()
}

/// Mnemonic + optional BIP-39 passphrase → 64-byte seed.
pub fn mnemonic_to_seed(words: &str, passphrase: &str) -> Result<[u8; 64], MnemonicError> {
    let mnemonic = Mnemonic::parse_in(Language::English, words)
        .map_err(|e| MnemonicError::Invalid(e.to_string()))?;
    Ok(mnemonic.to_seed(passphrase))
}
