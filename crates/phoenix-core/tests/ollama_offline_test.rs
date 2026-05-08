use phoenix_core::llm::OllamaClient;
use phoenix_core::llm::LlmClient;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn generate_returns_response_text() {
    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/generate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "model": "llama3.3:70b",
            "response": "hello world",
            "done": true
        })))
        .mount(&mock)
        .await;

    let client = OllamaClient::new(&mock.uri(), "llama3.3:70b");
    let result = client.generate("say hi").await.unwrap();

    assert_eq!(result, "hello world");
}

#[tokio::test]
async fn generate_returns_error_on_5xx() {
    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/generate"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock)
        .await;

    let client = OllamaClient::new(&mock.uri(), "llama3.3:70b");
    let result = client.generate("say hi").await;

    assert!(result.is_err());
}
