use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LLMId(String);

impl LLMId {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns `true` if this ID represents a locally-discovered model
    /// (e.g. an Ollama or LM Studio model with the `local-*` prefix).
    /// Local models are discovered at runtime and may not be present in the
    /// server-provided model list, so their IDs should not be cleared during
    /// validation against the current model choices.
    pub fn is_local(&self) -> bool {
        self.0.starts_with("local-")
    }
}

impl From<String> for LLMId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for LLMId {
    fn from(value: &str) -> Self {
        value.to_owned().into()
    }
}

impl From<LLMId> for String {
    fn from(value: LLMId) -> Self {
        value.0
    }
}

impl std::fmt::Display for LLMId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
