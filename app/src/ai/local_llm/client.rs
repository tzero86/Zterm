//! Local LLM HTTP client for Ollama, LM Studio, and OpenAI-compatible endpoints

use crate::ai::local_llm::LocalLLMProvider;
use anyhow::{anyhow, Result};
use bytes::BytesMut;
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct LocalLLMClient {
    provider: LocalLLMProvider,
    base_url: String,
    client: Client,
    timeout_secs: u64,
}

impl LocalLLMClient {
    /// Create a new client for the given provider and base URL
    pub fn new(provider: LocalLLMProvider, base_url: impl Into<String>) -> Self {
        Self {
            provider,
            base_url: base_url.into(),
            client: Client::new(),
            timeout_secs: 300,
        }
    }

    /// Stream a chat completion (OpenAI-compatible format)
    pub async fn generate(
        &self,
        messages: Vec<ChatMessage>,
        model: &str,
        _tools: Option<Vec<serde_json::Value>>,
    ) -> Result<Box<dyn futures::stream::Stream<Item = Result<ChatChunk>> + Unpin + Send>> {
        let payload = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": true,
            "temperature": 0.7,
            "max_tokens": 4096,
        });

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .json(&payload)
            .timeout(Duration::from_secs(self.timeout_secs))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "LLM error: {} - {}",
                response.status(),
                response.text().await.unwrap_or_default()
            ));
        }

        use futures::stream::StreamExt;

        let buffer = Arc::new(Mutex::new(BytesMut::new()));

        let stream = response
            .bytes_stream()
            .then(move |bytes_result| {
                let buffer = Arc::clone(&buffer);
                async move {
                    match bytes_result {
                        Ok(chunk) => {
                            let mut buf = buffer.lock().await;
                            buf.extend_from_slice(&chunk);

                            // Find newline and extract text in a scoped block
                            let (complete_text, bytes_to_keep) = {
                                let text = String::from_utf8_lossy(&buf);

                                if let Some(last_newline_pos) = text.rfind('\n') {
                                    let complete = text[..=last_newline_pos].to_string();
                                    let incomplete = text[last_newline_pos + 1..].to_string();
                                    (Some(complete), incomplete.len())
                                } else {
                                    (None, 0)
                                }
                            };

                            if let Some(complete_text) = complete_text {
                                // Now text borrow is dropped, we can mutate buf
                                let total_len = buf.len();
                                let _ = buf.split_to(total_len - bytes_to_keep);
                                parse_sse_line(&complete_text)
                            } else {
                                // No complete line yet, wait for more data
                                Ok(ChatChunk {
                                    content: None,
                                    finish_reason: None,
                                })
                            }
                        }
                        Err(e) => Err(anyhow!("Stream error: {}", e)),
                    }
                }
            })
            .boxed();

        Ok(Box::new(stream))
    }

    /// List models available from this provider
    pub async fn list_models(&self) -> Result<Vec<LocalModel>> {
        let is_ollama = matches!(self.provider, LocalLLMProvider::Ollama);
        let url = self.provider.models_endpoint(&self.base_url);

        let response = self
            .client
            .get(&url)
            .timeout(Duration::from_secs(10))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch models: {}", response.status()));
        }

        let text = response.text().await?;

        if is_ollama {
            parse_ollama_models(&text)
        } else {
            parse_openai_models(&text)
        }
    }

    /// Health check - verify provider is reachable
    pub async fn health_check(&self) -> Result<u64> {
        let start = std::time::Instant::now();

        let health_url = self.provider.health_endpoint(&self.base_url);

        let response = self
            .client
            .get(&health_url)
            .timeout(Duration::from_secs(5))
            .send()
            .await?;

        let elapsed = start.elapsed().as_millis() as u64;

        if response.status().is_success() {
            Ok(elapsed)
        } else {
            Err(anyhow!("Health check failed: {}", response.status()))
        }
    }
}

// === Request/Response Types ===

#[derive(Serialize, Clone, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OpenAIChunk {
    pub choices: Vec<ChoiceDelta>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ChoiceDelta {
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Delta {
    pub content: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ChatChunk {
    pub content: Option<String>,
    pub finish_reason: Option<String>,
}

#[derive(Clone, Debug)]
pub struct LocalModel {
    pub name: String,
    pub size_bytes: Option<u64>,
    pub modified_at: Option<String>,
}

// === Ollama Response Format ===

#[derive(Deserialize)]
struct OllamaTagsResponse {
    models: Vec<OllamaModelInfo>,
}

#[derive(Deserialize)]
struct OllamaModelInfo {
    name: String,
    #[serde(default)]
    size: u64,
    #[serde(default)]
    modified_at: String,
}

// === OpenAI Response Format ===

#[derive(Deserialize)]
struct OpenAIModelsResponse {
    data: Vec<ModelData>,
}

#[derive(Deserialize)]
struct ModelData {
    id: String,
}

// === Parsing Functions ===

fn parse_sse_line(text: &str) -> anyhow::Result<ChatChunk> {
    for line in text.lines() {
        if line.starts_with("data: ") {
            let json_str = &line[6..];
            if json_str == "[DONE]" {
                return Ok(ChatChunk {
                    content: None,
                    finish_reason: Some("stop".to_string()),
                });
            }

            match serde_json::from_str::<OpenAIChunk>(json_str) {
                Ok(chunk) => {
                    if let Some(choice) = chunk.choices.first() {
                        return Ok(ChatChunk {
                            content: choice.delta.content.clone(),
                            finish_reason: choice.finish_reason.clone(),
                        });
                    }
                }
                Err(_) => continue,
            }
        }
    }

    Ok(ChatChunk {
        content: None,
        finish_reason: None,
    })
}

fn parse_ollama_models(text: &str) -> Result<Vec<LocalModel>> {
    let response: OllamaTagsResponse = serde_json::from_str(text)?;
    Ok(response
        .models
        .into_iter()
        .map(|m| LocalModel {
            name: m.name,
            size_bytes: if m.size > 0 { Some(m.size) } else { None },
            modified_at: if m.modified_at.is_empty() {
                None
            } else {
                Some(m.modified_at)
            },
        })
        .collect())
}

fn parse_openai_models(text: &str) -> Result<Vec<LocalModel>> {
    let response: OpenAIModelsResponse = serde_json::from_str(text)?;
    Ok(response
        .data
        .into_iter()
        .map(|m| LocalModel {
            name: m.id,
            size_bytes: None,
            modified_at: None,
        })
        .collect())
}
