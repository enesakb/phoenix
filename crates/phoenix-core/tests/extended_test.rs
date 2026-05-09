use phoenix_core::crypto::{
    address::AddressKind,
    hashcat::{build_command, common_passphrase_seeds, AttackMode, HashcatMode},
    reconstruct::{brute_force_passphrase, reconstruct_multi},
};
use phoenix_core::forensic::{
    mbox::{self, MboxExtractor},
    Extractor,
};
use phoenix_core::interview::state::MemoryNodeKind;
use std::io::Write;
use std::path::PathBuf;

#[test]
fn reconstruct_multi_handles_one_missing() {
    let template =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?";
    let target = "0x9858effd232b4033e47d90003d41ec34ecaeda94";
    let result = reconstruct_multi(template, target, AddressKind::Eth, "", 1).unwrap();
    assert_eq!(result.recovered_words, vec!["about"]);
}

#[test]
fn reconstruct_multi_two_missing_eth() {
    // Two missing positions. Brute force should converge; allow 60s budget.
    let template =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ? ?";
    let target = "0x9858effd232b4033e47d90003d41ec34ecaeda94";
    let result = reconstruct_multi(template, target, AddressKind::Eth, "", 1).unwrap();
    assert_eq!(result.recovered_words.len(), 2);
    assert!(result.recovered_mnemonic.ends_with("abandon about"));
}

#[test]
fn reconstruct_multi_rejects_three_missing() {
    let template = "abandon abandon abandon abandon abandon abandon abandon abandon abandon ? ? ?";
    let target = "0x9858effd232b4033e47d90003d41ec34ecaeda94";
    let result = reconstruct_multi(template, target, AddressKind::Eth, "", 1);
    assert!(result.is_err());
}

#[test]
fn brute_force_passphrase_finds_known_passphrase() {
    // "abandon × 11 about" + passphrase "TREZOR" → known seed
    // Derive the actual address Phoenix produces with that passphrase.
    use phoenix_core::crypto::{
        address::eth_address, derive::derive_eth_key, mnemonic::mnemonic_to_seed,
    };
    let mnemonic =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let pp = "TREZOR";
    let seed = mnemonic_to_seed(mnemonic, pp).unwrap();
    let (_, pk) = derive_eth_key(&seed, 0).unwrap();
    let target = eth_address(&pk);

    let candidates = common_passphrase_seeds();
    let result =
        brute_force_passphrase(mnemonic, &target, AddressKind::Eth, &candidates, 1).unwrap();
    assert_eq!(result.passphrase, "TREZOR");
}

#[test]
fn brute_force_passphrase_returns_no_match_for_unknown() {
    let mnemonic =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let target = "0x0000000000000000000000000000000000000000";
    let candidates = vec!["foo".to_string(), "bar".to_string()];
    let result = brute_force_passphrase(mnemonic, target, AddressKind::Eth, &candidates, 1);
    assert!(result.is_err());
}

#[test]
fn hashcat_command_includes_mode_and_attack() {
    let path = PathBuf::from("/tmp/wallet.hash");
    let cmd = build_command(
        &path,
        HashcatMode::BitcoinWalletDat,
        AttackMode::Straight,
        "rockyou.txt",
    );
    assert!(cmd.contains("-m 11300"));
    assert!(cmd.contains("-a 0"));
    assert!(cmd.contains("rockyou.txt"));
}

#[test]
fn common_passphrase_seeds_includes_empty_and_trezor() {
    let list = common_passphrase_seeds();
    assert!(list.iter().any(|s| s.is_empty()));
    assert!(list.iter().any(|s| s == "TREZOR"));
}

#[test]
fn mbox_scan_pulls_subject_keyword_lead() {
    let raw = "From a@b 2026 Jan 1\nSubject: My Bitcoin wallet backup\n\nbody text\n\nFrom b@c\nSubject: Lunch plans\n\nirrelevant body\n";
    let nodes = mbox::scan_mbox(raw, "test.mbox");
    let leads: Vec<_> = nodes
        .iter()
        .filter(|n| n.kind == MemoryNodeKind::ContextualLead)
        .collect();
    assert_eq!(leads.len(), 1);
    assert!(leads[0].content.to_lowercase().contains("bitcoin"));
}

#[test]
fn mbox_scan_finds_seed_phrase_in_body() {
    let raw = "From a@b\nSubject: notes\n\nabandon ability able about above absent absorb abstract absurd abuse access accident\n";
    let nodes = mbox::scan_mbox(raw, "test.mbox");
    assert!(nodes.iter().any(|n| n.kind == MemoryNodeKind::SeedFragment));
}

#[test]
fn mbox_extractor_reads_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("inbox.mbox");
    let mut f = std::fs::File::create(&path).unwrap();
    writeln!(
        f,
        "From a@b\nSubject: Wallet seed reminder\n\nabandon ability able about above absent absorb abstract absurd abuse access accident"
    )
    .unwrap();

    let nodes = MboxExtractor.extract(&path).unwrap();
    assert!(nodes.iter().any(|n| n.kind == MemoryNodeKind::SeedFragment));
    assert!(nodes
        .iter()
        .any(|n| n.kind == MemoryNodeKind::ContextualLead));
}
