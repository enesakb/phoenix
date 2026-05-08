use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse config TOML: {0}")]
    Parse(#[from] toml::de::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub telemetry: TelemetryConfig,
    #[serde(default)]
    pub ollama: OllamaConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelemetryConfig {
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    #[serde(default = "default_endpoint")]
    pub endpoint: String,
    #[serde(default = "default_model")]
    pub model: String,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            endpoint: default_endpoint(),
            model: default_model(),
        }
    }
}

fn default_endpoint() -> String {
    "http://localhost:11434".to_string()
}

fn default_model() -> String {
    "llama3.3:70b".to_string()
}

impl Config {
    pub fn load_or_default(path: &Path) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Ok(Config::default());
        }
        let raw = std::fs::read_to_string(path)?;
        let cfg: Config = toml::from_str(&raw)?;
        Ok(cfg)
    }
}
