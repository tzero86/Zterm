# Local LLM Inference Failure - Root Cause & Fixes

## Executive Summary
When user runs `/agent hello dude`, the local LLM model discovery works, model is selected, but chat inference fails with generic error message "Request did not successfully complete." The root cause is insufficient error logging and context, making the actual failure reason invisible to users and developers.

## Root Cause Identified

**Primary Issue**: Missing error logging in `generate_local_llm_output()` function
- **File**: `app/src/ai/agent/api/impl.rs:286-446`
- **Lines**: 311-316
- When `generate_with_tools()` fails, error is wrapped but not logged
- Users see only generic message without knowing: model name, URL, or actual error details

## Error Path Analysis
```
User: /agent hello dude
  ↓
parse_local_model_id() → extracts (provider, model_name) ✓
  ↓
generate_local_llm_output(provider, model, params) 
  ↓
generate_with_tools(messages, model, tools) 
  ↓ ERROR OCCURS HERE
  ↓
Error wrapped: "Local LLM request failed: {e}"
  ↓ NO LOGGING - Error details lost
  ↓
ConvertToAPITypeError::Other
  ↓
ResponseStream error stream (no Finished event)
  ↓
Controller: "Request did not successfully complete"
```

## Root Cause - Why It Fails

Most likely failures:
1. **Model not found**: Ollama has model listed but not actually loaded
2. **Endpoint wrong**: Custom Ollama port or base URL mismatch
3. **Connection refused**: Ollama stopped between discovery and inference
4. **Response format mismatch**: Ollama returns error JSON instead of expected response

## Critical Fixes Required

### Fix 1: Add Logging in generate_local_llm_output
**Why**: Users and developers have no visibility into what's failing

**Location**: `app/src/ai/agent/api/impl.rs:310-316`

**Current Code**:
```rust
for _ in 0..MAX_ITERATIONS {
    let response = client
        .generate_with_tools(messages.clone(), &model, Some(tools.clone()))
        .await
        .map_err(|e| {
            ConvertToAPITypeError::Other(anyhow::anyhow!("Local LLM request failed: {e}"))
        })?;
```

**Fixed Code**:
```rust
for _ in 0..MAX_ITERATIONS {
    let response = client
        .generate_with_tools(messages.clone(), &model, Some(tools.clone()))
        .await
        .map_err(|e| {
            log::error!("Local LLM inference failed - Model: {}, Provider: {:?}, Error: {e:?}", model, provider);
            ConvertToAPITypeError::Other(anyhow::anyhow!("Local LLM request failed: {e}"))
        })?;
```

### Fix 2: Add Context to Error Messages in LocalLLMClient
**Why**: HTTP and parse errors should include model name and URL for debugging

**Location**: `app/src/ai/local_llm/client.rs:114-156`

**Current Code** (line 115-120):
```rust
pub async fn generate_with_tools(
    &self,
    messages: Vec<AgentMessage>,
    model: &str,
    tools: Option<Vec<serde_json::Value>>,
) -> Result<NonStreamingResponse> {
```

**Should Add**: Pass provider info to add to error messages

### Fix 3: Ensure Error Details Reach User
**Why**: Currently error details exist in logs but aren't shown in error message displayed to user

**Location**: Already handled by lines 154-155:
```rust
.map_err(|e| anyhow!("Failed to parse LLM response: {e}\nResponse: {text}"))?;
```
This is good - response body is included in error.

## Recommended Implementation

1. **Immediate**: Add logging at line 315 to capture actual error
2. **Quick**: Pass more context through error messages (URL, provider, model)
3. **Follow-up**: Validate model exists before first inference attempt
4. **Testing**: Add test that reproduces failure scenario (model not loaded)

## Code Changes Needed

### File 1: app/src/ai/agent/api/impl.rs

Around line 26-28, add logging at function entry:
```rust
pub async fn generate_multi_agent_output(
    server_api: Arc<ServerApi>,
    mut params: RequestParams,
    cancellation_rx: futures::channel::oneshot::Receiver<()>,
) -> Result<ResponseStream, ConvertToAPITypeError> {
    if let Some((provider, model_name)) = parse_local_model_id(&params.model) {
        log::info!("Starting local LLM generation with model: {}", model_name);  // ADD THIS
        return generate_local_llm_output(provider, model_name, params, cancellation_rx).await;
    }
    // ...rest of code
}
```

Around line 315, add error logging:
```rust
let response = client
    .generate_with_tools(messages.clone(), &model, Some(tools.clone()))
    .await
    .map_err(|e| {
        log::error!(
            "Local LLM generate_with_tools failed\nModel: {}\nProvider: {:?}\nBase URL: {}\nError: {:#}",
            model,
            provider,
            base_url,
            e
        );
        ConvertToAPITypeError::Other(anyhow::anyhow!("Local LLM request failed: {e}"))
    })?;
```

## Testing & Validation

After applying fixes, users should see detailed error logs like:
```
ERROR Local LLM generate_with_tools failed
Model: llama2
Provider: Ollama
Base URL: http://localhost:11434/v1
Error: LLM error: 404 - model not found
```

This clearly indicates what went wrong vs. the current generic error.

## Additional Insights

1. **Discovery works**: list_models() succeeds, so URL is reachable
2. **Inference fails**: generate_with_tools() fails
3. **Most likely causes** (in order):
   - Model listed but not actually loaded
   - Wrong model name passed (encoding issue)
   - Ollama native format vs. OpenAI compatibility mismatch
   - Model removed/unloaded between discovery and inference

## Command to Reproduce

```bash
# 1. Start Ollama with a model (don't load it yet)
ollama serve  

# 2. List models (works)
curl http://localhost:11434/api/tags

# 3. Run agent (should fail gracefully but show why)
/agent hello world

# 4. Check logs to see detailed error
```
