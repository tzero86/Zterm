# Zterm Rebranding Guide

**Version:** 1.0  
**Date:** 2026-05-01  
**Status:** Complete

---

## Overview

This document describes the comprehensive rebranding effort from **Warp** to **Zterm**, which updates all user-facing components, configuration paths, and infrastructure to reflect Zterm's independent identity.

### What Changed

**Zterm** is now the primary brand across:
- ✓ Skill discovery & AI agent interactions
- ✓ Configuration file paths (`~/.zterm/`)
- ✓ GitHub organization (tzero86)
- ✓ Plugin naming & integration
- ✓ Credential messages & UI text
- ✓ Project context files (ZTERM.md)

**Backwards Compatibility:** All changes maintain compatibility with existing Warp-era configurations and data formats.

---

## For Users

### Configuration Paths

**Zterm now uses:**
- `~/.zterm/tab_configs/` - Tab configuration files
- `~/.zterm/skills/` - Custom skill definitions
- `ZTERM.md` - Project context (primary)

**Legacy paths still work:**
- `~/.warp/tab_configs/` - Still supported (fallback)
- `~/.warp/skills/` - Still supported (fallback)
- `WARP.md` - Still supported (fallback)

**Recommendation:** Gradually migrate new configs to `~/.zterm/` directory. Both can coexist indefinitely.

### Credential Loading

If you see "Zterm will not load your AWS CLI credentials until AWS Bedrock is enabled..." this is normal—it's the new message indicating AWS credential loading status in Zterm.

### Plugin Updates (Claude Code Integration)

If you use the Claude Code plugin with Zterm:

**For the Zterm notification plugin:**
- Plugin package: `claude-code-zterm` (was `claude-code-warp`)
- Plugin ID: `zterm@claude-code-zterm` (was `warp@claude-code-warp`)

**To update:**
```bash
# In Claude Code terminal
claude plugin marketplace remove claude-code-warp
claude plugin marketplace add tzero86/claude-code-zterm
claude plugin install zterm@claude-code-zterm
# Restart Claude Code
```

---

## For Developers

### Key Architectural Changes

#### 1. Skill Provider Enum

**Internal representation now uses `SkillProvider::Zterm`** (was `SkillProvider::Warp`)

```rust
// Zterm now defines provider as:
enum SkillProvider {
    Zterm,              // <- Primary
    Claude,
    OpenAI,
    // ...
}
```

- Wire protocol unchanged (API still uses compatible types)
- Skill discovery UI updated
- User-facing references all updated

#### 2. Bundled Skill ID Format

**New format:** `@zterm-skill:skill-name`  
**Old format:** `@warp-skill:skill-name`

This is a **breaking change** for persisted skill references. See migration guide below.

#### 3. Project Context Rules

**Priority order** for rule files:
1. `ZTERM.md` (primary, new)
2. `WARP.md` (fallback, legacy)
3. `AGENTS.md` (fallback)

Create `ZTERM.md` in your project root for Zterm-specific project context. Old `WARP.md` files continue to work.

#### 4. Citation Types

AI responses now cite `AIAgentCitation::ZtermDocumentation` instead of `WarpDocumentation`.

### Filesystem Structure

Updated configuration directory structure:

```
~/.zterm/                           # Primary (new)
├── tab_configs/                   # Tab layouts
├── skills/                        # Custom skills
└── config.toml

~/.warp/                           # Legacy (still supported)
├── tab_configs/                   # Tab layouts (fallback)
├── skills/                        # Custom skills (fallback)
└── config.toml
```

Both directories can coexist. New configurations should use `~/.zterm/`.

### GitHub Organization Change

**Zterm repositories:**
- Main: `tzero86/Zterm`
- Claude Code plugin: `tzero86/claude-code-zterm`
- Documentation: `tzero86/Zterm-docs` (if applicable)

**Deprecated (Warp):**
- `warpdotdev/warp`
- `warpdotdev/claude-code-warp`

Update any git remotes, issue URLs, or documentation links.

---

## Migration Guide

### For End Users

#### 1. Tab Configuration Files

No action required—existing `~/.warp/tab_configs/*.toml` files continue to work.

**To migrate to new path (optional):**
```bash
mkdir -p ~/.zterm/tab_configs
cp ~/.warp/tab_configs/*.toml ~/.zterm/tab_configs/
# New tabs will save to ~/.zterm/tab_configs/
```

#### 2. Custom Skills

Existing skills in `~/.warp/skills/` continue to load.

**To migrate to new path (optional):**
```bash
mkdir -p ~/.zterm/skills
cp ~/.warp/skills/* ~/.zterm/skills/
```

#### 3. Project Context Rules

**Check if you have a WARP.md in your project:**
```bash
find . -name "WARP.md" -type f
```

**To migrate (recommended for clarity):**
1. Create `ZTERM.md` with your current project context
2. Keep or remove `WARP.md` (both work, but `ZTERM.md` takes precedence)

#### 4. Persisted Skill References

**Old format:** `@warp-skill:xxx`  
**New format:** `@zterm-skill:xxx`

If you have saved agent conversations referencing `@warp-skill:xxx`, they may not resolve to skills in the new version. Recreate these references with the new `@zterm-skill:` format or use skill names directly.

### For Developers

#### 1. Update Code References

If your code references `SkillProvider::Warp`:
```rust
// Old
if provider == SkillProvider::Warp { ... }

// New
if provider == SkillProvider::Zterm { ... }
```

#### 2. Update Skill References

If you hardcoded skill references:
```rust
// Old
skill_id = "@warp-skill:my-skill"

// New
skill_id = "@zterm-skill:my-skill"
```

#### 3. Update Configuration Paths

If you hardcoded paths:
```rust
// Old
let path = "~/.warp/tab_configs/";

// New
let path = "~/.zterm/tab_configs/";
```

Both paths are still supported by the app, but should use the new path for new code.

#### 4. Update Project Context Handling

If you generate or read project rules:
```rust
// Old - only checks WARP.md
if let Some(rules) = read_warp_md() { ... }

// New - checks ZTERM.md first, then WARP.md as fallback
if let Some(rules) = read_zterm_md().or_else(|| read_warp_md()) { ... }
```

---

## Backwards Compatibility Summary

| Component | Legacy | Current | Compatibility |
|-----------|--------|---------|---|
| **Skill Provider** | `SkillProvider::Warp` | `SkillProvider::Zterm` | Internal only—wire protocol unchanged |
| **Skill IDs** | `@warp-skill:xxx` | `@zterm-skill:xxx` | ⚠️ Breaking—old refs won't resolve |
| **Config Paths** | `~/.warp/...` | `~/.zterm/...` | ✓ Both supported indefinitely |
| **Project Rules** | `WARP.md` | `ZTERM.md` | ✓ WARP.md still works as fallback |
| **Plugin** | `claude-code-warp` | `claude-code-zterm` | ⚠️ Breaking—requires manual update |
| **Credentials UI** | "Warp will not..." | "Zterm will not..." | ✓ Cosmetic only |

---

## FAQ

### Q: Do I need to migrate my configuration?

**A:** No, existing configurations in `~/.warp/` continue to work indefinitely. Migrate when you're ready, or not at all.

### Q: Will old `@warp-skill:` references work?

**A:** No, they will fail to resolve. Use `@zterm-skill:` format or skill names directly in new interactions.

### Q: What about the Claude Code plugin?

**A:** Update from `claude-code-warp` to `claude-code-zterm` manually. See plugin update instructions above.

### Q: Can I use both WARP.md and ZTERM.md?

**A:** Yes, but `ZTERM.md` takes precedence. If you have both, Zterm reads `ZTERM.md` first.

### Q: Is the wire protocol changing?

**A:** No, the wire protocol remains compatible with Warp-era systems. Only user-facing names and paths changed.

### Q: What about warpdotdev dependencies?

**A:** Migrate to tzero86 organization. See GitHub organization section above.

---

## Related Documentation

- **[MIGRATION_GUIDE.md](MIGRATION_GUIDE.md)** - Detailed step-by-step migration for all scenarios
- **[CLI_COMMANDS.md](#)** - Command reference for configuration management
- **[PROJECT_CONTEXT.md](#)** - Guide to ZTERM.md project context files

---

## Support

If you encounter issues during the rebranding transition:

1. **Check this guide** - Most common scenarios covered above
2. **Review migration guide** - Step-by-step instructions for specific use cases
3. **Open an issue** - Report bugs or compatibility problems

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-05-01 | Initial release—complete rebranding documentation |

---

**Last Updated:** 2026-05-01  
**Maintained by:** Zterm Team  
**License:** MIT/AGPL (same as Zterm)
