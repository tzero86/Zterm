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

    /// Non-streaming chat with optional tool support for the agentic loop.
    pub async fn generate_with_tools(
        &self,
        messages: Vec<AgentMessage>,
        model: &str,
        tools: Option<Vec<serde_json::Value>>,
    ) -> Result<NonStreamingResponse> {
        let mut payload = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": false,
            "temperature": 0.7,
            "max_tokens": 4096,
        });

        if let Some(tools) = tools {
            if !tools.is_empty() {
                payload["tools"] = serde_json::Value::Array(tools);
                payload["tool_choice"] = serde_json::json!("auto");
            }
        }

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

        let text = response.text().await?;
        let parsed: NonStreamingResponse = serde_json::from_str(&text)
            .map_err(|e| anyhow!("Failed to parse LLM response: {e}\nResponse: {text}"))?;
        Ok(parsed)
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

/// Flexible message type for the agentic loop (all roles + tool calls/results)
#[derive(Serialize, Clone, Debug)]
pub struct AgentMessage {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolCallInfo {
    pub id: String,
    pub r#type: String,
    pub function: ToolCallFunction,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Deserialize, Debug)]
pub struct NonStreamingResponse {
    pub choices: Vec<NonStreamingChoice>,
}

#[derive(Deserialize, Debug)]
pub struct NonStreamingChoice {
    pub message: NonStreamingMessage,
    pub finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct NonStreamingMessage {
    pub role: String,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCallInfo>>,
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
    let mut content = String::new();
    let mut finish_reason: Option<String> = None;

    for line in text.lines() {
        if line.starts_with("data: ") {
            let json_str = &line[6..];
            if json_str == "[DONE]" {
                finish_reason = Some("stop".to_string());
                break;
            }

            if let Ok(chunk) = serde_json::from_str::<OpenAIChunk>(json_str) {
                if let Some(choice) = chunk.choices.first() {
                    if let Some(c) = &choice.delta.content {
                        content.push_str(c);
                    }
                    if choice.finish_reason.is_some() {
                        finish_reason = choice.finish_reason.clone();
                    }
                }
            }
        }
    }

    Ok(ChatChunk {
        content: if content.is_empty() {
            None
        } else {
            Some(content)
        },
        finish_reason,
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
