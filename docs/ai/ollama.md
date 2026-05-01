# Ollama

[Ollama](https://ollama.com) is the easiest way to run local LLMs on macOS, Linux, and Windows.

## Installation

Download from [ollama.com](https://ollama.com) or:

```bash
# macOS / Linux
curl -fsSL https://ollama.com/install.sh | sh

# Windows
# Download the installer from https://ollama.com/download
```

## Pull a model

```bash
ollama pull llama3        # Meta Llama 3 8B
ollama pull gemma3        # Google Gemma 3
ollama pull mistral       # Mistral 7B
ollama pull qwen2.5-coder # Great for coding tasks
```

## Verify it's running

```bash
ollama list              # shows installed models
curl http://localhost:11434/api/tags   # should return JSON
```

## Connect to Zterm

Zterm auto-detects Ollama at `http://localhost:11434`. Once Ollama is running with at least one model pulled, your models will appear in **Settings → Profiles → Edit Profile** model dropdowns.
