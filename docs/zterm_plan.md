# Zterm — Full Rebrand & Local-First Architecture Plan

> **Vision:** A fully open, login-free terminal with built-in AI that runs entirely on-device
> using small local models (Gemma 3/4, Llama, Mistral, etc.) via Ollama or any
> OpenAI-compatible endpoint. No Warp account, no cloud dependency, no telemetry.

---

## Table of Contents

1. [Current State Assessment](#1-current-state-assessment)
2. [What We're Keeping](#2-what-were-keeping)
3. [What We're Replacing or Removing](#3-what-were-replacing-or-removing)
4. [Phase 1 — Remove Login](#phase-1--remove-login-12-days)
5. [Phase 2 — Local LLM Integration](#phase-2--local-llm-integration-12-weeks)
6. [Phase 3 — Strip Cloud-Only Features](#phase-3--strip-cloud-only-features-35-days)
7. [Phase 4 — Filesystem & Code Rebrand](#phase-4--filesystem--code-rebrand-35-days)
8. [Phase 5 — Replace warp_multi_agent_api](#phase-5--replace-warp_multi_agent_api-ongoing)
9. [Recommended Execution Order](#9-recommended-execution-order)
10. [Dependency Risk Register](#10-dependency-risk-register)

---

## 1. Current State Assessment

### Architecture Summary

Zterm is a fork of the Warp terminal. The codebase is a Rust app using a custom UI
framework (`zterm_ui`). The terminal emulator (PTY, input/output, syntax highlighting,
themes, keybindings) is **fully local** — nothing in the core terminal requires network
access.

The cloud dependencies are layered on top:

| Layer | Technology | Dependency level |
|---|---|---|
| Terminal emulator | Custom PTY + VTE | ✅ Fully local |
| UI framework | Custom `zterm_ui` (GPU-rendered) | ✅ Fully local |
| Settings / preferences | SQLite + local files | ✅ Fully local |
| AI inference | HTTP POST → `app.warp.dev/ai/multi-agent` | 🔴 Cloud only |
| Authentication | Firebase (Google Identity Platform) | 🔴 Cloud only |
| Drive sync | WebSocket → `wss://rtc.app.warp.dev` | 🔴 Cloud only |
| Model list | GraphQL → `app.warp.dev/graphql` | 🔴 Cloud only |
| Telemetry | Rudderstack (fires even anonymously) | 🔴 Cloud only |
| Session sharing | WebSocket → `wss://sessions.app.warp.dev` | 🔴 Cloud only |
| Crash reporting | Sentry | 🔴 Cloud only |

### The Good News

- **Login bypass is one line.** `RootView::new()` already has a
  `SkipFirebaseAnonymousUser` branch that goes straight to the terminal. Making it
  unconditional removes the entire login gate.
- **A local LLM injection point already exists.** `ServerApi::generate_multi_agent_output()`
  is the single function all AI inference flows through. We add a new local path there.
- **`WARP_SERVER_ROOT_URL` env var already wired.** All API calls respect a runtime URL
  override, meaning a local proxy server works today with zero code changes.
- **UI string rebrand is already done** (Phases 0 completed in prior sessions).

### The Hard Dependencies

| Item | Why it's hard | Mitigation |
|---|---|---|
| `warp_multi_agent_api` crate | Warp's closed-source protobuf protocol — all AI streaming uses it | Bypass it for local mode; replace entirely in Phase 5 |
| `app.warp.dev` / Firebase | Auth + cloud AI backend — Warp-owned infrastructure | Remove auth in Phase 1; route AI locally in Phase 2 |
| `~/.warp/` config dirs | Real user files on disk — renaming orphans existing installs | Migrate on first run (copy if new path absent) |
| `warp.sqlite` database | Rename without migration loses all user data | One-time file rename migration at startup |
| `X-Warp-*` HTTP headers | Server-side API contract — Warp's servers expect these | Only matters while we still talk to Warp's servers; irrelevant after Phase 2–3 |

---

## 2. What We're Keeping

- The terminal emulator (PTY, VTE, shell integration, SSH Ztermify)
- The GPU-rendered UI framework (`zterm_ui`)
- The agent action system (shell commands, file reads/writes, code diffs, grep)
- The MCP (Model Context Protocol) server integration
- Settings persistence (SQLite + local file settings)
- Themes, keybindings, command palette
- The block-based UI for AI conversations
- Skills / prompt templates system
- The execution profiles / permissions model (repurposed for local model config)
- Command signatures and completions

---

## 3. What We're Replacing or Removing

| Item | Action | Phase |
|---|---|---|
| Firebase authentication | Remove entirely | 1 |
| Login / auth UI screens | Remove / delete | 1 |
| Rudderstack telemetry | No-op / remove | 1 |
| `notify_login()` server ping | Remove | 1 |
| Cloud model list fetching | Replace with local Ollama discovery | 2 |
| `app.warp.dev/ai/multi-agent` inference | Replace with local OpenAI-compatible client | 2 |
| Cloud settings sync | Remove | 3 |
| Warp Drive (cloud object sync) | Remove / gate behind `cloud` feature | 3 |
| Team management | Remove / gate behind `cloud` feature | 3 |
| Billing & usage UI | Remove | 3 |
| Session sharing (WebSocket) | Remove / gate behind `cloud` feature | 3 |
| `~/.warp/` config directory | Migrate → `~/.zterm/` | 4 |
| `warp.sqlite` database | Migrate → `zterm.sqlite` | 4 |
| `TERM_PROGRAM=WarpTerminal` | Change to `ZtermTerminal` | 4 |
| `warp://` URL scheme | Change to `zterm://` | 4 |
| Internal `Warp*` type names | Rename to `Zterm*` | 4 |
| `warp_multi_agent_api` crate | Replace with `zterm_agent_api` | 5 |

---

## Phase 1 — Remove Login (1–2 days)

**Goal:** App opens straight to terminal. No account required. No Firebase calls.

### Step 1.1 — Hard-wire the no-login path

**File:** `app/src/root_view.rs`

The `else` branch in `RootView::new()` when `!is_logged_in()` currently defaults to
showing the auth screen. Change it to always produce `AuthOnboardingState::Terminal`.
This is essentially what the `SkipFirebaseAnonymousUser` branch already does — just
make it the unconditional default.

```rust
// BEFORE (simplified):
let auth_onboarding_state = if auth_state.is_logged_in() {
    AuthOnboardingState::Terminal(...)
} else if FeatureFlag::SkipFirebaseAnonymousUser.is_enabled() {
    AuthOnboardingState::Terminal(...)   // ← this branch
} else {
    AuthOnboardingState::Auth(...)       // ← remove this
};

// AFTER:
let auth_onboarding_state = AuthOnboardingState::Terminal(...);
```

### Step 1.2 — Disable telemetry phone-home

**File:** `app/src/server/telemetry/`

Make `send_telemetry_event()` and `flush_telemetry_events()` unconditional no-ops.
Remove the Rudderstack write key from `ChannelState`. Users must not be sending
behavioral data to Warp's analytics pipeline.

### Step 1.3 — Remove server pings on login

**File:** `app/src/auth/auth_manager.rs`

- Remove the `server_api.notify_login()` call in `on_user_fetched()`
- Remove the Firebase token refresh loop (`fetch_auth_tokens()`)
- Remove `get_or_refresh_access_token()` from all non-cloud code paths
  (Firebase will never be contacted since we have no credentials)

### Step 1.4 — Disable the cloud cascade on startup

**File:** `app/src/auth/auth_manager.rs` — `on_user_fetched()`

This function triggers Drive sync, team polling, settings sync, shared sessions,
model list fetch, and more. Comment it out or gate the whole body behind a
`cfg!(feature = "cloud")` check. The terminal still opens and works; cloud features
simply never start.

### Step 1.5 — Remove / hide auth UI

Mark or delete the following as dead code:

- `app/src/auth/auth_view_body.rs`
- `app/src/auth/auth_override_warning_body.rs`
- `app/src/auth/login_slide.rs`
- `app/src/auth/needs_sso_link_view.rs`
- `crates/onboarding/src/slides/` (login-specific slides)

The welcome / intro slide can be kept and simplified to just a "Get started" button
that immediately opens the terminal.

### ✅ Deliverable

App boots straight to a terminal. No login prompt. No Firebase network calls.
No telemetry. The AI panel opens but shows a "Configure local AI" prompt instead
of requiring a Warp account.

---

## Phase 2 — Local LLM Integration (1–2 weeks)

**Goal:** AI features work entirely via a local Ollama endpoint. No calls to
`app.warp.dev`. Supports Gemma 3/4 and any OpenAI-compatible model.

### Step 2.1 — Add `LocalLLMClient`

**New file:** `app/src/ai/local_llm/mod.rs`

Implement an HTTP client that speaks the
[OpenAI Chat Completions API](https://platform.openai.com/docs/api-reference/chat)
with streaming (`stream: true`). Ollama implements this at:
`http://localhost:11434/v1/chat/completions`

Use `reqwest` (already a dependency) and parse the SSE stream:
```
data: {"choices":[{"delta":{"content":"hello"}}]}
data: {"choices":[{"delta":{"content":" world"}}]}
data: [DONE]
```

The client needs:
- `generate(messages: Vec<ChatMessage>, model: &str, tools: Option<Vec<ToolDef>>) -> impl Stream<Item = ChatChunk>`
- `list_models(base_url: &str) -> Vec<String>` (calls `GET /v1/models` or Ollama's `GET /api/tags`)

### Step 2.2 — Add a local inference path in `generate_multi_agent_output()`

**File:** `app/src/ai/agent/api/impl.rs`

Add a check at the top of the AI dispatch function:

```rust
if LocalLLMSettings::as_ref(ctx).is_enabled() {
    return self.generate_local(input, ctx).await;
}
// ... existing warp_multi_agent_api path continues unchanged
```

The `generate_local()` function:
1. Converts `AIAgentInput` messages to `Vec<ChatMessage>` (OpenAI format)
2. Maps tool definitions (grep, read_files, shell_command, write_files) to OpenAI `tools` JSON
3. Calls `LocalLLMClient::generate()`
4. Converts streamed `ChatChunk` responses back to `AIAgentOutput`

This keeps the existing Warp cloud path fully intact for anyone who wants it.

### Step 2.3 — Add Local LLM settings

**File:** `app/src/settings_view/ai_page.rs` (new section) +
`app/src/ai/local_llm/settings.rs` (new settings struct)

Settings stored in the existing SQLite-backed settings system:

| Setting | Type | Default |
|---|---|---|
| `local_llm_enabled` | `bool` | `false` |
| `local_llm_base_url` | `String` | `"http://localhost:11434/v1"` |
| `local_llm_model` | `String` | `"gemma3:4b"` |
| `local_llm_context_window` | `u32` | `8192` |

UI components:
- Enable/disable toggle
- Base URL text field
- Model dropdown (populated from live `/v1/models` call)
- "Test connection" button → calls the endpoint and shows latency + available models
- A note explaining Ollama setup (`ollama pull gemma3:4b`)

### Step 2.4 — Bypass server-side model validation

**File:** `app/src/ai/agent_sdk/common.rs`

`validate_agent_mode_base_model_id()` rejects any model ID not in the server-fetched
allowlist. When `local_llm_enabled = true`, skip this validation entirely — any string
is a valid model name.

### Step 2.5 — Implement tool calling for local models

This is the most complex step. The existing agent system uses structured tool calls
to drive `grep`, `read_files`, `shell_command`, `write_files`, etc.

Map existing tool types to OpenAI `tools` JSON schema format:

```json
{
  "type": "function",
  "function": {
    "name": "shell_command",
    "description": "Run a shell command in the terminal",
    "parameters": {
      "type": "object",
      "properties": {
        "command": { "type": "string" }
      }
    }
  }
}
```

Parse `tool_calls` in the streamed response and route them to the existing
action executors in `app/src/ai/blocklist/action_model/execute/`.

**Priority order for tool support:**
1. `shell_command` — most important for an AI terminal
2. `read_files` — needed for code context
3. `write_files` / `request_file_edits` — needed for code changes
4. `grep` — codebase search
5. `ask_user_question` — clarification prompts

### Step 2.6 — Ollama model discovery

**File:** `app/src/ai/local_llm/discovery.rs`

When the settings page opens, call `GET http://localhost:11434/api/tags` to list
installed models. Populate the model dropdown with the results. If Ollama is not
running, show a friendly message with install instructions.

Also detect if Ollama is running at startup and show a status indicator in the AI
panel (green dot = local model ready, grey = not running).

### ✅ Deliverable

Select "Local (Ollama)" as AI provider, pick any installed model (e.g. `gemma3:4b`),
and the full agent experience runs entirely on-device. Shell commands, file reads,
code diffs — all working, all local. No `app.warp.dev` calls.

---

## Phase 3 — Strip Cloud-Only Features (3–5 days)

**Goal:** Remove all features that only work with a Warp account. Clean up dead code.
Reduce binary size.

### Step 3.1 — Add `cloud` Cargo feature flag

**File:** `app/Cargo.toml`

```toml
[features]
cloud = []
```

Wrap all cloud-dependent code with `#[cfg(feature = "cloud")]`. The default build
has `cloud` off. This makes the boundary explicit and allows future opt-in if
someone wants to build with Warp cloud support.

### Step 3.2 — Remove Firebase entirely

Once Phase 1 means we never try to log in, Firebase is dead code.

- Delete the Firebase token refresh logic from `app/src/auth/credentials.rs`
- Remove `firebase_auth_api_key` from `crates/zterm_core/src/channel/config.rs`
- Remove the `securetoken.googleapis.com` and `identitytoolkit.googleapis.com` calls
- Remove the `tink-*` crates from `Cargo.toml` if they were only used for
  Firebase credential encryption

### Step 3.3 — Stub / gate the GraphQL client

**File:** `app/src/server/server_api.rs`

When `cloud` feature is off, make `ServerApi` a zero-dependency stub — all methods
return `Err(ServerApiError::CloudDisabled)`. All callers already handle errors
gracefully (they just show empty states). This removes the `reqwest` cloud HTTP
client from the default build path.

### Step 3.4 — Remove Drive, Teams, Billing, Session Sharing UI

Gate or delete:

- `app/src/drive/` — Warp Drive UI and sync
- `app/src/settings_view/teams_page.rs`
- `app/src/settings_view/billing_and_usage_page.rs`
- `app/src/workspace/view/session_sharing/`
- `app/src/billing/`

These will naturally become dead code after Step 3.3 since they depend on
`ServerApi` calls that are now stubs.

### Step 3.5 — Remove crash reporting phone-home

**File:** `app/src/crash_reporting/`

Sentry crash reporting sends crash data to Warp's Sentry project. Either remove it
entirely or replace the DSN with a self-hosted Sentry instance. For now, remove.

### ✅ Deliverable

The binary no longer links any Firebase, Rudderstack, or Sentry SDKs. Settings
page shows only local/AI tabs. Binary size meaningfully reduced.

---

## Phase 4 — Filesystem & Code Rebrand (3–5 days)

**Goal:** Everything on disk says "zterm". Existing installs migrate automatically.

### Step 4.1 — Migrate `~/.warp/` → `~/.zterm/`

**File:** `crates/zterm_core/src/paths.rs`

Change:
```rust
pub const ZTERM_CONFIG_DIR: &str = ".warp";  // → ".zterm"
```

Add a one-time migration at app startup (before any config files are read):

```rust
fn migrate_config_dir_if_needed() {
    let old = home_dir().join(".warp");
    let new = home_dir().join(".zterm");
    if !new.exists() && old.exists() {
        fs::rename(&old, &new)
            .or_else(|_| copy_dir_recursive(&old, &new))  // cross-device fallback
            .log_if_error("config dir migration");
    }
}
```

Run this in `app::init()` before `ChannelState` resolves any paths.

### Step 4.2 — Migrate `warp.sqlite` → `zterm.sqlite`

**File:** `app/src/persistence/sqlite.rs`

Change:
```rust
const ZTERM_SQLITE_FILE_NAME: &str = "warp.sqlite";  // → "zterm.sqlite"
```

Add file rename migration before the DB connection is opened:

```rust
fn migrate_sqlite_if_needed(data_dir: &Path) {
    let old = data_dir.join("warp.sqlite");
    let new = data_dir.join("zterm.sqlite");
    if !new.exists() && old.exists() {
        let _ = fs::rename(&old, &new);
        // Also migrate -shm and -wal files if present
        let _ = fs::rename(data_dir.join("warp.sqlite-shm"), new.with_extension("sqlite-shm"));
        let _ = fs::rename(data_dir.join("warp.sqlite-wal"), new.with_extension("sqlite-wal"));
    }
}
```

### Step 4.3 — Rename `TERM_PROGRAM` value

**Files:**
- `app/src/terminal/local_tty/unix.rs`
- `app/src/terminal/local_tty/windows/environment.rs`
- `app/src/ai/agent_sdk/mod.rs` (`is_running_in_warp()`)

```rust
// unix.rs
builder.env("TERM_PROGRAM", "ZtermTerminal");  // was "WarpTerminal"

// mod.rs
pub fn is_running_in_zterm() -> bool {
    std::env::var("TERM_PROGRAM")
        .map(|v| v == "ZtermTerminal")
        .unwrap_or(false)
}
```

> **Note:** Shell plugins and detection scripts that check `$TERM_PROGRAM == "WarpTerminal"`
> will stop matching. This is acceptable — Zterm is a new terminal.

### Step 4.4 — Rename remaining internal `Warp*` types

A targeted find-and-replace refactor pass:

| Old name | New name | Files affected |
|---|---|---|
| `WarpAiExecutionContext` | `ZtermAiExecutionContext` | `ai_assistant/execution_context.rs` + users |
| `WarpAiOsContext` | `ZtermAiOsContext` | same |
| `WarpifiedRemote` | `ZtermifiedRemote` (or `SshRemote`) | `terminal/model/session/` + ~25 match arms |
| `WarpingProps` | `RunningProps` | `ai/blocklist/block/view_impl/common.rs` |
| `WarpingIndicatorProps` | `RunningIndicatorProps` | same |
| `MCPProvider::Warp` | `MCPProvider::Zterm` | `ai/mcp/mod.rs` |
| `WarpMcpConfigPath` | `ZtermMcpConfigPath` | `warp_managed_paths_watcher.rs` |
| `Icon::Warp` / `Icon::WarpLogoLight` | `Icon::Zterm` etc. | icon system |

### Step 4.5 — Rename DB column names (SQLite migration)

Three columns in the SQLite schema still use `warp_` prefixes:
- `warp_ai_width`
- `warp_drive_index_width`
- `is_warp_pack`

SQLite doesn't support `ALTER COLUMN RENAME`. Migration strategy: write a Diesel
migration that creates a new table with renamed columns, copies the data, drops the
old table, and renames the new one. Add this as a numbered migration in
`crates/persistence/migrations/`.

### Step 4.6 — Replace `warp://` URL scheme with `zterm://`

**File:** `crates/zterm_core/src/channel/state.rs` — `url_scheme()`

```rust
Channel::Stable  => "zterm",
Channel::Preview => "ztermpreview",
Channel::Dev     => "ztermdev",
Channel::Local   => "ztermlocal",
```

Update:
- `CLI_AGENT_NOTIFICATION_SENTINEL` in `app/src/terminal/cli_agent_sessions/mod.rs`
  from `"warp://cli-agent"` to `"zterm://cli-agent"`
- All `UriHost` parsing in `app/src/uri/mod.rs`
- macOS plist `CFBundleURLSchemes` entries
- Windows registry installer entries

### Step 4.7 — Re-host `warpdotdev/` fork dependencies

Re-host the purely-cosmetic forks (no Warp API contract) under a `zterm-dev` GitHub
org and update `Cargo.toml` URLs. Priority order:

| Crate | Priority | Reason |
|---|---|---|
| `vte` | High | Core terminal emulator VTE parser |
| `winit` | High | Window management |
| `font-kit` | Medium | Font loading |
| `pathfinder_simd` | Medium | GPU rendering math |
| `objc` | Medium | macOS ObjC bindings |
| `yaml-rust` | Low | YAML parsing |
| `notify` | Low | File watching |
| `difflib` | Low | Diff utilities |
| `jemallocator` | Low | Memory allocator |
| `tink-rust` | Low (remove with Firebase) | Encryption — only needed for Firebase |

Protocol crates (`warp-proto-apis`, `warp-workflows`, `command-signatures`,
`session-sharing-protocol`) — handle in Phase 5 or when those features are removed.

### ✅ Deliverable

Fresh install creates `~/.zterm/`. Existing installs automatically migrate data from
`~/.warp/`. App registers `zterm://` URL scheme. `TERM_PROGRAM=ZtermTerminal`.
Internal codebase has zero `Warp*` type names.

---

## Phase 5 — Replace `warp_multi_agent_api` (ongoing)

**Goal:** Remove the last hard dependency on Warp's closed-source protobuf protocol.
The local LLM path (Phase 2) already bypasses this — Phase 5 completes the removal.

### Step 5.1 — Audit current usage

`warp_multi_agent_api` appears as `use warp_multi_agent_api as api` in ~15 files.
The actual proto types are only used in two places:
- `ai/agent/api/convert_to.rs` — App types → proto
- `ai/agent/api/convert_from.rs` — Proto → App types
- `server/server_api.rs` — serializes the proto, sends it, deserializes the SSE response

Everything else uses internal Rust types (`AIAgentInput`, `AIAgentOutput`, etc.)
that are already clean and independent of the proto crate.

### Step 5.2 — Define `zterm_agent_api` crate

Create `crates/zterm_agent_api/` with plain Rust structs (serde-based, no proto):

```
zterm_agent_api/
├── src/
│   ├── request.rs    — AgentRequest, ChatMessage, ToolDefinition
│   ├── response.rs   — AgentResponseEvent, ToolCall, TextChunk
│   └── lib.rs
```

Model after OpenAI's API schema since all local backends speak it. This becomes the
canonical wire format for Zterm.

### Step 5.3 — Replace the conversion layer

Replace `convert_to.rs` and `convert_from.rs` to target `zterm_agent_api` types
instead of `warp_multi_agent_api` proto types. Replace the SSE stream parser in
`server_api.rs` to decode OpenAI-format responses.

### Step 5.4 — Remove `warp_multi_agent_api` from `Cargo.toml`

Once all references are migrated, remove the git dependency. This severs the final
functional tie to Warp's private GitHub repositories.

### Step 5.5 — Evaluate remaining protocol crates

| Crate | Decision |
|---|---|
| `warp-workflows` | Keep format, re-host repo as `zterm-workflows`, rename types |
| `warp-command-signatures` | Keep, re-host as `zterm-command-signatures` |
| `command-corrections` | Keep as-is (no Warp branding in public API) |
| `session-sharing-protocol` | Remove (feature removed in Phase 3) |
| `rmcp` | Keep (MCP is a third-party protocol standard — already re-hosted) |

### ✅ Deliverable

Zero `warpdotdev` protocol dependencies. The `Cargo.toml` contains only standard
crates.io dependencies and `zterm-dev` hosted forks. The codebase is fully
self-contained and forkable by anyone.

---

## 9. Recommended Execution Order

```
Week 1
├── Phase 1  (remove login)              — 1–2 days
│   └── Immediate UX win, low risk, unblocks everything else
└── Phase 2.1–2.3  (local LLM base)     — parallel
    └── Core feature work, highest user value

Week 2
├── Phase 2.4–2.6  (tool calls, model discovery)
└── Phase 3.1–3.3  (cloud feature gating, Firebase removal)

Week 3
├── Phase 4.1–4.3  (filesystem migration, TERM_PROGRAM)
├── Phase 4.4–4.5  (type renames, DB migration)
└── Phase 3.4–3.5  (remove Drive/Teams/Billing/Sharing UI)

Week 4
├── Phase 4.6  (re-host fork dependencies)
└── Phase 4.7  (URL scheme: warp:// → zterm://)

Ongoing (parallel with all above)
└── Phase 5  (replace warp_multi_agent_api)
```

**Start with Phase 1.** It is the biggest UX improvement with the least risk and
produces a working app you can test Phase 2 against. **Phase 2 second** — it is the
defining feature of this fork. Everything else is cleanup.

---

## 10. Dependency Risk Register

| Item | Risk | Mitigation |
|---|---|---|
| `~/.warp` → `~/.zterm` migration | 🟡 Medium — existing users lose config if migration fails | Copy (not rename) on first run; keep `.warp` as fallback for one release |
| `warp.sqlite` rename | 🟡 Medium — data loss if migration not atomic | Rename at startup before DB opens; keep `.warp.sqlite` backup for one release |
| `warp_multi_agent_api` removal | 🟡 Medium — 15+ files need updating | Phase 2 bypass means local LLM works immediately; Phase 5 is cleanup not unblocking |
| `WarpifiedRemote` rename | 🟡 Medium — ~25 match arms in SSH code | Mechanical rename, no logic change; do in one PR |
| DB column renames (`warp_*`) | 🟡 Medium — SQLite table rebuild required | Low user-visible impact; do as a numbered Diesel migration |
| `TERM_PROGRAM=WarpTerminal` change | 🟢 Low risk for new users, 🟡 medium for existing | Existing shell plugins break; document the change |
| Firebase removal | 🟢 Low — we're removing auth entirely | No users to break since we're removing the login requirement |
| `X-Warp-*` HTTP headers | 🟢 Low — only matters while talking to `app.warp.dev` | Irrelevant after Phase 2–3; rename whenever convenient |
| Forked `warpdotdev/` lib re-hosting | 🟢 Low — these are cosmetic forks | Simple git URL changes; do gradually |
| `warp://` URL scheme | 🟢 Low — no external users depend on it for our fork | Change in Phase 4; update installer scripts |

---

## Appendix: Key Files Quick Reference

| What you want to change | File |
|---|---|
| Login bypass / startup auth gate | `app/src/root_view.rs` ~L1752 |
| Firebase token refresh | `app/src/auth/credentials.rs` |
| Cloud feature cascade on login | `app/src/auth/auth_manager.rs` — `on_user_fetched()` |
| Telemetry send | `app/src/server/telemetry/` |
| All AI inference (cloud) | `app/src/ai/agent/api/impl.rs` — `generate_multi_agent_output()` |
| Server API client | `app/src/server/server_api.rs` |
| Model list / validation | `app/src/ai/llms.rs`, `app/src/ai/agent_sdk/common.rs` |
| Config directory paths | `crates/zterm_core/src/paths.rs` |
| SQLite filename | `app/src/persistence/sqlite.rs` |
| `TERM_PROGRAM` value | `app/src/terminal/local_tty/unix.rs`, `windows/environment.rs` |
| URL scheme (`warp://`) | `crates/zterm_core/src/channel/state.rs` — `url_scheme()` |
| Deep link handling | `app/src/uri/mod.rs` |
| OSC sentinel for CLI agent | `app/src/terminal/cli_agent_sessions/mod.rs` |
| AI settings page | `app/src/settings_view/ai_page.rs` |
| Execution profiles (model config) | `app/src/ai/execution_profiles/` |
| MCP config paths | `app/src/ai/mcp/mod.rs` |
| Proto ↔ app type conversion | `app/src/ai/agent/api/convert_to.rs`, `convert_from.rs` |
