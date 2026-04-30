//! Local LLM provider abstraction and integration
//!
//! Supports Ollama, LM Studio, and any OpenAI-compatible endpoint.
//! Provides auto-detection, model discovery, and streaming chat inference.

pub mod client;
pub mod discovery;
pub mod provider;
pub mod settings;

pub use client::{
    AgentMessage, ChatMessage, LocalLLMClient, NonStreamingResponse, ToolCallFunction, ToolCallInfo,
};
pub use provider::LocalLLMProvider;
pub use settings::LocalLLMSettings;
