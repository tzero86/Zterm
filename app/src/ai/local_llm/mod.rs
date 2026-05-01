//! Local LLM provider abstraction and integration
//!
//! Supports Ollama, LM Studio, and any OpenAI-compatible endpoint.
//! Provides auto-detection, model discovery, and streaming chat inference.

// Many items here are part of the public Local LLM API surface. They may be
// unused on certain build configurations (e.g. tools-only, wasm) but are
// retained intentionally so callers can use them when needed.
#![allow(dead_code)]

pub mod client;
pub mod discovery;
pub mod provider;
pub mod settings;

#[cfg(not(target_arch = "wasm32"))]
pub use client::ToolCallInfo;
pub use client::{AgentMessage, ChatMessage, LocalLLMClient};
pub use provider::LocalLLMProvider;
pub use settings::LocalLLMSettings;
