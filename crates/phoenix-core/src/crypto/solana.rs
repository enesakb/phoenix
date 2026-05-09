//! Solana key derivation and address generation.
//!
//! Unlike BTC/ETH which use secp256k1 + BIP-32, Solana uses ed25519 + SLIP-10
//! for hierarchical key derivation, with all path components hardened.
//!
//! Three derivation paths are supported because the Solana wallet ecosystem
//! never converged on a single standard:
//!
//! | Wallet              | Path                  |
//! |---------------------|-----------------------|
//! | Phantom, Backpack   | m/44'/501'/0'/0'      |
//! | Solflare            | m/44'/501'/0'         |
//! | Sollet (legacy)     | m/501'/0'/0/0         |
//!
//! Phoenix tries all three when reconstructing.

use ed25519_dalek::SigningKey;
use slip10_ed25519::derive_ed25519_private_key;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SolanaError {
    #[error("invalid derivation path index: {0}")]
    BadIndex(String),
}

/// Standard Phantom / Backpack / Trust Wallet path for the first account.
pub const PHANTOM_PATH: &[u32] = &[44, 501, 0, 0];
/// Standard Solflare path (no trailing change segment).
pub const SOLFLARE_PATH: &[u32] = &[44, 501, 0];
/// Legacy Sollet wallet path.
pub const SOLLET_PATH: &[u32] = &[501, 0, 0, 0];

/// All three paths Phoenix tries when reconstructing a Solana wallet.
pub const ALL_PATHS: &[&[u32]] = &[PHANTOM_PATH, SOLFLARE_PATH, SOLLET_PATH];

/// Derive an ed25519 signing key from a 64-byte seed and a hardened derivation
/// path. Each `path` element represents the index BEFORE hardening; the
/// SLIP-10 ed25519 spec hardens every component of the path.
pub fn derive_solana_signing_key(seed: &[u8; 64], path: &[u32]) -> SigningKey {
    let derived = derive_ed25519_private_key(seed, path);
    SigningKey::from_bytes(&derived)
}

/// Solana base58 address — public key bytes encoded directly.
pub fn solana_address(signing_key: &SigningKey) -> String {
    let verifying = signing_key.verifying_key();
    bs58::encode(verifying.as_bytes()).into_string()
}

/// Convenience: derive at PHANTOM_PATH and return the address.
pub fn phantom_address(seed: &[u8; 64]) -> String {
    solana_address(&derive_solana_signing_key(seed, PHANTOM_PATH))
}

/// Iterate all three known wallet paths and return the (path-name, address)
/// pairs. Useful for the recovery UX which displays "your seed produces these
/// candidate addresses across the three Solana wallet families."
pub fn all_addresses(seed: &[u8; 64]) -> Vec<(&'static str, String)> {
    vec![
        (
            "Phantom/Backpack",
            solana_address(&derive_solana_signing_key(seed, PHANTOM_PATH)),
        ),
        (
            "Solflare",
            solana_address(&derive_solana_signing_key(seed, SOLFLARE_PATH)),
        ),
        (
            "Sollet (legacy)",
            solana_address(&derive_solana_signing_key(seed, SOLLET_PATH)),
        ),
    ]
}
