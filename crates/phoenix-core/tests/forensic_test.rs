use phoenix_core::forensic::{
    bip39_text::{self, Bip39TextExtractor},
    bitwarden_csv::BitwardenCsvExtractor,
    chrome_history::ChromeHistoryExtractor,
    Extractor, ExtractorRegistry,
};
use phoenix_core::interview::state::MemoryNodeKind;
use std::io::Write;

#[test]
fn bip39_scan_finds_full_seed_phrase() {
    let phrase =
        "abandon ability able about above absent absorb abstract absurd abuse access accident";
    let nodes = bip39_text::scan(phrase, "test");
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].kind, MemoryNodeKind::SeedFragment);
    assert!(nodes[0].confidence >= 0.9);
}

#[test]
fn bip39_scan_finds_partial_phrase() {
    let mixed = "Hello here is a partial: abandon ability able and then garbage";
    let nodes = bip39_text::scan(mixed, "test");
    assert_eq!(nodes.len(), 1);
    assert_eq!(
        nodes[0].confidence, 0.6,
        "3 consecutive bip39 words = partial confidence"
    );
}

#[test]
fn bip39_scan_ignores_isolated_words() {
    let lonely = "ability is fun but absurd is harder";
    let nodes = bip39_text::scan(lonely, "test");
    assert_eq!(nodes.len(), 0, "isolated bip39 words below threshold");
}

#[test]
fn bip39_scan_handles_punctuation() {
    let punct = "Look at this: abandon, ability, able! And: above above above.";
    let nodes = bip39_text::scan(punct, "test");
    assert!(!nodes.is_empty());
}

#[test]
fn bip39_text_extractor_runs_on_arbitrary_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("notes.txt");
    let mut f = std::fs::File::create(&path).unwrap();
    writeln!(
        f,
        "abandon ability able about above absent absorb abstract absurd abuse access accident"
    )
    .unwrap();

    let nodes = Bip39TextExtractor.extract(&path).unwrap();
    assert_eq!(nodes.len(), 1);
    assert!(nodes[0].confidence >= 0.9);
}

#[test]
fn bitwarden_csv_extracts_passwords_and_notes() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("export.csv");
    let mut f = std::fs::File::create(&path).unwrap();
    writeln!(
        f,
        "folder,favorite,type,name,notes,fields,login_uri,login_username,login_password,login_totp"
    )
    .unwrap();
    writeln!(
        f,
        ",,login,Wallet,seedhint:bluemoon99,,https://example.com,me@example.com,SuperSecret123!,"
    )
    .unwrap();
    writeln!(f, ",,login,EmptyPwd,,,https://x.com,user,,").unwrap();

    let nodes = BitwardenCsvExtractor.extract(&path).unwrap();
    let passwords: Vec<_> = nodes
        .iter()
        .filter(|n| n.kind == MemoryNodeKind::PasswordPattern)
        .collect();
    let notes: Vec<_> = nodes
        .iter()
        .filter(|n| n.kind == MemoryNodeKind::ContextualLead)
        .collect();

    assert_eq!(passwords.len(), 1, "only one row has a password");
    assert_eq!(passwords[0].content, "SuperSecret123!");
    assert_eq!(notes.len(), 1);
    assert!(notes[0].content.contains("bluemoon99"));
}

#[test]
fn chrome_history_extractor_pulls_crypto_domains() {
    use rusqlite::Connection;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("History");
    let conn = Connection::open(&path).unwrap();
    conn.execute(
        "CREATE TABLE urls (id INTEGER PRIMARY KEY, url TEXT, title TEXT)",
        [],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO urls (url, title) VALUES ('https://metamask.io/download', 'MetaMask install')",
        [],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO urls (url, title) VALUES ('https://news.ycombinator.com', 'HN')",
        [],
    )
    .unwrap();
    drop(conn);

    let nodes = ChromeHistoryExtractor.extract(&path).unwrap();
    assert_eq!(nodes.len(), 1);
    assert!(nodes[0].content.to_lowercase().contains("metamask"));
}

#[test]
fn registry_dispatches_csv_to_bitwarden_and_bip39() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("notes.csv");
    let mut f = std::fs::File::create(&path).unwrap();
    writeln!(f, "name,login_password,notes").unwrap();
    writeln!(
        f,
        "wallet,abandon ability able about above absent absorb abstract absurd abuse access accident,seed inside"
    )
    .unwrap();

    let registry = ExtractorRegistry::new()
        .register(BitwardenCsvExtractor)
        .register(Bip39TextExtractor);
    let nodes = registry.dispatch(&path).unwrap();
    // BitwardenCsvExtractor produces password + note; Bip39TextExtractor produces seed.
    assert!(nodes.len() >= 2);
    assert!(nodes.iter().any(|n| n.kind == MemoryNodeKind::SeedFragment));
    assert!(nodes
        .iter()
        .any(|n| n.kind == MemoryNodeKind::PasswordPattern));
}

#[test]
fn registry_errors_on_unsupported_extension_when_no_universal_extractor() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("foo.zzz");
    std::fs::write(&path, b"x").unwrap();
    let registry = ExtractorRegistry::new().register(BitwardenCsvExtractor);
    assert!(registry.dispatch(&path).is_err());
}
