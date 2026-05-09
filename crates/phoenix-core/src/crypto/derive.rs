use bip32::{DerivationPath, XPrv};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DerivationError {
    #[error("bip32: {0}")]
    Bip32(#[from] bip32::Error),
    #[error("invalid path: {0}")]
    InvalidPath(String),
    #[error("secp256k1: {0}")]
    Secp(#[from] secp256k1::Error),
}

/// Derive an EC key for an arbitrary BIP-32 path from a 64-byte seed.
pub fn derive_secp256k1_key(
    seed: &[u8; 64],
    path_str: &str,
) -> Result<(SecretKey, PublicKey), DerivationError> {
    let xprv = XPrv::derive_from_path(
        seed,
        &path_str
            .parse::<DerivationPath>()
            .map_err(|e| DerivationError::InvalidPath(e.to_string()))?,
    )?;
    let sk = SecretKey::from_slice(&xprv.private_key().to_bytes())?;
    let secp = Secp256k1::new();
    let pk = PublicKey::from_secret_key(&secp, &sk);
    Ok((sk, pk))
}

/// Standard BTC native segwit (P2WPKH, BIP-84) account 0, address index 0.
pub fn derive_btc_segwit_key(
    seed: &[u8; 64],
    address_index: u32,
) -> Result<(SecretKey, PublicKey), DerivationError> {
    let path = format!("m/84'/0'/0'/0/{address_index}");
    derive_secp256k1_key(seed, &path)
}

/// Standard Ethereum BIP-44 path (m/44'/60'/0'/0/i).
pub fn derive_eth_key(
    seed: &[u8; 64],
    address_index: u32,
) -> Result<(SecretKey, PublicKey), DerivationError> {
    let path = format!("m/44'/60'/0'/0/{address_index}");
    derive_secp256k1_key(seed, &path)
}
