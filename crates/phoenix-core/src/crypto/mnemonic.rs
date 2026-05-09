use bip39::{Language, Mnemonic};
use thiserror::Error;

/// Generate a fresh BIP-39 mnemonic using the OS RNG. The output is a 12-word
/// English phrase suitable for use as a wallet seed. The caller is responsible
/// for recording it securely; Phoenix never persists it.
pub fn generate_fresh_mnemonic() -> String {
    use rand::RngCore;
    let mut entropy = [0u8; 16]; // 128 bits → 12 words
    rand::rngs::OsRng.fill_bytes(&mut entropy);
    Mnemonic::from_entropy_in(Language::English, &entropy)
        .expect("OsRng output is valid 128-bit entropy")
        .to_string()
}

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
