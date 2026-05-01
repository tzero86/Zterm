//! Local LLM settings and configuration

use crate::ai::local_llm::LocalLLMProvider;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use zterm_ui::{AppContext, Entity, SingletonEntity};

/// Settings for local LLM inference
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct LocalLLMSettings {
    /// Whether local LLM inference is enabled
    pub enabled: bool,

    /// Provider (Ollama, LM Studio, or Custom)
    pub provider: LocalLLMProvider,

    /// Optional base URL override (uses provider default if None)
    pub base_url: Option<String>,

    /// Selected model name (e.g., "mistral:7b")
    pub selected_model: Option<String>,
}

/// Events that can occur on LocalLLMSettings
#[derive(Clone, Debug)]
pub enum LocalLLMSettingsEvent {
    /// Settings were updated
    SettingsUpdated,
}

impl LocalLLMSettings {
    /// Create a new default instance
    pub fn new() -> Self {
        Self {
            enabled: false,
            provider: LocalLLMProvider::default(),
            base_url: None,
            selected_model: None,
        }
    }

    /// Check if local LLM is properly configured and enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled && self.selected_model.is_some()
    }

    /// Get the base URL, using provider default if not overridden
    pub fn base_url(&self) -> String {
        self.base_url
            .clone()
            .unwrap_or_else(|| self.provider.default_base_url().to_string())
    }

    /// Set the enabled flag
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Set the provider
    pub fn set_provider(&mut self, provider: LocalLLMProvider) {
        self.provider = provider;
    }

    /// Set the base URL override
    pub fn set_base_url(&mut self, base_url: Option<String>) {
        self.base_url = base_url;
    }

    /// Set the selected model
    pub fn set_selected_model(&mut self, model: Option<String>) {
        self.selected_model = model;
    }
}

impl Default for LocalLLMSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalLLMSettings {
    /// Initialize the singleton in the app context.
    pub fn init(ctx: &mut AppContext) {
        ctx.add_singleton_model(|_| LocalLLMSettings::new());
    }
}

impl Entity for LocalLLMSettings {
    type Event = LocalLLMSettingsEvent;
}

impl SingletonEntity for LocalLLMSettings {}
