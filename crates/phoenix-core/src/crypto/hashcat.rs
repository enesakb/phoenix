//! Hashcat command builder.
//!
//! Phoenix does not run Hashcat itself (different deployment model, GPU drivers,
//! etc.). Instead we generate the precise command line the user can paste into
//! their cracking rig. Each enum variant corresponds to a Hashcat hash mode
//! defined in the upstream wiki.

use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashcatMode {
    /// Bitcoin/Litecoin wallet.dat
    BitcoinWalletDat = 11300,
    /// Blockchain, My Wallet (V1)
    BlockchainV1 = 12700,
    /// MultiBit Classic .key (MD5 AES)
    MultiBitKey = 22500,
    /// Electrum Wallet (Salt-Type 1)
    Electrum1 = 21700,
    /// Electrum Wallet (Salt-Type 2)
    Electrum2 = 21800,
    /// MyEtherWallet, V2 (PBKDF2-HMAC-SHA256)
    MewV2 = 15700,
    /// Bitcoin Core (BIP38)
    Bip38 = 27800,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackMode {
    /// Straight dictionary
    Straight = 0,
    /// Combination
    Combination = 1,
    /// Brute-force / mask
    Mask = 3,
}

/// Build a Hashcat command line.
///
/// `hash_file` is the path to the extracted hash blob (e.g. via Hashcat's
/// `bitcoin2john.py` for wallet.dat). `wordlist_or_mask` is either a wordlist
/// path (for Straight) or a Hashcat mask string (for Mask).
pub fn build_command(
    hash_file: &Path,
    mode: HashcatMode,
    attack: AttackMode,
    wordlist_or_mask: &str,
) -> String {
    format!(
        "hashcat -m {} -a {} {:?} {} --status --status-timer 30",
        mode as u32, attack as u32, hash_file, wordlist_or_mask
    )
}

/// Curated list of common BIP-39 passphrase patterns to try first. Drawn from
/// rockyou-style password-leak studies; keep short to amortize the cost on
/// the user's local CPU before falling back to GPU offload.
pub fn common_passphrase_seeds() -> Vec<String> {
    [
        "",
        "password",
        "Password",
        "Password1",
        "Password123",
        "qwerty",
        "qwerty123",
        "letmein",
        "welcome",
        "secret",
        "wallet",
        "bitcoin",
        "satoshi",
        "ethereum",
        "metamask",
        "ledger",
        "trezor",
        "phantom",
        "12345",
        "123456",
        "1234567",
        "12345678",
        "abc123",
        "iloveyou",
        "monkey",
        "dragon",
        "whatever",
        "trustno1",
        "BIP39",
        "bip39",
        "TREZOR",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}
