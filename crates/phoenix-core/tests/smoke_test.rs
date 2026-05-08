use phoenix_core::version;

#[test]
fn version_matches_cargo_manifest() {
    assert_eq!(version(), "0.1.0");
}
