# Contributing to Zterm

Thank you for your interest in contributing to Zterm! This document explains how to file issues, open pull requests, and generally participate in the project.

---

## TL;DR

- Search for existing issues before filing a new one.
- For bugs, use the bug report template; for features, use the feature request template.
- Build with `cargo run --bin zterm-oss --features gui`.
- Format with `cargo fmt` and lint with `cargo clippy` before opening a PR.
- Report security vulnerabilities privately — do **not** open a public issue.

---

## How to Contribute

There are many ways to contribute to Zterm:

- **Bug reports** — Help us identify problems by filing clear, reproducible bug reports.
- **Feature requests** — Propose new ideas or improvements.
- **Code contributions** — Fix bugs, implement features, or improve tests.
- **Documentation** — Improve or extend the docs, README, or inline code comments.
- **Triage** — Help label and reproduce existing issues.

All contributors are expected to follow our [Code of Conduct](CODE_OF_CONDUCT.md).

---

## Filing Issues

Before filing a new issue, please [search existing issues](../../issues) to check whether your bug or feature request has already been reported. If it has, add a reaction or comment rather than opening a duplicate.

### Bug Reports

When filing a bug, please include:

- A clear, descriptive title.
- Steps to reproduce the problem.
- Expected behaviour and actual behaviour.
- Your platform (macOS version, Linux distro, etc.) and Rust toolchain version.
- Any relevant logs or screenshots.

### Feature Requests

When requesting a feature, please include:

- A clear description of the problem you are trying to solve.
- Your proposed solution, if you have one in mind.
- Any alternatives you have considered.

---

## Opening a PR

1. **Fork** this repository and create a new branch from `main`:

   ```sh
   git checkout -b my-feature-branch
   ```

2. **Bootstrap** your development environment:

   ```sh
   ./script/bootstrap
   ```

   See `ZTERM.md` for the full engineering guide.

3. **Make your changes.** Keep commits focused and atomic — one logical change per commit is ideal.

4. **Build and run** to verify your changes work end-to-end:

   ```sh
   cargo run --bin zterm-oss --features gui
   ```

5. **Run the presubmit checks** before pushing:

   ```sh
   ./script/presubmit
   ```

6. **Push** your branch and open a pull request against `main`. Fill in the PR template, linking to any related issue.

7. A maintainer will review your PR. Be prepared to make revisions based on feedback.

---

## Testing

- Unit and integration tests live alongside the code they test.
- Run the full test suite with:

  ```sh
  cargo test --workspace
  ```

- When adding new functionality, please add corresponding tests.
- When fixing a bug, try to add a regression test that would have caught it.

---

## Code Style

Zterm uses the standard Rust formatting and lint toolchain:

- **Format** your code before committing:

  ```sh
  cargo fmt
  ```

- **Lint** your code and fix any warnings:

  ```sh
  cargo clippy --workspace --all-targets --all-features --tests -- -D warnings
  ```

  PRs that introduce new Clippy warnings will not be merged.

- Follow idiomatic Rust conventions. Prefer clarity over cleverness.
- Keep public API surfaces documented with `///` doc comments.

---

## Commit Conventions

- Write commit messages in the imperative mood: *"Fix crash on startup"*, not *"Fixed crash"* or *"Fixes crash"*.
- Keep the subject line under 72 characters.
- If the change warrants more explanation, add a blank line after the subject and then a body paragraph.
- Reference relevant issues in the footer, e.g. `Closes #123`.

---

## Code of Conduct

This project follows the [Contributor Covenant v2.1](CODE_OF_CONDUCT.md). By participating you agree to uphold a welcoming and respectful environment for all contributors. Violations can be reported by opening a GitHub issue or discussion on this repository.

---

## Reporting Security Issues

**Please do not open a public GitHub issue for security vulnerabilities.**

Instead, report security issues by opening a [GitHub Security Advisory](../../security/advisories/new) on this repository. This allows us to assess and address the vulnerability privately before public disclosure.

See [SECURITY.md](SECURITY.md) for the full security policy.

---

## Getting Help

If you have a question about the codebase, the build system, or how to approach a contribution, please [open a GitHub issue](../../issues/new) with the `question` label. The maintainers and community members monitor issues and will do their best to help.
