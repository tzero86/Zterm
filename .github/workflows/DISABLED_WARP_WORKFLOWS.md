# Disabled Warp-Dependent Workflows

This document catalogs workflows that depend on Warp-owned infrastructure or actions that are not available in the Zterm fork. These are disabled to prevent CI startup failures.

## Fully Disabled Workflows

### Repo Sync Workflows
**Files:** `repo-sync.yml`
**Purpose:** Synced Zterm fork with upstream Warp repository
**Dependencies:**
- `warpdotdev/repo-sync` GitHub action (private/Warp-owned)
- Requires: `public_repo: warpdotdev/warp`, `private_repo: warpdotdev/warp-internal`
- Requires: `ZTERM_API_KEY` secret

**Status for Fork:** ❌ Not applicable
- Zterm is now an independent fork, not synced with Warp
- Should not re-enable; upstream sync workflow is Warp-specific

**Replacement Approach:** Manual GitHub sync via PR if needed, or maintain as separate fork

---

### Release Management Workflows
**Files:** `create_release.yml`, `cut_new_release_candidate.yml`, `cut_new_releases.yml`, `delete_release.yml`

**Purpose:** Automated release creation and channel versioning
**Dependencies:**
- `warpdotdev/channel-versions` repository (private, Warp-owned)
- Warp Sentry organization (`warpdotdev`)
- Warp S3 bucket (`warp-releases/`)
- Various Warp-specific secrets

**Status for Fork:** ⚠️ Needs replacement
- Current releases are manual or need custom CI setup
- Zterm doesn't have a release channel system yet

**Replacement Approach:**
1. **Option A (Quick):** Use GitHub Releases + Artifacts directly
   - Create releases manually via GitHub CLI or UI
   - Attach built binaries as release artifacts
   
2. **Option B (Recommended):** Build custom release workflow
   - Implement `.github/workflows/release.yml` that:
     - Builds binaries for Linux/macOS/Windows
     - Creates GitHub Release with artifacts
     - Tags releases with semantic versioning
     - (Optional) Upload to Homebrew/other package managers
   - Replace `release_configurations.json` with Zterm-specific config

---

### Feature Flag Cleanup Workflow
**File:** `feature_flag_cleanup.yml`

**Purpose:** Automated cleanup of merged/unused feature flags
**Dependencies:**
- `warpdotdev/oz-agent-action` (Warp-owned AI agent)
- Requires: `ZTERM_API_KEY` secret
- Posts to Slack via Warp's infrastructure

**Status for Fork:** ⚠️ Could be useful but needs replacement
- Feature flags are important for Zterm too (local LLM, orchestration, etc.)

**Replacement Approach:**
1. Use GitHub Actions + scripting instead of Warp's agent action:
   - Parse Cargo.toml for feature definitions
   - Track feature usage in codebase
   - Create PR to remove unused flags
   
2. Or keep manual for now (low-frequency task)

---

### Warp-Owned oz-for-oss Workflows (Local Agent Integrations)
**Files:**
- `comment-on-unready-assigned-issue-local.yml`
- `create-implementation-from-issue-local.yml`
- `create-spec-from-issue-local.yml`
- `enforce-pr-issue-state.yml`
- `remove-stale-issue-labels-on-plan-approved-local.yml`
- `respond-to-pr-comment-local.yml`
- `respond-to-triaged-issue-comment-local.yml`
- `review-pull-request.yml`
- `triage-new-issues-local.yml`
- `trigger-implementation-on-plan-approved-local.yml`
- `update-dedupe-local.yml`
- `update-pr-review-local.yml`
- `update-triage-local.yml`
- `verify-pr-comment-local.yml`

**Purpose:** Automated GitHub issue/PR workflows using Warp's Oz AI agent framework
**Dependencies:**
- `warpdotdev/oz-for-oss` GitHub Action (Warp-owned)
- `warpdotdev/oz-agent-action` (Warp-owned)
- Requires: `ZTERM_API_KEY` or similar Warp secret

**Status for Fork:** ⚠️ Valuable but all use Warp-owned actions
- These workflows automate: issue triage, spec/implementation generation, PR reviews, code cleanup, etc.
- Current setup relies on Warp's private Oz AI agent framework

**Replacement Approach:**
1. **Option A (Most Practical):** Replace with GitHub's native + Claude
   - Use `actions/github-script@v7` for automation
   - Call Claude API directly (via your own action or workflow secrets)
   - Implement custom logic for:
     - Issue triage (add labels, route to reviewers)
     - Spec generation (call Claude to write PRODUCT.md)
     - Implementation (call Claude to implement from spec)
     - PR review (call Claude for code review feedback)
     - Auto-label stale issues

2. **Option B (Manual for now):** Remove these and rely on manual workflows
   - Human-driven process for specs/implementation
   - Less automation, but lower complexity

---

### Build Cache Population Workflow
**File:** `populate_build_cache.yml`

**Purpose:** Warm Rust build cache on schedule
**Dependencies:**
- References `ci.yml` as reusable workflow
- GCP/Warp auth in CI context

**Status for Fork:** ❌ Not critical, but causes startup failures
- Nice-to-have for faster builds, not essential

**Replacement Approach:**
- Remove (CI will still work, just slower first build after cache expiry)
- Or use GitHub Actions cache directly (already in ci.yml)

---

## Workflows Still Enabled

### CI/Quality (Active & Working)
- `ci.yml` - Main test/lint pipeline ✅
- `deploy-docs.yml` - Deploy VitePress docs to GitHub Pages ✅
- `check_approvals.yml` - Check OWNERS file for PR approvals ✅
- `sync-pr-checks.yml` - Sync PR check aggregates ✅
- `close_stale_fix_prs.yml` - Close stale PRs ✅
- `docubot_reply_to_comment.yml` - Docubot AI comments ✅
- `warp_cleanup_fix_prs.yml` - Cleanup stale fix PRs ✅

---

## Recommended Next Steps

### Phase 1 (Immediate): Prevent startup failures
- [x] Disable/remove all Warp-dependent workflows
- [x] Document what each does (this file)
- [ ] Test CI pipeline runs cleanly on `master`

### Phase 2 (Future): Replace with Zterm-specific workflows
- [ ] Implement custom release workflow (Option B recommended)
- [ ] Add issue triage automation with Claude
- [ ] Add spec/implementation generation with Claude
- [ ] Add PR review automation with Claude

### Phase 3 (Long-term): Feature improvements
- [ ] Custom feature flag cleanup workflow
- [ ] GitHub Pages auto-deployment (already done ✅)
- [ ] Changelog auto-generation
- [ ] Release notes auto-generation from commits

---

## Notes for Future Implementation

1. **GitHub API limits:** Custom workflows using Claude API should respect GitHub API rate limits
2. **Secrets:** Store Claude API key (if using direct calls) in repository secrets
3. **Labels & Triage:** Define Zterm-specific issue labels and triage rules
4. **Release strategy:** Decide on versioning (semver recommended) and release cadence
5. **Documentation:** Keep this file updated as new workflows are added

---

**Last Updated:** 2026-05-01  
**Decision Made By:** Zterm team (fork independence phase)
