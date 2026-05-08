//! LLM client abstraction. The default implementation is OllamaClient.

mod ollama;

pub use ollama::OllamaClient;

use async_trait::async_trait;

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate(&self, prompt: &str) -> anyhow::Result<String>;
}
