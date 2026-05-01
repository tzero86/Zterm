# Configuration

Zterm stores all configuration locally. There is no cloud sync and no account required.

## Config location

| OS | Path |
|---|---|
| Windows | `%LOCALAPPDATA%\zterm\ZtermOss\` |
| macOS | `~/Library/Application Support/dev.zterm.ZtermOss/` |
| Linux | `~/.local/share/dev.zterm.ZtermOss/` |

## Settings UI

Most settings are accessible via the gear icon (⚙) in the top-right corner of the window. Key areas:

- **Appearance** — themes, fonts, opacity
- **Profiles** — AI model selection per profile
- **AI** — agent behaviour, MCP servers, rules, allowed/blocked commands
- **Features** — shell integrations, notifications, startup options
- **About** — version info

## Themes

Zterm supports custom themes in YAML format. Place them in your config directory under `themes/`.
