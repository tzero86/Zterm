# Agent Mode

Zterm's agent mode lets you have a conversation with your local LLM that can take real actions in your terminal — running commands, reading files, and exploring your codebase.

## Starting a conversation

Type `/agent` in the terminal input to start a new agent conversation.

## What the agent can do

The agent has access to two tools:

### `run_shell_command`
Executes a shell command in your current working directory and returns the output.

```
You: what files are in this repo?
Agent: [runs: ls -la / dir]
       Here's what's in your repository: ...
```

### `read_file`
Reads the contents of a file relative to your current directory.

```
You: what does the Cargo.toml say about this project?
Agent: [reads: Cargo.toml]
       This is a Rust workspace with the following members: ...
```

## Multi-turn conversations

The agent maintains context across messages in the same session. You can ask follow-up questions:

```
You: what is this repo about?
Agent: This is Zterm, an open-source terminal...

You: which file handles the AI routing?
Agent: [reads: app/src/ai/agent/api/impl.rs]
       The AI routing is handled in...
```

## Tips

- Be specific: "explore the src/ai directory and summarize what each file does" works better than "what's in this repo?"
- The agent uses a non-streaming mode for tool calls, then returns a final answer
- Maximum 10 tool-call iterations per response to prevent runaway loops
