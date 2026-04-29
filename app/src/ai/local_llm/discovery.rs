//! Auto-discovery and provider detection for local LLMs

use crate::ai::local_llm::client::LocalModel;
use crate::ai::local_llm::{LocalLLMClient, LocalLLMProvider};

/// Discover available local LLM providers by probing default ports
pub async fn discover_providers() -> Vec<(LocalLLMProvider, u64)> {
    let mut available = Vec::new();

    // Try Ollama on default port
    if let Ok(latency) = try_provider(LocalLLMProvider::Ollama).await {
        available.push((LocalLLMProvider::Ollama, latency));
    }

    // Try LM Studio on default port
    if let Ok(latency) = try_provider(LocalLLMProvider::LMStudio).await {
        available.push((LocalLLMProvider::LMStudio, latency));
    }

    available
}

/// Try connecting to a provider at its default base URL
async fn try_provider(provider: LocalLLMProvider) -> anyhow::Result<u64> {
    let base_url = provider.default_base_url();
    let client = LocalLLMClient::new(provider, base_url);
    client.health_check().await
}

/// List models available from a specific provider
pub async fn list_models_for_provider(
    provider: LocalLLMProvider,
    base_url: Option<&str>,
) -> anyhow::Result<Vec<LocalModel>> {
    let url = base_url.unwrap_or(provider.default_base_url());
    let client = LocalLLMClient::new(provider, url);
    client.list_models().await
}
