# Local LLM Setup

Zterm's AI features work entirely locally using [Ollama](https://ollama.com) or [LM Studio](https://lmstudio.ai). No API key, no account, no internet connection required.

## Supported providers

| Provider | Default URL | Notes |
|---|---|---|
| **LM Studio** | `http://localhost:1234` | OpenAI-compatible API |
| **Ollama** | `http://localhost:11434` | Native Ollama API |
| **Custom** | Any URL | Any OpenAI-compatible server |

## Setup with LM Studio

1. Download and install [LM Studio](https://lmstudio.ai)
2. Download a model (e.g. `gemma-3`, `llama-3`, `mistral`)
3. Start the local server in LM Studio (Developer tab → Start Server)
4. In Zterm: **Settings → Profiles → Edit Profile**
5. Set all three model dropdowns to your loaded model
6. Open a new agent session with `/agent`

## Setup with Ollama

1. Install [Ollama](https://ollama.com)
2. Pull a model: `ollama pull llama3`
3. Ollama starts automatically — no extra steps needed
4. In Zterm: **Settings → Profiles → Edit Profile**
5. Select the Ollama model from the dropdowns

## Agent capabilities

The Zterm agent can:

- **Run shell commands** — executes commands in your current working directory
- **Read files** — reads file contents to answer questions about your codebase
- **Multi-turn conversations** — maintains context across multiple messages
- **Tool loops** — automatically chains multiple tool calls to complete complex tasks

## Tips for best results

- Use a model with at least 7B parameters for reliable tool use (e.g. `gemma-3-7b`, `llama3-8b`)
- Instruct mode / chat models work better than base models
- Keep LM Studio's server running before launching Zterm
