use phoenix_core::crypto::{
    address::AddressKind,
    mnemonic::mnemonic_to_seed,
    reconstruct::reconstruct_missing_word,
    solana::{
        all_addresses, derive_solana_signing_key, phantom_address, solana_address, ALL_PATHS,
        PHANTOM_PATH, SOLFLARE_PATH, SOLLET_PATH,
    },
};

const TEST_MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

#[test]
fn solana_derivation_is_deterministic() {
    let seed = mnemonic_to_seed(TEST_MNEMONIC, "").unwrap();
    let addr1 = phantom_address(&seed);
    let addr2 = phantom_address(&seed);
    assert_eq!(addr1, addr2, "derivation must be deterministic");
}

#[test]
fn solana_address_is_valid_base58() {
    let seed = mnemonic_to_seed(TEST_MNEMONIC, "").unwrap();
    let addr = phantom_address(&seed);
    assert!(!addr.is_empty());
    let decoded = bs58::decode(&addr)
        .into_vec()
        .expect("address must be valid base58");
    assert_eq!(decoded.len(), 32, "Solana addresses are 32 bytes");
}

#[test]
fn three_paths_produce_distinct_addresses() {
    let seed = mnemonic_to_seed(TEST_MNEMONIC, "").unwrap();
    let phantom = solana_address(&derive_solana_signing_key(&seed, PHANTOM_PATH));
    let solflare = solana_address(&derive_solana_signing_key(&seed, SOLFLARE_PATH));
    let sollet = solana_address(&derive_solana_signing_key(&seed, SOLLET_PATH));
    // Each derivation path should yield a different address (different derived
    // private key under SLIP-10 ed25519).
    assert_ne!(phantom, solflare);
    assert_ne!(phantom, sollet);
    assert_ne!(solflare, sollet);
}

#[test]
fn all_addresses_returns_three_pairs() {
    let seed = mnemonic_to_seed(TEST_MNEMONIC, "").unwrap();
    let pairs = all_addresses(&seed);
    assert_eq!(pairs.len(), 3);
    assert_eq!(pairs[0].0, "Phantom/Backpack");
    assert_eq!(pairs[1].0, "Solflare");
    assert_eq!(pairs[2].0, "Sollet (legacy)");
}

#[test]
fn all_paths_constant_matches_individual_constants() {
    assert_eq!(ALL_PATHS.len(), 3);
    assert_eq!(ALL_PATHS[0], PHANTOM_PATH);
    assert_eq!(ALL_PATHS[1], SOLFLARE_PATH);
    assert_eq!(ALL_PATHS[2], SOLLET_PATH);
}

/// Round-trip: derive a Solana address from the test mnemonic, then prove
/// `reconstruct_missing_word` can recover the missing 12th word using that
/// derived address as target. This validates that the reconstruct pipeline
/// understands Solana correctly, independent of whether our derivation
/// matches Phantom.
#[test]
fn reconstructs_missing_word_via_solana_phantom_path() {
    let seed = mnemonic_to_seed(TEST_MNEMONIC, "").unwrap();
    let target_address = phantom_address(&seed);

    let template =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?";
    let result =
        reconstruct_missing_word(template, &target_address, AddressKind::Solana, "", 1).unwrap();
    assert_eq!(result.recovered_word, "about");
    // Solana stores the matching path index in `address_index`. PHANTOM_PATH
    // is index 0 in ALL_PATHS.
    assert_eq!(result.address_index, 0);
}

#[test]
fn reconstructs_missing_word_via_solana_solflare_path() {
    let seed = mnemonic_to_seed(TEST_MNEMONIC, "").unwrap();
    let target_address = solana_address(&derive_solana_signing_key(&seed, SOLFLARE_PATH));

    let template =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?";
    let result =
        reconstruct_missing_word(template, &target_address, AddressKind::Solana, "", 1).unwrap();
    assert_eq!(result.recovered_word, "about");
    assert_eq!(result.address_index, 1, "Solflare is index 1 in ALL_PATHS");
}

/// Locked vectors — exact addresses produced by the canonical 'abandon × 11
/// about' BIP-39 zero vector. Cross-verified against an independent Node.js
/// pipeline using the libraries Phantom itself ships with:
///   - bip39 (Node.js)
///   - ed25519-hd-key
///   - tweetnacl
///   - bs58
/// All three derivation paths produced byte-identical addresses across the
/// two implementations on 2026-05-09. If this test ever fails, someone has
/// changed the derivation in a way that diverges from the wider Solana
/// ecosystem — investigate before merging.
#[test]
fn locked_vectors_match_phantom_libs() {
    let seed = mnemonic_to_seed(TEST_MNEMONIC, "").unwrap();
    let phantom = solana_address(&derive_solana_signing_key(&seed, PHANTOM_PATH));
    let solflare = solana_address(&derive_solana_signing_key(&seed, SOLFLARE_PATH));
    let sollet = solana_address(&derive_solana_signing_key(&seed, SOLLET_PATH));

    assert_eq!(phantom, "HAgk14JpMQLgt6rVgv7cBQFJWFto5Dqxi472uT3DKpqk");
    assert_eq!(solflare, "GjJyeC1r2RgkuoCWMyPYkCWSGSGLcz266EaAkLA27AhL");
    assert_eq!(sollet, "DaYoLHpp7RRyAqn1HBPZYZpsKVEAmCDWemW18GABpT5");
}

#[test]
fn solana_reconstruct_returns_no_match_for_unknown_address() {
    let template =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?";
    // 32 zero-bytes encoded as base58 — definitively not a real address.
    let zero_address = bs58::encode([0u8; 32]).into_string();
    let result = reconstruct_missing_word(template, &zero_address, AddressKind::Solana, "", 1);
    assert!(result.is_err());
}
