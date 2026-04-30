# Introduction

Zterm is an open-source terminal application forked from [Warp](https://github.com/warpdotdev/Warp). The goal is simple: take the best GPU-accelerated terminal available and remove every restriction — no account required, no premium tier, no telemetry, no cloud lock-in.

## Why Zterm?

Warp is an excellent terminal, but many of its best features are gated behind:

- A mandatory login / account creation
- A premium subscription
- Internet connectivity (for AI features)

Zterm removes all of those gates. Every feature works offline, with no account, from the first launch.

## Key differences from Warp

| Feature | Warp | Zterm |
|---|---|---|
| Login required | Yes | **No** |
| AI features | Cloud only (paid) | **Local LLM (free)** |
| Telemetry | Yes | **None** |
| Source available | Partial | **Fully open source** |
| Paywalls | Yes | **None** |

## Local AI

Zterm's AI features are designed to work with local models via [Ollama](https://ollama.com) or [LM Studio](https://lmstudio.ai). The agent can run shell commands, read files, and answer questions about your codebase — entirely on your machine, with no data leaving it.

See [Local LLM Setup](/ai/local-llm) to get started.
