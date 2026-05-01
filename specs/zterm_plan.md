# Zterm — Full Rebrand & Local-First Architecture Plan

> **Vision:** A fully open, login-free terminal with built-in AI that runs entirely on-device
> using small local models (Gemma 3/4, Llama, Mistral, etc.) via Ollama, LM Studio, or any
> OpenAI-compatible endpoint. No Warp account, no cloud dependency, no telemetry.

---

## Table of Contents

1. [Current State Assessment](#1-current-state-assessment)
2. [What We're Keeping](#2-what-were-keeping)
3. [What We're Replacing or Removing](#3-what-were-replacing-or-removing)
4. [Gated Features Catalogue](#4-gated-features-catalogue)
5. [Phase 1 — Remove Login](#phase-1--remove-login-12-days)
6. [Phase 2 — Local LLM Integration](#phase-2--local-llm-integration-12-weeks)
7. [Phase 3 — Strip Cloud-Only Features](#phase-3--strip-cloud-only-features-35-days)
8. [Phase 4 — Filesystem & Code Rebrand](#phase-4--filesystem--code-rebrand-35-days)
9. [Phase 5 — Replace warp_multi_agent_api](#phase-5--replace-warp_multi_agent_api-ongoing)
10. [Recommended Execution Order](#10-recommended-execution-order)
11. [Dependency Risk Register](#11-dependency-risk-register)

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
| Cloud model list fetching | Replace with multi-provider local discovery (Ollama + LM Studio) | 2 |
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

## 4. Gated Features Catalogue

This section maps every feature currently locked behind a login or paid plan, explains
what actually enforces the gate (client-side only vs. server-enforced), and states
exactly what we need to do to make it freely available in Zterm.

> **Key insight from research:** Almost every AI feature flag is already `ON` by
> default in the codebase. The only thing blocking them from working without an account
> is a handful of runtime `is_logged_in()` / `is_any_ai_enabled()` checks deep in the
> auth and settings models. The paywall limits (quotas, model tiers) are
> **server-enforced** — they simply won't apply once we route AI through a local
> endpoint instead of `app.warp.dev`.

---

### 4.1 Login-Gated Features (Requires Any Account)

These features show a "Sign up" modal or are silently disabled when the user has no
account. They are gated by `is_logged_in()` or `is_anonymous_or_logged_out()` checks
at runtime, not by Cargo feature flags.

#### 4.1.1 — AI Features (Critical — Must Fix)

The most important gate. `AISettings::is_any_ai_enabled()` returns `false` for any
user who is anonymous or logged out, even if the `AgentMode` feature flag is on.
This cascades to disable agent mode, the AI context menu, conversation search,
code-review AI, and AI workflows.

| What is gated | Gate location | What happens when not logged in |
|---|---|---|
| All AI / Agent Mode | `app/src/settings/ai.rs` — `is_any_ai_enabled()` | Returns `false` → entire AI subsystem treated as disabled |
| AI toggle in settings | `app/src/settings_view/ai_page.rs:L3068` | Replaced with "create an account" prompt |
| Agent SDK driver startup | `app/src/ai/agent_sdk/driver.rs:L500` | Returns `AgentDriverError::NotLoggedIn` immediately |
| All CLI agent commands | `app/src/ai/agent_sdk/mod.rs:L1308` | Error: "please log in with `zterm login`" |
| Authenticated model list | `app/src/ai/llms.rs:L852` | Only free/public models fetched |
| AI request usage counters | `app/src/ai/request_usage_model.rs:L228` | Never fetched — stays empty |
| Agent Mode Workflow runs | `app/src/workspace/view.rs:L14689` | Login gate modal shown |
| `/remote-control` chip | `app/src/ai/blocklist/agent_input_footer/mod.rs:L1795` | Button disabled: "Log in to use /remote-control" |

**What to do:**
1. Patch `is_any_ai_enabled()` in `app/src/settings/ai.rs` — when local LLM is
   configured, always return `true` regardless of login state.
2. Remove the `is_logged_in()` guard in `AgentDriver::new()` — local agent runs
   need no auth.
3. Remove the `launch_command` auth check in `agent_sdk/mod.rs` for local-mode CLI
   commands.
4. The anonymous user AI soft/hard gate in `prompt_alert.rs`
   (`AnonymousUserRequestLimitHardGate`) — remove it. With local LLM there are no
   request credits to exhaust.

#### 4.1.2 — Warp Drive (Remove Entirely)

The entire Drive feature is gated behind login. `is_warp_drive_enabled()` returns
`false` for anonymous/logged-out users, suppressing the Drive panel, toolbar entry,
and all Drive command palette items.

| What is gated | Gate location | Action |
|---|---|---|
| Warp Drive panel and toolbar | `app/src/drive/settings.rs:L43` | Remove in Phase 3 |
| Share Object (Drive) | `app/src/drive/index.rs:L4891` | Remove in Phase 3 |
| Team Drive operations | `app/src/workspace/view.rs:L19651` | Remove in Phase 3 |
| Drive object creation limits | `app/src/server/cloud_objects/update_manager.rs` | Remove in Phase 3 |
| Non-personal object editing | `app/src/cloud_object/model/view.rs:L228` | Remove in Phase 3 |

**What to do:** Gate the entire `app/src/drive/` tree behind `#[cfg(feature = "cloud")]`
in Phase 3. No client-side fix needed — it's a cloud-only feature.

#### 4.1.3 — Session Sharing (Remove Entirely)

| What is gated | Gate location | Action |
|---|---|---|
| Share Session (pane) | `app/src/pane_group/mod.rs:L2438` | Remove in Phase 3 |
| Share Block (context menu) | `app/src/terminal/view.rs:L19362` | Remove in Phase 3 |
| Shared Blocks settings page | `app/src/settings_view/show_blocks_view.rs:L617` | Remove in Phase 3 |
| Shared Sessions settings | `app/src/workspaces/workspace.rs` `SessionSharingPolicy` | Remove in Phase 3 |

**What to do:** Gate behind `#[cfg(feature = "cloud")]` in Phase 3.

#### 4.1.4 — Teams (Remove Entirely)

All team management actions call `attempt_login_gated_feature()` and ultimately
require a server. ~13 separate actions: create team, leave, delete, invite,
domain restrictions, discoverability, admin panel, contact support, etc.

**What to do:** Gate `app/src/settings_view/teams_page.rs` and all team-related
workspace actions behind `#[cfg(feature = "cloud")]` in Phase 3.

#### 4.1.5 — Billing & Upgrade CTAs (Remove Entirely)

| What is gated | Gate location | Action |
|---|---|---|
| Billing and Usage page | `app/src/settings_view/billing_and_usage_page.rs:L689` | Remove in Phase 3 |
| "Upgrade AI Usage" CTAs | Multiple files (AI page, billing page, workspace view) | Remove in Phase 3 |
| "Upgrade Plan" / Stripe portal | `app/src/settings_view/main_page.rs:L187` | Remove in Phase 3 |
| View Plans action | `app/src/drive/index.rs:L5222` | Remove in Phase 3 |

**What to do:** Delete billing UI entirely in Phase 3.

#### 4.1.6 — Cloud Settings Sync (Remove)

| What is gated | Gate location | Action |
|---|---|---|
| Settings Sync toggle | `app/src/settings_view/main_page.rs:L683` | Remove in Phase 3 |
| Privacy settings server-push | `app/src/settings/privacy.rs:L539,575,607,709` | Remove in Phase 3 |
| Cloud preferences syncer | `app/src/auth/auth_manager.rs` `on_user_fetched()` | Remove in Phase 3 |

#### 4.1.7 — Miscellaneous Login-Gated UI (Clean Up)

| What is gated | Gate location | Action |
|---|---|---|
| Tab bar "Sign up" button | `app/src/workspace/view.rs:L17365` | Delete in Phase 1 |
| Anonymous AI sign-up banner (in terminal) | `app/src/terminal/view.rs:L11831` | Delete in Phase 1 |
| Referrals / Earn Rewards widget | `app/src/settings_view/main_page.rs:L772` | Delete in Phase 3 |
| Log out menu item hiding | `app/src/app_menus.rs:L234` | Simplify: keep Log out visible always |
| Inline AI sign-up prompt on AI toggle | `app/src/settings_view/ai_page.rs:L3068` | Replace with local LLM config prompt |
| "Sign in to edit" tooltips on shared objects | `app/src/cloud_object/model/view.rs:L228` | Removed with Drive in Phase 3 |
| Agent onboarding skipped for anonymous users | `app/src/workspace/view.rs:L6683` | Fix after Phase 1 (user is never anonymous) |
| `ForceLogin` preview-only gate | `app/src/bin/preview.rs` | Already off in default/OSS builds — leave it |

---

### 4.2 Paywall-Gated Features (Requires Paid Plan)

These features are gated by billing tier checks (`CustomerType`, `Tier` policies).

> **Critical distinction for Zterm:** Once we route AI through a local endpoint
> (Phase 2), **all server-side quota enforcement becomes irrelevant**. The server
> gates listed below only apply when calling `app.warp.dev`. For local LLM, there is
> no quota, no model tier, no rate limit — the user's hardware is the only constraint.

#### 4.2.1 — AI Request Quota

| Item | Free tier | Paid tier | Our approach |
|---|---|---|---|
| Monthly AI requests | 150/month hard cap | Higher limit or unlimited | **Irrelevant with local LLM** — remove the `PromptAlertState` hard/soft gate entirely for local mode |
| Anonymous user request cap | Fraction of free quota | N/A | **Remove** — no anonymous users in Zterm |
| Telemetry-off AI block (`TelemetryDisabledOnFreeTier`) | Blocks AI if analytics off | Not enforced for paid | **Remove** — no telemetry, no gate |
| `FreeUserNoAi` experiment | Blocks all AI | Not enrolled | **Leave flag OFF** — already off by default |

**Gate location:** `app/src/ai/blocklist/prompt/prompt_alert.rs` — `determine_state()`

**What to do:** When local LLM is enabled, `determine_state()` must always return
`PromptAlertState::NoAlert`. Add a short-circuit at the top of the function:
```rust
if LocalLLMSettings::as_ref(ctx).is_enabled() {
    return PromptAlertState::NoAlert;
}
```

#### 4.2.2 — AI Model Tier Access (`DisableReason::RequiresUpgrade`)

| Item | Free tier | Paid tier | Our approach |
|---|---|---|---|
| Frontier models (Claude Opus, GPT-4o, etc.) | Greyed out in model picker | Accessible | **Irrelevant** — local model picker shows only locally installed Ollama models |

**Gate location:** `app/src/ai/llms.rs` — `DisableReason::RequiresUpgrade` set
server-side per `LLMInfo`.

**What to do:** When local LLM is enabled, the model picker shows only locally
discovered models with no `DisableReason`. The server model list fetch is bypassed
entirely (Phase 2, Step 2.4).

#### 4.2.3 — Voice Input

| Item | Free tier | Paid tier | Our approach |
|---|---|---|---|
| Voice input enabled | `WarpAiPolicy.is_voice_enabled` from server | Policy may enable it | **Leave as-is for now** — voice is a cloud feature tied to Warp's speech-to-text. Revisit in a later phase with a local STT option. |
| Voice request quota | 10,000 req limit | Higher or unlimited | N/A until local STT |

#### 4.2.4 — Passive AI Features (Prompt Suggestions, Code Suggestions, Next Command)

| Feature | Gate | Our approach |
|---|---|---|
| Prompt Suggestions | `is_prompt_suggestions_toggleable()` server policy | When local LLM on, enable unconditionally — these are local inference features |
| Code Suggestions | `is_code_suggestions_toggleable()` server policy | Same |
| Next Command / autosuggestions | `is_next_command_enabled()` server policy | Same |

**What to do:** In `app/src/workspaces/user_workspaces.rs`, make these three
methods return `true` when local LLM is enabled, bypassing the server policy check.

#### 4.2.5 — Codebase Indexing Limits

| Item | Free tier | Our approach |
|---|---|---|
| Max indexed repos | 3 | **Remove client-side cap** — no server to enforce it for local indexing. Set `max_codebase_indices` to `usize::MAX` when not using Warp's cloud. |
| Max files per repo | 5,000 | **Remove or make configurable** |
| Embedding batch size | 100 | **Make configurable** |

**Gate location:** `app/src/ai/request_usage_model.rs` — `hit_codebase_index_limit()`

**What to do:** When local LLM is enabled, override `RequestLimitInfo` with
unlimited values (`is_unlimited = true`, `max_codebase_indices = usize::MAX`).

#### 4.2.6 — Bring-Your-Own-API-Key (BYOK)

| Item | Gate | Our approach |
|---|---|---|
| BYOK for team accounts | `ByoApiKeyPolicy.enabled` from server | **Irrelevant** — our local LLM is better BYOK than their BYOK. Keep the BYO key UI for users who want to use a cloud provider directly. |
| BYOK for solo users (`SoloUserByok` flag) | Feature flag, off by default | **Enable by default** — add `solo_user_byok` to the `default` feature array. |

#### 4.2.7 — Cloud / Ambient Agents

| Item | Gate | Our approach |
|---|---|---|
| Cloud agent runs (`oz` in cloud) | `AmbientAgentsPolicy` from server tier | **Cloud-only feature — remove/gate** in Phase 3. Local agent runs are fully supported. |
| Concurrent agent limit | `max_concurrent_agents` policy | N/A — removed with cloud agents |
| Cloud environment VMs (instance shape) | `instance_shape` policy | N/A — removed with cloud agents |

#### 4.2.8 — Features Safe to Unconditionally Enable (Client-Side Only)

These features are technically pay-gated but the enforcement is purely a UI check
with no independent server enforcement. Safe to just turn on:

| Feature | Gate | Why safe |
|---|---|---|
| Multiple admins in teams | `is_multi_admin_enabled()` in `team.rs` | Pure client-side UI check, no server validation |
| Team discoverability toggle | `CustomerType != Enterprise` check in `teams_page.rs` | Pure UI — but teams are removed in Phase 3 anyway |
| `TelemetryDisabledOnFreeTier` AI block | `prompt_alert.rs` | Remove — telemetry is gone entirely |
| `FreeUserNoAi` experiment | Feature flag | Leave OFF by default — already off |
| Anonymous user hard-gate banner | `AnonymousUserRequestLimitHardGate` | Remove — no anonymous users |

---

### 4.3 Feature Flags: Current State vs. What We Need

The feature flag system is compile-time (Cargo features) combined with a runtime
hashset. **Almost everything is already on by default.** The table below lists only
the flags relevant to our goals:

| Flag | Default? | Status | Action |
|---|---|---|---|
| `AgentMode` | ✅ YES | On | Nothing needed |
| `APIKeyAuthentication` | ✅ YES | On | Works — but we're replacing auth entirely |
| `SkipFirebaseAnonymousUser` | ✅ YES | On | Already makes app skip to terminal |
| `ForceLogin` | ❌ NO (Preview only) | Off | Leave off — never add to default |
| `FreeUserNoAi` | ❌ NO | Off | Leave off |
| `SoloUserByok` | ❌ NO | Off | **Add to default** |
| `Orchestration` | ❌ NO (Preview+) | Off | **Add to default** — multi-agent is a core feature |
| `LocalComputerUse` | ❌ NO (Dev only) | Off | **Add to default** — local users should have computer use |
| `AgentHarness` | ❌ NO (Dev only) | Off | **Add to default** — allows external harnesses like Claude Code |
| `OrchestrationV2` | ❌ NO (Dev only) | Off | Add to default once Phase 5 provides local equivalent |
| All other AI flags | ✅ YES | On | Nothing needed |

**Concrete change:** Add these four lines to the `default` feature array in
`app/Cargo.toml`:
```toml
"orchestration",
"local_computer_use",
"agent_harness",
"solo_user_byok",
```

---

### 4.4 The Single Most Important Code Change for Unlocking AI

Everything in sections 4.1.1 and 4.2.1 flows from one function. This is the
**critical patch** that must land in Phase 1 alongside the login bypass:

**File:** `app/src/settings/ai.rs` — `is_any_ai_enabled()`

```rust
// CURRENT BEHAVIOUR (simplified):
pub fn is_any_ai_enabled(ctx: &AppContext) -> bool {
    if auth_state.is_anonymous_or_logged_out() {
        return false;  // ← THIS is the gate. Remove it.
    }
    // ... checks AI toggle settings ...
}

// TARGET BEHAVIOUR:
pub fn is_any_ai_enabled(ctx: &AppContext) -> bool {
    // Local LLM always counts as "AI enabled" regardless of login state.
    if LocalLLMSettings::as_ref(ctx).is_enabled() {
        return true;
    }
    // For cloud AI path, keep the existing logic but don't gate on login.
    // ... checks AI toggle settings ...
}
```

This one change unblocks: Agent Mode input, the `@` context menu, conversation
search, the code-review AI send button, AI workflows, the agent view, and every
other AI surface that branches on `is_any_ai_enabled()`.

---

### 4.5 Summary: What Actually Needs Code Changes vs. What Just Disappears

| Category | # of gates | What happens to them |
|---|---|---|
| AI features blocked for anonymous/logged-out users | 8 | **Patch** — 4 code changes (see 4.1.1 + 4.4) |
| Prompt alert quota hard-gates | 7 states | **Patch** — 1 short-circuit in `determine_state()` |
| Drive (all gates) | 12 | **Delete** — entire feature removed in Phase 3 |
| Session sharing (all gates) | 5 | **Delete** — entire feature removed in Phase 3 |
| Teams (all gates) | 13 | **Delete** — entire feature removed in Phase 3 |
| Billing/upgrade CTAs | 8 | **Delete** — entire billing UI removed in Phase 3 |
| Server-enforced quotas (request limits, model tiers, codebase index caps) | 6 | **Irrelevant** — local LLM bypasses the server entirely; remove client-side caps |
| Cloud/ambient agent policies | 4 | **Delete** — cloud agents removed in Phase 3 |
| Passive AI feature policies (prompt suggestions, code suggestions, next command) | 3 | **Patch** — return `true` when local LLM enabled |
| Feature flags to enable | 4 | **Add to default** in `app/Cargo.toml` |

**Total code changes required to fully unlock all local features: ~10 targeted patches.**
Everything else is deletion of cloud-only code.

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

**Goal:** AI features work entirely via a local provider — Ollama or LM Studio — with
zero configuration for the common case. Supports Gemma 3/4 and any model the user
has installed. No calls to `app.warp.dev`.

---

### Step 2.1 — Define the `LocalLLMProvider` abstraction

**New files:**
```
app/src/ai/local_llm/
├── mod.rs          — re-exports, LocalLLMSettings
├── provider.rs     — LocalLLMProvider enum + per-provider config
├── client.rs       — HTTP client (OpenAI-compatible streaming)
├── discovery.rs    — auto-detection and model listing per provider
└── settings.rs     — persisted settings struct
```

**`provider.rs`** defines the three supported provider types:

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LocalLLMProvider {
    Ollama,    // localhost:11434
    LMStudio,  // localhost:1234
    Custom,    // user-specified URL
}

impl LocalLLMProvider {
    pub fn default_base_url(&self) -> &'static str {
        match self {
            Self::Ollama   => "http://localhost:11434/v1",
            Self::LMStudio => "http://localhost:1234/v1",
            Self::Custom   => "http://localhost:8080/v1",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Ollama   => "Ollama",
            Self::LMStudio => "LM Studio",
            Self::Custom   => "Custom (OpenAI-compatible)",
        }
    }

    /// Endpoint used to list available models.
    /// Ollama supports both /api/tags (native) and /v1/models (OpenAI compat).
    /// LM Studio and Custom use the standard OpenAI /v1/models endpoint.
    pub fn models_endpoint(&self, base_url: &str) -> String {
        match self {
            // Prefer Ollama's native endpoint — richer metadata (size, modified date)
            Self::Ollama => base_url
                .trim_end_matches("/v1")
                .to_string() + "/api/tags",
            _ => format!("{base_url}/models"),
        }
    }

    /// Health-check endpoint — a cheap GET that confirms the server is alive.
    pub fn health_endpoint(&self, base_url: &str) -> String {
        match self {
            Self::Ollama => base_url
                .trim_end_matches("/v1")
                .to_string() + "/",   // returns "Ollama is running"
            _            => format!("{base_url}/models"),
        }
    }
}
```

---

### Step 2.2 — Add `LocalLLMClient`

**File:** `app/src/ai/local_llm/client.rs`

A thin `reqwest`-based HTTP client (already a dependency) that speaks the
[OpenAI Chat Completions API](https://platform.openai.com/docs/api-reference/chat)
with streaming (`stream: true`).

Both Ollama and LM Studio implement the exact same wire format:
`POST {base_url}/chat/completions`

The client needs three public methods:

```rust
impl LocalLLMClient {
    /// Stream a chat completion. Works with Ollama, LM Studio, and any
    /// OpenAI-compatible server.
    pub async fn generate(
        &self,
        messages: Vec<ChatMessage>,
        model: &str,
        tools: Option<Vec<ToolDefinition>>,
    ) -> impl Stream<Item = Result<ChatChunk>>;

    /// List models available on this provider.
    /// Handles both Ollama's /api/tags format and the standard /v1/models format.
    pub async fn list_models(&self) -> Result<Vec<LocalModel>>;

    /// Fast connectivity check — returns Ok(latency_ms) or Err.
    pub async fn health_check(&self) -> Result<u64>;
}
```

`LocalModel` is a simple struct:
```rust
pub struct LocalModel {
    pub id: String,          // e.g. "gemma3:4b" or "lmstudio-community/gemma-3-4b-it-GGUF"
    pub display_name: String,// e.g. "Gemma 3 4B" (cleaned up for the UI)
    pub size_gb: Option<f32>,// shown as hint in the model picker
    pub context_length: Option<u32>,
}
```

Parse SSE stream:
```
data: {"choices":[{"delta":{"content":"hello"}}]}
data: {"choices":[{"delta":{"content":" world"}}]}
data: {"choices":[{"delta":{"tool_calls":[{"function":{"name":"shell_command","arguments":"{\\"command\\":\\"ls\\"}"}}]}}]}
data: [DONE]
```

---

### Step 2.3 — Multi-provider auto-discovery

**File:** `app/src/ai/local_llm/discovery.rs`

At app startup (and when the settings page opens), probe both known providers
concurrently and report what's available:

```rust
pub struct DiscoveryResult {
    pub provider: LocalLLMProvider,
    pub base_url: String,
    pub models: Vec<LocalModel>,
    pub latency_ms: u64,
    pub is_running: bool,
}

pub async fn auto_discover() -> Vec<DiscoveryResult> {
    // Fire health checks at Ollama and LM Studio concurrently
    let (ollama, lmstudio) = tokio::join!(
        probe_provider(LocalLLMProvider::Ollama,   "http://localhost:11434/v1"),
        probe_provider(LocalLLMProvider::LMStudio, "http://localhost:1234/v1"),
    );
    [ollama, lmstudio].into_iter().flatten().collect()
}
```

**Provider-specific model list parsing:**

| Provider | Endpoint | Response format | Notes |
|---|---|---|---|
| Ollama | `GET /api/tags` | `{"models":[{"name":"gemma3:4b","size":2500000000,...}]}` | Has size + modified date |
| LM Studio | `GET /v1/models` | `{"data":[{"id":"lmstudio-community/gemma-3-4b-it-GGUF",...}]}` | Standard OpenAI format |
| Custom | `GET /v1/models` | OpenAI format | Fall back to user-entered model name if endpoint not found |

**Model display-name normalisation** (for the UI dropdown):
```rust
fn normalize_model_name(raw_id: &str) -> String {
    // "lmstudio-community/gemma-3-4b-it-GGUF" → "Gemma 3 4B (GGUF)"
    // "gemma3:4b"                              → "Gemma 3 4B"
    // "llama3.2:3b-instruct-q4_K_M"            → "Llama 3.2 3B Instruct Q4"
    ...
}
```

---

### Step 2.4 — Local LLM Settings UI

**Files:** `app/src/settings_view/ai_page.rs` (new section) +
`app/src/ai/local_llm/settings.rs`

**Persisted settings:**

| Setting | Type | Default |
|---|---|---|
| `local_llm_enabled` | `bool` | `false` |
| `local_llm_provider` | `LocalLLMProvider` | `Ollama` |
| `local_llm_base_url` | `String` | `""` (empty = use provider default) |
| `local_llm_model` | `String` | `"gemma3:4b"` |
| `local_llm_context_window` | `u32` | `8192` |

**Settings UI layout:**

```
┌─ Local AI Provider ──────────────────────────────────────────┐
│                                                               │
│  ● Enable local AI                              [toggle ON]  │
│                                                               │
│  Provider    [Ollama ▼]  ← dropdown: Ollama / LM Studio /    │
│                               Custom                         │
│                                                               │
│  Status      ● Running  — 3 models available  — 42ms         │
│    or        ○ Not found — Install Ollama at ollama.ai       │
│                                                               │
│  Model       [gemma3:4b ▼]  ← populated from live discovery  │
│              Gemma 3 4B · 2.5 GB · ctx 8192                  │
│                                                               │
│  Base URL    [http://localhost:11434/v1     ]  ← editable     │
│              (leave blank to use provider default)            │
│                                                               │
│  [Test connection]                                            │
│                                                               │
│  ℹ  Ollama:    ollama pull gemma3:4b                         │
│     LM Studio: lmstudio.ai → Local Server tab → Load model   │
└───────────────────────────────────────────────────────────────┘
```

**Behaviour details:**
- Switching the **Provider** dropdown immediately re-fills the Base URL field with
  that provider's default, clears the model list, and fires a new discovery probe.
- The **Status** line auto-updates every 30s in the background (no polling while
  the settings page is closed).
- The **Model** dropdown is populated from the live discovery result. If discovery
  fails, it falls back to a free-text input so the user can type a model name.
- **Test connection** fires `health_check()` + `list_models()` and shows a
  result inline: latency, model count, or a friendly error.
- When **Custom** is selected, the Base URL field is required and the Model field
  is always free-text.

---

### Step 2.5 — Add a local inference path in `generate_multi_agent_output()`

**File:** `app/src/ai/agent/api/impl.rs`

Add a check at the top of the AI dispatch function:

```rust
if LocalLLMSettings::as_ref(ctx).is_enabled() {
    return self.generate_local(input, ctx).await;
}
// ... existing warp_multi_agent_api path continues unchanged
```

The `generate_local()` function:
1. Reads `LocalLLMSettings` to get provider, base URL, and model name
2. Constructs the `LocalLLMClient` with the resolved endpoint
3. Converts `AIAgentInput` messages to `Vec<ChatMessage>` (OpenAI format)
4. Maps tool definitions to OpenAI `tools` JSON
5. Calls `LocalLLMClient::generate()`
6. Converts streamed `ChatChunk` responses back to `AIAgentOutput`

This keeps the existing Warp cloud path fully intact.

---

### Step 2.6 — Bypass server-side model validation

**File:** `app/src/ai/agent_sdk/common.rs`

`validate_agent_mode_base_model_id()` rejects any model ID not in the server-fetched
allowlist. When `local_llm_enabled = true`, skip this validation entirely — any string
is a valid model name.

---

### Step 2.7 — Implement tool calling for local models

The existing agent system drives tools via structured `tool_calls` in the LLM response.
Both Ollama (`>= 0.3`) and LM Studio support OpenAI-compatible function calling with
streaming.

Map existing tool types to OpenAI `tools` JSON schema:

```json
{
  "type": "function",
  "function": {
    "name": "shell_command",
    "description": "Run a shell command in the terminal",
    "parameters": {
      "type": "object",
      "properties": {
        "command": { "type": "string", "description": "The command to execute" }
      },
      "required": ["command"]
    }
  }
}
```

Parse `tool_calls` in the streamed chunks and route them to the existing executors
in `app/src/ai/blocklist/action_model/execute/`.

**Priority order for tool support:**
1. `shell_command` — most important for an AI terminal
2. `read_files` — needed for code context
3. `write_files` / `request_file_edits` — code changes
4. `grep` — codebase search
5. `ask_user_question` — clarification prompts

**Note on small models and tool calling:** Gemma 3 supports function calling but
smaller variants (1B, 2B) are less reliable. Start with `gemma3:4b` or larger.
For models that don't support tool calls natively, implement a fallback
system-prompt-based tool dispatch (XML or JSON in `<think>` tags).

---

### Step 2.8 — Testing with Gemma 3 locally

This is the first concrete test of the full local pipeline. Run through this
checklist before considering Phase 2 complete:

#### Setup (Ollama path)
```bash
# Install Ollama
winget install Ollama.Ollama          # Windows
brew install ollama                   # macOS

# Pull the test model (4B is the sweet spot — fast enough, capable enough)
ollama pull gemma3:4b

# Confirm it's serving the OpenAI-compatible endpoint
curl http://localhost:11434/v1/models
# → {"object":"list","data":[{"id":"gemma3:4b",...}]}
```

#### Setup (LM Studio path)
```
1. Download LM Studio from lmstudio.ai
2. Search for "gemma-3-4b-it" → Download GGUF
3. Go to Local Server tab → Select model → Start Server
4. Default: http://localhost:1234/v1
```

#### Test checklist

| Test | Expected result |
|---|---|
| App starts, no login screen | Terminal opens immediately |
| Settings → AI → Enable Local AI → select Ollama | Status shows "Running", model list populated |
| Settings → AI → Enable Local AI → select LM Studio | Status shows "Running", model list populated |
| Provider switch | Base URL updates, model list refreshes |
| Test connection button | Shows latency + model count |
| Basic chat in AI panel | Gemma responds in streaming |
| `ls` shell command via agent | Agent runs command, shows output |
| `read_files` on a local file | Agent reads and quotes file content |
| Write a file edit | Agent proposes diff, user approves, file changes |
| Disconnect Ollama mid-conversation | Graceful error, not a panic |
| Switch from Ollama to LM Studio mid-session | Settings update applies to next request |
| `gemma3:1b` (small model, weaker tool calling) | Falls back to prompt-based tool dispatch |

#### Known limitations with small models to document

- **Context window:** Gemma 3 4B has an 8K context window by default (128K with
  special config in Ollama). Long conversations will truncate. Expose `context_window`
  as a user-configurable setting.
- **Tool calling reliability:** Smaller models may hallucinate tool call syntax.
  Implement retry logic — if a tool call JSON parse fails, send the raw response
  as text and ask the model to retry with valid JSON.
- **Speed:** On CPU-only hardware, expect 5–20 tokens/sec for 4B. The streaming
  UI handles this gracefully since it renders tokens as they arrive.
- **Model format:** LM Studio model IDs are longer
  (`lmstudio-community/gemma-3-4b-it-GGUF`) and must be passed verbatim — the
  normalisation in `LocalModel::display_name` handles this for the UI only.

---

### Step 2.9 — Status indicator in the AI panel

Add a persistent status chip at the top of the AI input area (next to the model
selector) that shows:

| State | Display |
|---|---|
| Local AI running, model loaded | `● Gemma 3 4B  (Ollama)` in green |
| Local AI enabled but provider not running | `○ Ollama not running` in amber + link to settings |
| Local AI disabled | `Cloud AI` label (if cloud path still available) or nothing |

This gives the user immediate feedback without having to open settings.

---

### ✅ Deliverable

- Settings page shows a **Provider** dropdown: Ollama / LM Studio / Custom
- Switching provider auto-fills the URL and discovers available models
- Both Ollama and LM Studio are probed at startup — whichever is running is used
- `gemma3:4b` is the default model for new installs
- Full agent loop works: chat, shell commands, file reads, file edits
- A status chip in the AI panel shows which provider and model is active
- Graceful errors if no local provider is running, with actionable setup instructions

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
