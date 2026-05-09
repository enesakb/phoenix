use phoenix_core::crypto::{
    address::{btc_p2wpkh_address, eth_address, AddressKind},
    derive::{derive_btc_segwit_key, derive_eth_key},
    mnemonic::{is_valid_mnemonic, mnemonic_to_seed},
    reconstruct::reconstruct_missing_word,
};

const TEST_MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

#[test]
fn validates_known_good_mnemonic() {
    assert!(is_valid_mnemonic(TEST_MNEMONIC));
}

#[test]
fn rejects_bad_checksum() {
    let bad = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon";
    assert!(!is_valid_mnemonic(bad));
}

#[test]
fn rejects_non_wordlist_token() {
    let bad =
        "zzz abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    assert!(!is_valid_mnemonic(bad));
}

/// Standard BIP-39 test vector: empty passphrase.
#[test]
fn test_vector_seed_no_passphrase() {
    let seed = mnemonic_to_seed(TEST_MNEMONIC, "").unwrap();
    let expected = hex::decode(
        "5eb00bbddcf069084889a8ab9155568165f5c453ccb85e70811aaed6f6da5fc19a5ac40b389cd370d086206dec8aa6c43daea6690f20ad3d8d48b2d2ce9e38e4",
    )
    .unwrap();
    assert_eq!(&seed[..], &expected[..], "BIP-39 standard vector mismatch");
}

#[test]
fn derives_known_eth_address() {
    let seed = mnemonic_to_seed(TEST_MNEMONIC, "").unwrap();
    let (_, pk) = derive_eth_key(&seed, 0).unwrap();
    let addr = eth_address(&pk);
    // Known address for the BIP-39 zero-vector mnemonic at m/44'/60'/0'/0/0.
    assert_eq!(
        addr.to_lowercase(),
        "0x9858effd232b4033e47d90003d41ec34ecaeda94"
    );
}

#[test]
fn derives_known_btc_segwit_address() {
    let seed = mnemonic_to_seed(TEST_MNEMONIC, "").unwrap();
    let (_, pk) = derive_btc_segwit_key(&seed, 0).unwrap();
    let addr = btc_p2wpkh_address(&pk);
    // Known native segwit address at m/84'/0'/0'/0/0.
    assert_eq!(addr, "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu");
}

#[test]
fn reconstructs_missing_last_word_eth() {
    // 11 known + 1 missing (last word "?")
    let template =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?";
    let target = "0x9858effd232b4033e47d90003d41ec34ecaeda94";
    let result = reconstruct_missing_word(template, target, AddressKind::Eth, "", 1).unwrap();
    assert_eq!(result.recovered_word, "about");
    assert_eq!(result.address_index, 0);
    assert!(result.recovered_mnemonic.contains("about"));
}

#[test]
fn reconstructs_missing_middle_word_btc() {
    // Replace word at index 5 (zero-based) with '?'.
    let template =
        "abandon abandon abandon abandon abandon ? abandon abandon abandon abandon abandon about";
    let target = "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu";
    let result = reconstruct_missing_word(template, target, AddressKind::BtcSegwit, "", 1).unwrap();
    assert_eq!(result.recovered_word, "abandon");
}

#[test]
fn reconstruct_returns_no_match_for_wrong_target() {
    let template =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?";
    let target = "0x0000000000000000000000000000000000000000";
    let result = reconstruct_missing_word(template, target, AddressKind::Eth, "", 1);
    assert!(result.is_err());
}

#[test]
fn reconstruct_rejects_template_without_wildcard() {
    let template = TEST_MNEMONIC;
    let target = "0x9858effd232b4033e47d90003d41ec34ecaeda94";
    let result = reconstruct_missing_word(template, target, AddressKind::Eth, "", 1);
    assert!(result.is_err());
}

#[test]
fn reconstruct_rejects_wrong_token_count() {
    let template = "abandon abandon ?";
    let target = "0x0";
    let result = reconstruct_missing_word(template, target, AddressKind::Eth, "", 1);
    assert!(result.is_err());
}
