# Zterm Release Process

This document describes how to create and publish Zterm releases using the custom GitHub Actions workflow.

## Release Channels

Zterm uses three release channels:

1. **Dev** (`dev`): Nightly development builds
   - Prerelease versions
   - Released frequently for testing
   - Example: `0.1.0-dev.1`

2. **Preview** (`preview`): Testing/beta releases
   - Prerelease versions
   - Released weekly or before major features
   - Example: `0.2.0-preview.1`

3. **Stable** (`stable`): Production releases
   - Stable semantic versions
   - Released when features are ready for general use
   - Example: `0.2.0`

## Creating a Release

### Prerequisites

- Push all changes to the appropriate branch (usually `master`)
- Ensure CI passes (all tests, linting, formatting)
- Update `CHANGELOG.md` or release notes if needed

### Step 1: Trigger the Release Workflow

Go to **Actions** → **Create Release** and click **Run workflow**.

Or use GitHub CLI:

```bash
gh workflow run release.yml \
  -f version=0.2.0 \
  -f channel=stable \
  -f create_tag=true
```

### Step 2: Fill in Release Details

The workflow will prompt for:

- **version**: Release version in semantic versioning format (e.g., `0.2.0`, `0.1.0-rc.1`)
- **channel**: Release channel (`dev`, `preview`, or `stable`)
- **create_tag**: Whether to create a git tag (default: true)

### Step 3: Monitor the Workflow

The workflow will:

1. **Validate** the version format and channel
2. **Build** binaries for:
   - Linux (x86_64)
   - macOS (x86_64)
   - Windows (x86_64)
3. **Create a GitHub Release** with:
   - Automatic release notes from git history
   - Uploaded binary artifacts
   - Marked as prerelease if `channel != stable`
4. **Create a git tag** (if enabled)
5. **Cleanup** temporary artifacts

## Version Numbering

Follow [Semantic Versioning 2.0.0](https://semver.org/):

- **MAJOR**: Breaking changes (API changes, major features)
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Examples

```
0.1.0           # First release
0.1.1           # Bug fix
0.2.0           # New feature
1.0.0           # Major release / API stable
1.0.0-rc.1      # Release candidate
0.2.0-dev.1     # Dev preview
```

## Release Schedule

- **Dev**: As needed (continuous deployment)
- **Preview**: Weekly or before major milestones
- **Stable**: When features are ready for general use

## After Release

1. **Verify the Release**: Check the GitHub Release page for correct binaries and notes
2. **Test Binaries**: Download and test on each platform before announcing
3. **Announce**: Post release notes in:
   - GitHub Releases page (automatic)
   - README.md (if major version bump)
   - Project website/blog (if applicable)

## Troubleshooting

### Build Fails for Specific Platform

- Check the workflow logs for platform-specific errors
- Common issues:
  - Missing build dependencies (see `./script/bootstrap`)
  - Rust version mismatch (check `rust-toolchain.toml`)
  - Platform-specific code issues (check `rustc --version`)

### Release Already Exists

- The workflow will fail if the tag already exists
- Delete the tag locally and remotely, then re-run:
  ```bash
  git tag -d v0.2.0
  git push origin --delete v0.2.0
  ```

### Need to Retract a Release

1. Delete the GitHub Release (doesn't delete the tag)
2. Optionally delete the git tag:
   ```bash
   git tag -d v0.2.0
   git push origin --delete v0.2.0
   ```

## Configuration

Release settings are managed in:

- **`.github/workflows/release.yml`**: Workflow definition
- **`.github/workflows/release_configurations.json`**: Channel configuration
- **`rust-toolchain.toml`**: Rust version for builds

## CI Integration

The release workflow runs _after_ normal CI (tests, linting):

1. Push code to `master`
2. CI runs automatically (see `.github/workflows/ci.yml`)
3. Once CI passes, manually trigger release workflow

## Support

For questions or issues with the release process:

1. Check GitHub Issues: https://github.com/tzero86/Zterm/issues
2. Review the workflow logs in GitHub Actions
3. See `DISABLED_WARP_WORKFLOWS.md` for context on why custom workflows were needed

---

**Last Updated**: 2026-05-01

**Maintained By**: Zterm Team
