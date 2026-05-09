//! End-to-end stress tests for the recovery pipeline.
//!
//! These exercise the full reconstruction pipeline on multiple distinct
//! mnemonics across BTC, ETH, and Solana, with the missing word in different
//! positions. They are slower than the unit tests (each one runs a real
//! Rayon-parallel brute-force loop) but catch regressions the unit tests miss.

use phoenix_core::crypto::{
    address::{btc_p2wpkh_address, eth_address, AddressKind},
    derive::{derive_btc_segwit_key, derive_eth_key},
    mnemonic::mnemonic_to_seed,
    reconstruct::{brute_force_passphrase, reconstruct_missing_word, reconstruct_multi},
    solana::{phantom_address, solana_address, derive_solana_signing_key, SOLFLARE_PATH},
};

/// Five publicly-known BIP-39 test vectors. None of these have ever held real
/// funds; they are documented in BIP-39, Trezor docs, and various crypto books.
const VECTORS: &[&str] = &[
    // BIP-39 zero vector (the most-used cross-implementation reference)
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
    // Trezor docs example
    "legal winner thank year wave sausage worth useful legal winner thank yellow",
    // BIP-39 spec example
    "letter advice cage absurd amount doctor acoustic avoid letter advice cage above",
    // Common test fixture (Ledger docs)
    "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong",
    // Iancoleman.io default
    "satoshi satoshi satoshi satoshi satoshi satoshi satoshi satoshi satoshi satoshi satoshi satoshi",
];

fn replace_word(mnemonic: &str, position: usize, replacement: &str) -> String {
    mnemonic
        .split_whitespace()
        .enumerate()
        .map(|(i, w)| if i == position { replacement.to_string() } else { w.to_string() })
        .collect::<Vec<_>>()
        .join(" ")
}

fn last_word(mnemonic: &str) -> String {
    mnemonic.split_whitespace().last().unwrap().to_string()
}

fn first_word(mnemonic: &str) -> String {
    mnemonic.split_whitespace().next().unwrap().to_string()
}

fn middle_word(mnemonic: &str) -> String {
    mnemonic.split_whitespace().nth(5).unwrap().to_string()
}

#[test]
fn stress_eth_recovery_across_five_distinct_mnemonics() {
    for &mnemonic in VECTORS {
        // Skip vectors with invalid checksums (some test fixtures are deliberately invalid).
        let Ok(seed) = mnemonic_to_seed(mnemonic, "") else {
            continue;
        };
        let Ok((_, pk)) = derive_eth_key(&seed, 0) else {
            continue;
        };
        let target = eth_address(&pk);

        // Try missing last word.
        let template = replace_word(mnemonic, 11, "?");
        let result =
            reconstruct_missing_word(&template, &target, AddressKind::Eth, "", 1).unwrap();
        assert_eq!(
            result.recovered_word,
            last_word(mnemonic),
            "ETH recovery failed for: {mnemonic}"
        );
    }
}

#[test]
fn stress_recovery_at_first_middle_last_positions() {
    let mnemonic = VECTORS[0];
    let seed = mnemonic_to_seed(mnemonic, "").unwrap();
    let (_, pk) = derive_eth_key(&seed, 0).unwrap();
    let target = eth_address(&pk);

    // Position 0 (first word)
    let t1 = replace_word(mnemonic, 0, "?");
    let r1 = reconstruct_missing_word(&t1, &target, AddressKind::Eth, "", 1).unwrap();
    assert_eq!(r1.recovered_word, first_word(mnemonic));

    // Position 5 (middle)
    let t2 = replace_word(mnemonic, 5, "?");
    let r2 = reconstruct_missing_word(&t2, &target, AddressKind::Eth, "", 1).unwrap();
    assert_eq!(r2.recovered_word, middle_word(mnemonic));

    // Position 11 (last)
    let t3 = replace_word(mnemonic, 11, "?");
    let r3 = reconstruct_missing_word(&t3, &target, AddressKind::Eth, "", 1).unwrap();
    assert_eq!(r3.recovered_word, last_word(mnemonic));
}

#[test]
fn stress_btc_recovery_across_distinct_mnemonics() {
    for &mnemonic in VECTORS.iter().take(3) {
        let Ok(seed) = mnemonic_to_seed(mnemonic, "") else {
            continue;
        };
        let Ok((_, pk)) = derive_btc_segwit_key(&seed, 0) else {
            continue;
        };
        let target = btc_p2wpkh_address(&pk);

        let template = replace_word(mnemonic, 11, "?");
        let result =
            reconstruct_missing_word(&template, &target, AddressKind::BtcSegwit, "", 1).unwrap();
        assert_eq!(result.recovered_word, last_word(mnemonic));
    }
}

#[test]
fn stress_solana_recovery_across_distinct_mnemonics() {
    for &mnemonic in VECTORS.iter().take(3) {
        let Ok(seed) = mnemonic_to_seed(mnemonic, "") else {
            continue;
        };
        let target = phantom_address(&seed);

        let template = replace_word(mnemonic, 11, "?");
        let result =
            reconstruct_missing_word(&template, &target, AddressKind::Solana, "", 1).unwrap();
        assert_eq!(result.recovered_word, last_word(mnemonic));
        // Phantom path is index 0
        assert_eq!(result.address_index, 0);
    }
}

#[test]
fn stress_solana_solflare_path_specifically() {
    let mnemonic = VECTORS[0];
    let seed = mnemonic_to_seed(mnemonic, "").unwrap();
    let target = solana_address(&derive_solana_signing_key(&seed, SOLFLARE_PATH));

    let template = replace_word(mnemonic, 11, "?");
    let result =
        reconstruct_missing_word(&template, &target, AddressKind::Solana, "", 1).unwrap();
    assert_eq!(result.recovered_word, last_word(mnemonic));
    assert_eq!(result.address_index, 1, "Solflare is index 1 in ALL_PATHS");
}

#[test]
fn stress_passphrase_brute_force_eth() {
    // mnemonic + known passphrase combination → derive target → recover passphrase
    let mnemonic = VECTORS[0];
    let known_passphrase = "TREZOR"; // standard BIP-39 spec example passphrase
    let seed = mnemonic_to_seed(mnemonic, known_passphrase).unwrap();
    let (_, pk) = derive_eth_key(&seed, 0).unwrap();
    let target = eth_address(&pk);

    let candidates = vec![
        "wrong".to_string(),
        "incorrect".to_string(),
        "TREZOR".to_string(), // the actual one
        "extra".to_string(),
    ];
    let result =
        brute_force_passphrase(mnemonic, &target, AddressKind::Eth, &candidates, 1).unwrap();
    assert_eq!(result.passphrase, "TREZOR");
}

#[test]
fn stress_two_missing_word_recovery_eth() {
    let mnemonic = VECTORS[0];
    let seed = mnemonic_to_seed(mnemonic, "").unwrap();
    let (_, pk) = derive_eth_key(&seed, 0).unwrap();
    let target = eth_address(&pk);

    // Two adjacent missing words at the end
    let template =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ? ?";
    let started = std::time::Instant::now();
    let result = reconstruct_multi(template, &target, AddressKind::Eth, "", 1).unwrap();
    let elapsed = started.elapsed();

    assert_eq!(result.recovered_words.len(), 2);
    assert!(result.recovered_mnemonic.ends_with("abandon about"));
    // Should converge in well under 60 seconds even in debug mode.
    assert!(
        elapsed < std::time::Duration::from_secs(60),
        "two-missing-word recovery took {:?}, expected <60s",
        elapsed
    );
}

#[test]
fn stress_recovery_is_deterministic_repeat_10x() {
    let mnemonic = VECTORS[0];
    let seed = mnemonic_to_seed(mnemonic, "").unwrap();
    let (_, pk) = derive_eth_key(&seed, 0).unwrap();
    let target = eth_address(&pk);
    let template =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?";

    for i in 0..10 {
        let result =
            reconstruct_missing_word(template, &target, AddressKind::Eth, "", 1).unwrap();
        assert_eq!(
            result.recovered_word, "about",
            "iteration {i}: result is non-deterministic"
        );
    }
}

#[test]
fn stress_mismatched_kind_does_not_falsely_succeed() {
    // Use ETH target but ask Phoenix to derive Solana. Should fail.
    let mnemonic = VECTORS[0];
    let seed = mnemonic_to_seed(mnemonic, "").unwrap();
    let (_, pk) = derive_eth_key(&seed, 0).unwrap();
    let eth_target = eth_address(&pk);
    let template = replace_word(mnemonic, 11, "?");

    // Should not match because we're deriving Solana addresses but target is ETH-formatted.
    let result =
        reconstruct_missing_word(&template, &eth_target, AddressKind::Solana, "", 1);
    assert!(
        result.is_err(),
        "ETH target with Solana derivation must not produce a false positive"
    );
}
