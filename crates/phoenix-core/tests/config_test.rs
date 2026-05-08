use phoenix_core::config::{Config, TelemetryConfig};
use std::io::Write;

#[test]
fn loads_default_config_when_file_missing() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("phoenix-config.toml");

    let cfg = Config::load_or_default(&path).unwrap();

    assert_eq!(cfg.telemetry.enabled, false);
    assert_eq!(cfg.ollama.endpoint, "http://localhost:11434");
}

#[test]
fn parses_user_config_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("phoenix-config.toml");
    let mut file = std::fs::File::create(&path).unwrap();
    writeln!(
        file,
        r#"
[telemetry]
enabled = true

[ollama]
endpoint = "http://192.168.1.50:11434"
model = "llama3.3:70b"
"#
    )
    .unwrap();

    let cfg = Config::load_or_default(&path).unwrap();

    assert_eq!(cfg.telemetry.enabled, true);
    assert_eq!(cfg.ollama.endpoint, "http://192.168.1.50:11434");
    assert_eq!(cfg.ollama.model, "llama3.3:70b");
}

#[test]
fn rejects_malformed_toml() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("phoenix-config.toml");
    std::fs::write(&path, "this is :: not valid toml ===").unwrap();

    let result = Config::load_or_default(&path);

    assert!(result.is_err());
}

#[test]
fn telemetry_config_default_is_opt_out() {
    let cfg = TelemetryConfig::default();
    assert_eq!(cfg.enabled, false);
}
