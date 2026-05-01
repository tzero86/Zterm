# ZTERM.md Project Context Files

**Version:** 1.0  
**Date:** 2026-05-01  
**Status:** Complete

---

## Overview

`ZTERM.md` is the primary project context file for Zterm projects. It provides agents and AI assistants with important information about your project structure, conventions, and goals.

Previously, projects used `WARP.md` for this purpose. **Zterm now prioritizes `ZTERM.md` while maintaining backwards compatibility with `WARP.md`.**

---

## What Is a Project Context File?

A project context file (ZTERM.md, WARP.md, or AGENTS.md) is a markdown file placed in your project root that tells agents:

- Project structure and organization
- Key files and their purposes
- Development conventions
- Build and test commands
- Architecture decisions
- Important constraints or gotchas
- Links to relevant documentation

**File Priority:**
1. `ZTERM.md` ← Primary (checked first)
2. `WARP.md` ← Fallback (for backwards compatibility)
3. `AGENTS.md` ← Fallback

---

## Creating a ZTERM.md File

### Basic Template

```markdown
# Project Name

## Overview
[One-paragraph description of what this project does]

## Key Files & Directories
- `/src/main.rs` - Application entry point
- `/tests/` - Unit and integration tests
- `/docs/` - Documentation
- `/crates/` - Workspace members (if applicable)

## Development Setup

### Prerequisites
- Rust 1.70+ (or your minimum version)
- [Any other dependencies]

### Building
```bash
cargo build --release
```

### Running
```bash
cargo run -- [arguments]
```

### Testing
```bash
# All tests
cargo test

# Specific test
cargo test test_name

# Integration tests
cargo test --test integration_tests
```

## Important Conventions

### Code Style
- [Your linting/formatting standards]
- Run `cargo fmt` before commits
- Run `cargo clippy` to check for warnings

### Naming
- [File naming conventions]
- [Function naming conventions]
- [Variable naming conventions]

### Testing
- All public APIs require tests
- [Test structure preferences]
- [Coverage expectations]

## Architecture

### Module Organization
[Describe how modules are organized]

### Key Components
- [Component 1]: [Purpose]
- [Component 2]: [Purpose]
- [Component 3]: [Purpose]

### Data Flow
[Describe how data flows through the system]

## Known Constraints

- [Constraint 1]
- [Constraint 2]
- [Breaking changes in development]

## Links
- [Issue Tracker](https://linear.app/...)
- [Documentation](https://docs...)
- [Architecture Decision Record](./docs/adr/)
- [Contributing Guide](./CONTRIBUTING.md)
```

### Minimal Template (For Small Projects)

```markdown
# Project Name

## Overview
[What this project does]

## Build & Test
```bash
cargo build
cargo test
```

## Key Files
- `src/main.rs` - Entry point
- `tests/` - Tests

## Links
- [Documentation](./README.md)
```

---

## Best Practices

### 1. Keep It Current

Update ZTERM.md when:
- Project structure changes (new modules, directories)
- Build/test process changes
- Key technologies or dependencies change
- Architecture decisions are made
- New important conventions are established

**Tip:** Create a reminder to review ZTERM.md with each major release.

### 2. Be Specific

❌ **Vague:**
```markdown
## Files
- Source files
- Tests
- Docs
```

✓ **Specific:**
```markdown
## Key Files
- `/src/main.rs` - Application entry point
- `/src/parser.rs` - Input parsing logic (uses `nom` for error handling)
- `/tests/integration/` - End-to-end tests
- `/docs/architecture.md` - System design overview
```

### 3. Document Gotchas

Include things that surprised or confused you:

```markdown
## Important Notes

- **Workspace members:** Build all members with `cargo build --all`
- **Feature flags:** The `local_fs` feature controls filesystem access
- **Database:** Run migrations with `cargo run --bin migrate` before first run
- **Async:** All I/O is async; blocking calls may panic
```

### 4. Link to Decision Records

Reference architectural decisions:

```markdown
## Decisions

- **Why Rust?** See [ADR-001](./docs/adr/001-language-choice.md)
- **Why async/await?** See [ADR-003](./docs/adr/003-async-runtime.md)
```

### 5. Include Development Workflows

Tell agents how to accomplish common tasks:

```markdown
## Common Tasks

### Adding a New Feature
1. Create a feature branch: `git checkout -b feat/my-feature`
2. Add tests in `tests/`
3. Implement feature in `src/`
4. Run `cargo test` and `cargo fmt`
5. Open a PR

### Running in Debug Mode
```bash
RUST_LOG=debug cargo run
```

### Profiling
```bash
cargo build --release
perf record -g ./target/release/app
perf report
```
```

---

## ZTERM.md vs WARP.md

### If You Have WARP.md

Zterm will read WARP.md if ZTERM.md doesn't exist. You have three options:

**Option 1: Keep WARP.md (No Action)**
- Zterm falls back to WARP.md
- Agents see your existing context
- Migrate whenever you're ready

**Option 2: Rename to ZTERM.md**
```bash
mv WARP.md ZTERM.md
```
- Clearer that it's Zterm-specific
- No functional change (same content works)

**Option 3: Create New ZTERM.md**
- Update content for Zterm-specific information
- Delete or keep WARP.md as reference
- Gradually evolve your documentation

### Differences

| Aspect | ZTERM.md | WARP.md |
|--------|----------|---------|
| **Priority** | Checked first | Fallback |
| **Agent Brand** | Zterm | Warp (legacy) |
| **Status** | Primary (recommended) | Legacy (supported) |
| **Performance** | Optimal | Slight delay (fallback lookup) |

**Recommendation:** Use ZTERM.md for all new projects. Migrate existing WARP.md files when convenient.

---

## Examples

### Example 1: Rust Library

```markdown
# Zterm Core Library

## Overview
Core utilities and traits for Zterm, including the SkillProvider system, project context handling, and AI agent coordination.

## Key Files
- `/src/lib.rs` - Public API exports
- `/src/skills/` - Skill provider implementation
- `/src/context/` - Project context system
- `/src/agent/` - Agent coordination logic

## Build & Test
```bash
# Build the library
cargo build

# Run all tests (including doctests)
cargo test --all

# Build documentation
cargo doc --open
```

## Important Conventions
- All public types must have doc comments
- Public APIs require unit tests
- Use `Result<T, Error>` for fallible operations
- Async functions use Tokio runtime

## Architecture
```
SkillProvider enum
├── Zterm (primary)
├── Claude
└── OpenAI

ProjectContext
├── ZTERM.md (primary)
├── WARP.md (fallback)
└── AGENTS.md (fallback)

Agent
├── AIAgentCitation
└── SkillReference
```

## Links
- [Architecture Decisions](./docs/adr/)
- [Contributing Guide](./CONTRIBUTING.md)
- [API Documentation](https://docs.rs/zterm-core/)
```

### Example 2: Web Application

```markdown
# Zterm Web Dashboard

## Overview
Interactive dashboard for monitoring and managing Zterm instances across local and cloud environments.

## Directory Structure
```
src/
├── main.rs           - Application entry point
├── routes/           - HTTP route handlers
├── models/           - Data models and schemas
├── services/         - Business logic
└── templates/        - HTML templates
public/              - Static assets (CSS, JS)
tests/               - Integration tests
docs/                - Architecture and guides
```

## Development Setup

### Prerequisites
- Node.js 18+
- Python 3.9+
- PostgreSQL 14+

### Build & Run
```bash
# Install dependencies
npm install

# Build frontend
npm run build

# Start development server
cargo run --bin dev-server
```

### Testing
```bash
# Backend tests
cargo test

# Frontend tests
npm test

# Integration tests
npm run test:e2e
```

## Environment Variables
- `DATABASE_URL` - PostgreSQL connection string
- `ZTERM_API_KEY` - API authentication key
- `DEBUG` - Enable debug logging

## Important Notes
- Database migrations run automatically on startup
- Environment variables must be set before starting
- WebSocket connections require `wss://` in production

## Links
- [API Docs](./docs/api/)
- [Database Schema](./docs/schema.md)
- [Frontend Guide](./frontend/README.md)
```

---

## Migrating from WARP.md

### Step 1: Examine Your Current WARP.md
```bash
cat WARP.md
```

### Step 2: Create ZTERM.md (Copy & Update)
```bash
cp WARP.md ZTERM.md
```

### Step 3: Update References (Optional)
Update any Warp-specific references to Zterm:
```bash
# In the file
sed -i 's/Warp/Zterm/g' ZTERM.md
sed -i 's/warp/zterm/g' ZTERM.md
```

### Step 4: Commit Changes
```bash
git add ZTERM.md
git commit -m "docs: add ZTERM.md project context"
```

### Step 5: Verify Agents See It
- Open Zterm
- In agent view, confirm ZTERM.md is loaded
- Check that agent responses reference correct project info

### Step 6: Clean Up (Optional)
```bash
# Remove WARP.md after verifying ZTERM.md works
rm WARP.md
git add WARP.md
git commit -m "docs: remove legacy WARP.md"
```

---

## FAQ

### Q: Do I need both ZTERM.md and WARP.md?

**A:** No. ZTERM.md is sufficient. Keep WARP.md only if you need backwards compatibility with Warp-era tooling.

### Q: What if I don't create a ZTERM.md?

**A:** Zterm will:
1. Look for ZTERM.md (not found)
2. Fall back to WARP.md (if it exists)
3. Fall back to AGENTS.md (if it exists)
4. Continue without project context (agents still work, just less informed)

### Q: Can I have multiple project context files?

**A:** Zterm reads only one (in priority order). Create just one file per project.

### Q: How often should I update ZTERM.md?

**A:** Whenever project structure or conventions change. A good practice is to review and update during:
- Major releases
- Architecture changes
- New contributor onboarding
- Quarterly reviews

### Q: Do agents automatically find ZTERM.md?

**A:** Yes. If ZTERM.md exists in your project root (or anywhere in the path Zterm is working with), agents will automatically read it.

### Q: Can I use ZTERM.md outside of Zterm?

**A:** Absolutely. It's just markdown documentation. Any tool that reads project context files can use it.

---

## Related Documentation

- **[REBRANDING.md](./REBRANDING.md)** - Overview of rebranding changes
- **[MIGRATION_GUIDE.md](./MIGRATION_GUIDE.md)** - Step-by-step migration instructions
- **[README.md](../README.md)** - Main project documentation

---

**Last Updated:** 2026-05-01  
**Maintained by:** Zterm Team  
**License:** MIT/AGPL (same as Zterm)
