# Getting Started

## Prerequisites

- Windows 10/11, macOS 13+, or Linux
- [Rust toolchain](https://rustup.rs/) (stable, see `rust-toolchain.toml` for exact version)
- For AI features: [Ollama](https://ollama.com) or [LM Studio](https://lmstudio.ai) running locally

## Building from source

```bash
git clone https://github.com/tzero86/Zterm.git
cd Zterm
cargo run --bin zterm-oss --features gui
```

The first build will take several minutes as it compiles all dependencies. Subsequent builds are incremental.

## First launch

Zterm opens directly to a terminal — no login screen, no account setup. You're ready to go immediately.

## Enabling AI features

1. Open **Settings** (gear icon, top right)
2. Go to **Profiles** and edit your profile
3. Set the **Base Model**, **Full Terminal Use Model**, and **Computer Use Model** dropdowns to your local model (e.g. `gemma` via LM Studio)
4. Start a new agent conversation with `/agent`

See [Local LLM Setup](/ai/local-llm) for detailed configuration.
