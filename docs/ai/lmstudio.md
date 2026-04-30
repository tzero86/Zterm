# LM Studio

[LM Studio](https://lmstudio.ai) provides a desktop app for downloading and running local LLMs with an OpenAI-compatible API server.

## Installation

Download from [lmstudio.ai](https://lmstudio.ai) — available for Windows, macOS, and Linux.

## Start the local server

1. Open LM Studio
2. Go to the **Developer** tab (or **Local Server** tab depending on version)
3. Select a model and click **Start Server**
4. The server runs at `http://localhost:1234` by default

## Connect to Zterm

Zterm auto-detects LM Studio at `http://localhost:1234`. Once the server is running, your loaded model will appear in **Settings → Profiles → Edit Profile** model dropdowns.

## Recommended models

For agent/tool use (running commands, reading files):

| Model | Size | Notes |
|---|---|---|
| `gemma-3-12b-it` | 12B | Good balance of speed and capability |
| `llama-3.1-8b-instruct` | 8B | Fast, reliable tool use |
| `mistral-7b-instruct` | 7B | Lightweight, good for older hardware |
| `qwen2.5-coder-7b-instruct` | 7B | Optimized for code tasks |
