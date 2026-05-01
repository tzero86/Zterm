# Local LLM Inference Failure Investigation

## Problem Summary
When users run `/agent hello dude` with a local LLM model selected, the model is discovered and selected successfully, but chat inference fails with:
- Error Message to User: "I'm sorry, I couldn't complete that request. Request did not successfully complete"
- This indicates the stream ends without emitting a `StreamFinished` event

## Error Flow Traced
1. User runs `/agent hello dude`
2. Model selection works (discovery succeeds)
3. `generate_multi_agent_output()` is called (app/src/ai/agent/api/impl.rs:26-27)
4. For local models, `generate_local_llm_output()` is called (impl.rs:287-446)
5. Inside the agentic loop (10 iterations max), `generate_with_tools()` is called (impl.rs:312)
6. **If `generate_with_tools()` returns an error**, it's wrapped as:
   - `ConvertToAPITypeError::Other(anyhow::anyhow!("Local LLM request failed: {e}"))` (impl.rs:315)
7. This error propagates to response_stream.rs:205-208
8. Error stream is completed without emitting `StreamFinished` event
9. Controller shows: "Request did not successfully complete" (controller.rs:2442)

## Files Involved
- **API Integration**: `app/src/ai/agent/api/impl.rs` (286-446)
- **Local LLM Client**: `app/src/ai/local_llm/client.rs` (115-156)
- **Provider Configuration**: `app/src/ai/local_llm/provider.rs` (18-46)
- **Response Stream Handling**: `app/src/ai/blocklist/controller/response_stream.rs` (2442)
- **Error Display**: `app/src/ai/blocklist/block/view_impl/common.rs` (129)

## Root Cause Analysis - Critical Issues Found

### Issue 1: Missing Response Content Field
**Location**: `app/src/ai/local_llm/client.rs:253-258`

**Problem**: The `NonStreamingMessage` struct expects optional content field, but when deserializing Ollama's response that might have tool_calls without content, the parsing could fail if Ollama doesn't include the field.

```rust
#[derive(Deserialize, Debug)]
pub struct NonStreamingMessage {
    pub role: String,
    pub content: Option<String>,  // <-- Optional, but might cause issues
    pub tool_calls: Option<Vec<ToolCallInfo>>,
}
```

Ollama's native API returns messages differently than OpenAI-compatible API. If the model is using native Ollama format but expecting OpenAI format, JSON parsing fails at line 154.

### Issue 2: Incorrect Endpoint Construction
**Location**: `app/src/ai/local_llm/client.rs:138 & provider.rs:34-45`

**Problem**: Inconsistent endpoint handling between different operations:

For Ollama, the provider returns:
- `default_base_url()`: `"http://localhost:11434/v1"` (WITH /v1)
- `models_endpoint()`: Trims `/v1` → `"http://localhost:11434/v1/api/tags"` → Actually becomes `"http://localhost:11434/api/tags"` ✓
- Chat endpoint: `"{base_url}/chat/completions"` → `"http://localhost:11434/v1/chat/completions"` ✓

**Status**: This is actually correct for OpenAI-compatible endpoints, but discovery uses Ollama's native API while inference uses OpenAI compatibility layer.

### Issue 3: Payload Serialization Inconsistency
**Location**: `app/src/ai/local_llm/client.rs:121-127 & agent/api/impl.rs:561-608`

**Problem**: The payload structure doesn't guarantee all required fields:

```rust
let mut payload = serde_json::json!({
    "model": model,
    "messages": messages,  // <-- AgentMessage with optional content
    "stream": false,
    "temperature": 0.7,
    "max_tokens": 4096,
});
```

When messages contain optional fields that are None, they're skipped due to `#[serde(skip_serializing_if = "Option::is_none")]`. This could cause Ollama to reject the request if it expects all fields present.

### Issue 4: No Error Details Propagated to User
**Location**: `app/src/ai/agent/api/impl.rs:314-316`

**Problem**: HTTP error responses from Ollama/LM Studio are wrapped generically:
```rust
.map_err(|e| {
    ConvertToAPITypeError::Other(anyhow::anyhow!("Local LLM request failed: {e}"))
})?;
```

If Ollama returns HTTP 400 (bad request) or 404 (model not found), the detailed error message is lost. Users see only generic message.

### Issue 5: Model Name Encoding/Decoding
**Location**: `app/src/ai/agent/api/impl.rs:448-491`

**Problem**: Model names can be hex-encoded or not:
- Hex format: `"local-ollama-hex-{hex_encoded_model_name}"`
- Legacy format: `"local-ollama-{model_name}"`

If there's a mismatch between how the model name is encoded during discovery vs. how it's decoded during inference, the model name could be wrong.

### Issue 6: No Validation That Model Actually Exists Before Inference
**Location**: `app/src/ai/agent/api/impl.rs:311-316`

**Problem**: The code assumes the model exists just because it was discovered. But:
1. Model could have been unloaded between discovery and inference
2. Model name could be misspelled or have encoding issues  
3. No pre-flight check before attempting inference

## Specific Code Issues

### In `client.rs` - Missing Error Context
```rust
// Line 144-150: Error message loses details
if !response.status().is_success() {
    return Err(anyhow!(
        "LLM error: {} - {}",
        response.status(),
        response.text().await.unwrap_or_default()  // Good: shows response
    ));
}

// But line 154: JSON parse error could be cryptic
let parsed: NonStreamingResponse = serde_json::from_str(&text)
    .map_err(|e| anyhow!("Failed to parse LLM response: {e}\nResponse: {text}"))?;
    // This is good - includes response body
```

### In `impl.rs` - No Stream Validation
```rust
// Line 310-353: Assumes response always has choices[0]
let response = client
    .generate_with_tools(messages.clone(), &model, Some(tools.clone()))
    .await
    .map_err(|e| {
        ConvertToAPITypeError::Other(anyhow::anyhow!("Local LLM request failed: {e}"))
    })?;

let choice = response.choices.into_iter().next().ok_or_else(|| {
    ConvertToAPITypeError::Other(anyhow::anyhow!("LLM returned no choices"))  // Good fallback
})?;
```

## Recommended Fixes

### Priority 1 (Critical - Blocks Inference)
1. **Add explicit logging** in generate_local_llm_output to log:
   - Model name being used
   - Base URL being used
   - First request/response for debugging

2. **Validate response structure** - check that NonStreamingResponse has choices before accessing
   - Already done at line 318-320 ✓

3. **Ensure payload compatibility** - guarantee all OpenAI API fields are present:
   - Messages might need content to always be present (not Optional)
   - May need to filter/validate messages before sending

### Priority 2 (High - Improves Debugging)
1. **Wrap errors with context** in generate_with_tools:
   - Include model name in error message
   - Include endpoint URL in error message
   - Include first 500 chars of response body for non-JSON responses

2. **Add pre-flight validation**:
   - Check that base URL is reachable
   - Verify model exists before attempting inference

### Priority 3 (Medium - Robustness)
1. **Handle Ollama native format** - detect and convert native API responses
2. **Add telemetry** for local LLM inference failures
3. **Validate message encoding** - ensure no serialization issues

## Testing Commands
```bash
# Verify Ollama is running and check model endpoint
curl http://localhost:11434/api/tags

# Test OpenAI-compatible endpoint
curl -X POST http://localhost:11434/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "model_name",
    "messages": [{"role": "user", "content": "hello"}],
    "stream": false
  }'
```
