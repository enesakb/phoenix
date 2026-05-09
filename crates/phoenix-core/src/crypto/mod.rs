//! Cryptographic core — Layer 5 of the Phoenix recovery pipeline.
//!
//! Given a partial mnemonic and a target on-chain address, derive candidate
//! seeds, compute addresses across standard BIP-44 paths, and report the
//! match. Pure-Rust, deterministic, side-effect-free — no network calls
//! happen anywhere in this module.

pub mod address;
pub mod derive;
pub mod mnemonic;
pub mod reconstruct;

pub use address::{btc_p2wpkh_address, eth_address, AddressKind};
pub use derive::{derive_btc_segwit_key, derive_eth_key, DerivationError};
pub use mnemonic::{is_valid_mnemonic, mnemonic_to_seed, MnemonicError};
pub use reconstruct::{reconstruct_missing_word, ReconstructError, ReconstructResult};
