//! Local LLM provider enum and configuration

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
pub enum LocalLLMProvider {
    /// Ollama (default: http://localhost:11434)
    #[default]
    Ollama,
    /// LM Studio (default: http://localhost:1234)
    LMStudio,
    /// Custom OpenAI-compatible endpoint
    Custom,
}

impl LocalLLMProvider {
    pub fn default_base_url(&self) -> &'static str {
        match self {
            Self::Ollama => "http://localhost:11434/v1",
            Self::LMStudio => "http://localhost:1234/v1",
            Self::Custom => "http://localhost:8080/v1",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Ollama => "Ollama",
            Self::LMStudio => "LM Studio",
            Self::Custom => "Custom",
        }
    }

    pub fn models_endpoint(&self, base_url: &str) -> String {
        match self {
            Self::Ollama => format!("{}/api/tags", base_url.trim_end_matches("/v1")),
            _ => format!("{}/models", base_url),
        }
    }

    pub fn health_endpoint(&self, base_url: &str) -> String {
        match self {
            Self::Ollama => format!("{}/", base_url.trim_end_matches("/v1")),
            _ => format!("{}/models", base_url),
        }
    }
}

impl std::fmt::Display for LocalLLMProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
