# Zterm Rebranding Migration Guide

**Version:** 1.0  
**Date:** 2026-05-01  
**Status:** Complete

---

## Quick Start

### For Most Users (No Action Required)

✓ Your existing `~/.warp/` configurations will continue to work.  
✓ No immediate migration needed.  
✓ Migrate at your own pace when convenient.

### For Users with Saved Agent Conversations

⚠️ **Skill references in saved conversations may break.**  
See [Skill ID Migration](#skill-id-migration) section below.

### For Claude Code Users

⚠️ **Update the notification plugin manually.**  
See [Claude Code Plugin Update](#claude-code-plugin-update) section below.

---

## Migration Scenarios

Choose the scenario that matches your situation:

1. [I want to migrate my tab configs](#scenario-1-migrate-tab-configs)
2. [I want to migrate my custom skills](#scenario-2-migrate-custom-skills)
3. [I want to add ZTERM.md to my project](#scenario-3-add-zterm-project-context)
4. [I have broken skill references in agent conversations](#scenario-4-fix-broken-skill-references)
5. [I use the Claude Code plugin](#scenario-5-update-claude-code-plugin)
6. [I'm a developer updating code](#scenario-6-developer-migration)

---

## Scenario 1: Migrate Tab Configs

### Status Quo
Tab configuration files are in `~/.warp/tab_configs/`:
```
~/.warp/tab_configs/
├── dev_setup.toml
├── testing_layout.toml
└── ...
```

### What's Changing
New tab configs will be saved to `~/.zterm/tab_configs/`, but old ones still load from `~/.warp/`.

### Migration Steps

#### Option A: Manual Migration (Recommended for Important Configs)

1. **List your existing configs:**
   ```bash
   ls -la ~/.warp/tab_configs/
   ```

2. **Create the new directory:**
   ```bash
   mkdir -p ~/.zterm/tab_configs
   ```

3. **Copy important configs:**
   ```bash
   cp ~/.warp/tab_configs/dev_setup.toml ~/.zterm/tab_configs/
   cp ~/.warp/tab_configs/testing_layout.toml ~/.zterm/tab_configs/
   ```

4. **Verify they load:**
   - Open Zterm
   - Click `+` → "New Tab: Dev Setup"
   - Confirm the layout loads correctly

5. **Optional: Delete old configs after verifying**
   ```bash
   rm ~/.warp/tab_configs/dev_setup.toml
   ```

#### Option B: Bulk Migration (Copy Everything)

If you want to migrate all configs at once:

```bash
# Create directory
mkdir -p ~/.zterm/tab_configs

# Copy all configs
cp -r ~/.warp/tab_configs/* ~/.zterm/tab_configs/

# Verify
ls -la ~/.zterm/tab_configs/
```

**Note:** Old configs in `~/.warp/` continue to work even after copying. You can delete them later.

#### Option C: No Migration (Keep Using Existing Configs)

Leave your configs in `~/.warp/tab_configs/`. Zterm will continue to load them.

**Use this if:**
- You don't create new configs often
- You want to migrate gradually
- You're happy with current setup

---

## Scenario 2: Migrate Custom Skills

### Status Quo
Custom skills are in `~/.warp/skills/`:
```
~/.warp/skills/
├── my-custom-skill/
│   └── SKILL.md
├── another-skill/
│   └── SKILL.md
└── ...
```

### What's Changing
New custom skills should be added to `~/.zterm/skills/`, but old ones still load from `~/.warp/`.

### Migration Steps

#### Option A: Copy Important Skills

1. **Identify skills to migrate:**
   ```bash
   ls -la ~/.warp/skills/
   ```

2. **Create new skills directory:**
   ```bash
   mkdir -p ~/.zterm/skills
   ```

3. **Copy specific skills:**
   ```bash
   cp -r ~/.warp/skills/my-custom-skill ~/.zterm/skills/
   cp -r ~/.warp/skills/another-skill ~/.zterm/skills/
   ```

4. **Test the skills:**
   - Open Zterm
   - Run `/skills` command
   - Verify your custom skills appear

#### Option B: Copy All Skills

If you want all skills in the new location:

```bash
# Create directory
mkdir -p ~/.zterm/skills

# Copy all skills
cp -r ~/.warp/skills/* ~/.zterm/skills/

# Verify
ls -la ~/.zterm/skills/
```

#### Option C: Keep Existing Skills (No Migration)

Custom skills in `~/.warp/skills/` continue to load.

**Use this if:**
- You don't modify skills often
- You prefer to migrate later
- Both locations work fine

---

## Scenario 3: Add ZTERM Project Context

### Status Quo (Optional)
You may have a `WARP.md` file in your project:
```
project-root/
├── WARP.md
└── ...
```

### What's Changing
Projects should use `ZTERM.md` instead (though `WARP.md` still works as fallback).

### Migration Steps

#### Option A: Create ZTERM.md (Recommended)

1. **Check if you have a WARP.md:**
   ```bash
   ls -la WARP.md
   ```

2. **If yes, copy it to ZTERM.md:**
   ```bash
   cp WARP.md ZTERM.md
   ```
   
   Or **create a new one** with Zterm-specific context:
   ```bash
   cat > ZTERM.md << 'EOF'
   # Zterm Project Context
   
   ## Overview
   [Your project description]
   
   ## Key Files
   - `/src/main.rs` - Entry point
   - `/tests/` - Test suite
   
   ## Development Setup
   [Your setup instructions]
   
   ## Commands
   - `cargo build` - Build the project
   - `cargo test` - Run tests
   EOF
   ```

3. **Verify Zterm sees the file:**
   - Open Zterm
   - In agent view, confirm project context loads
   - Look for references to ZTERM.md

4. **Optional: Keep or Remove WARP.md**
   - If you created a new ZTERM.md, you can remove WARP.md:
     ```bash
     rm WARP.md
     ```
   - Or keep both (ZTERM.md takes precedence)

#### Option B: Keep WARP.md (No Action)

Your existing `WARP.md` continues to work as project context.

**Use this if:**
- Your project context is stable
- You want to migrate later
- No need to change anything now

---

## Scenario 4: Fix Broken Skill References

### Problem
You see errors like: `Could not find skill: @warp-skill:my-skill`

### Cause
Saved agent conversations reference the old `@warp-skill:` format, which no longer resolves.

### Solution

#### Option A: Re-invoke the Skill (Recommended)

In your agent conversation, ask for the skill again:

```
Ask Zterm: "Can you help with [task]?"
# Or invoke explicitly: /skills → zterm-skill-name
```

#### Option B: Update Saved Conversations Manually

If you have exported or saved conversations:

1. **Find and replace in text:**
   ```bash
   sed -i 's/@warp-skill:/@zterm-skill:/g' myconversation.txt
   ```

2. **Update in database (if applicable):**
   - Contact support or check Zterm documentation for persisted conversation storage

#### Option C: Start Fresh

Create a new agent session with the skill reference:
```
/skills → [your skill name]
```

---

## Scenario 5: Update Claude Code Plugin

### Problem
Claude Code integration using `claude-code-warp` plugin stops working.

### Cause
Plugin package renamed from `claude-code-warp` to `claude-code-zterm`.

### Solution

**In Claude Code terminal, run:**

```bash
# Step 1: Remove old plugin
claude plugin marketplace remove claude-code-warp

# Step 2: Add new marketplace entry
claude plugin marketplace add tzero86/claude-code-zterm

# Step 3: Install new plugin
claude plugin install zterm@claude-code-zterm

# Step 4: Reload plugins
/reload-plugins

# Step 5: Restart Claude Code (if needed)
# Type: /exit
```

### Verify Installation

In Claude Code terminal:
```bash
claude plugin list
```

You should see: `zterm@claude-code-zterm` (not `warp@claude-code-warp`)

### Troubleshooting

**Plugin still shows as "warp":**
- Run `/exit` to fully restart Claude Code
- Try Step 3-4 again

**"Permission denied" errors:**
- Ensure Claude Code has permission to install plugins
- Check your Claude Code marketplace configuration

**Still not working:**
- Check Claude Code plugin marketplace documentation
- Open issue with Claude Code team

---

## Scenario 6: Developer Migration

### Code Changes

#### Update Skill Provider References

**Before:**
```rust
if provider == SkillProvider::Warp {
    println!("Using Warp skills");
}
```

**After:**
```rust
if provider == SkillProvider::Zterm {
    println!("Using Zterm skills");
}
```

#### Update Skill ID References

**Before:**
```rust
const SKILL_ID: &str = "@warp-skill:my-skill";
```

**After:**
```rust
const SKILL_ID: &str = "@zterm-skill:my-skill";
```

#### Update Configuration Paths

**Before:**
```rust
let config_dir = dirs::home_dir().join(".warp/tab_configs");
```

**After:**
```rust
let config_dir = dirs::home_dir().join(".zterm/tab_configs");
```

#### Update Project Context Handling

**Before:**
```rust
fn load_project_rules() -> Option<String> {
    fs::read_to_string("WARP.md").ok()
}
```

**After:**
```rust
fn load_project_rules() -> Option<String> {
    fs::read_to_string("ZTERM.md")
        .or_else(|_| fs::read_to_string("WARP.md"))
        .ok()
}
```

### Documentation Updates

Update any documentation, comments, or README files that reference:
- `SkillProvider::Warp` → `SkillProvider::Zterm`
- `~/.warp/` → `~/.zterm/`
- `@warp-skill:` → `@zterm-skill:`
- `warpdotdev` → `tzero86`

### Testing

After migration:

```bash
# Run tests to ensure compatibility
cargo test

# Check formatting
cargo fmt --check

# Check linting
cargo clippy

# Build the project
cargo build --release
```

---

## Troubleshooting

### "Skill not found" Errors

**Cause:** Old `@warp-skill:` format or skill in wrong location  
**Solution:** Use new `@zterm-skill:` format or check `~/.zterm/skills/`

### Configuration Not Loading

**Cause:** File in `~/.warp/` instead of `~/.zterm/`  
**Solution:** Copy config to `~/.zterm/`, or check if old location still works

### Plugin Errors in Claude Code

**Cause:** Outdated `claude-code-warp` plugin  
**Solution:** Follow Scenario 5 (plugin update) above

### ZTERM.md Not Found

**Cause:** File not in project root  
**Solution:** Create ZTERM.md or use WARP.md as fallback (see Scenario 3)

### Mixed Configuration Locations

**Cause:** Configs in both `~/.warp/` and `~/.zterm/`  
**Solution:** This is fine! Zterm loads from both. Clean up old location when ready.

---

## Rollback

If you need to revert migrations:

### Restore Tab Configs
```bash
# Copy back from ~/.zterm/ to ~/.warp/
cp -r ~/.zterm/tab_configs/* ~/.warp/tab_configs/
```

### Restore Custom Skills
```bash
# Copy back from ~/.zterm/ to ~/.warp/
cp -r ~/.zterm/skills/* ~/.warp/skills/
```

### Restore Project Context
```bash
# If you have ZTERM.md and want WARP.md instead
mv ZTERM.md WARP.md
```

---

## Timeline

| Phase | Action | Status |
|-------|--------|--------|
| **Now** | Use both old and new paths | ✓ Active |
| **3 months** | Gradual migration to new paths | → Coming |
| **6 months** | Legacy paths still supported | → Coming |
| **12 months+** | Consider deprecation (TBD) | → Future |

**Note:** No hard cutoff date. Legacy support will continue indefinitely unless announced.

---

## Getting Help

- **Stuck on migration?** Check related scenario above
- **Found a bug?** Open an issue with details
- **Need clarification?** See main [REBRANDING.md](REBRANDING.md) guide

---

**Last Updated:** 2026-05-01  
**Maintained by:** Zterm Team  
**License:** MIT/AGPL (same as Zterm)
