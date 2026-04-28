# Frequently Asked Questions

---

## General

### What is Zterm?

Zterm is an agentic terminal and development environment built in Rust. It combines a high-performance GPU-rendered UI with shell intelligence features such as block-based command output, intelligent autocompletion, and AI-assisted command generation and explanation.

Zterm is a fork of [Warp](https://github.com/warpdotdev/warp) by Denver Technologies, Inc. It preserves the open-source client while removing proprietary cloud infrastructure and backend dependencies.

---

### Is Zterm free to use?

Yes. Zterm is fully open source. You can build it from source, modify it, and redistribute it subject to the license terms described below.

---

## Contributing

### How do I contribute to Zterm?

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full guide. The short version:

1. Search existing issues before filing a new one.
2. For bugs, open a bug report. For features, open a feature request.
3. Fork the repo, make your changes on a branch, and open a pull request against `main`.
4. Run `./script/presubmit` before pushing to ensure your changes pass formatting, linting, and tests.

### What kinds of contributions are welcome?

- Bug fixes
- Feature implementations
- Documentation improvements
- Test coverage improvements
- Performance improvements
- Accessibility improvements

---

## Building and Running

### How do I build Zterm from source?

First, bootstrap your development environment:

```sh
./script/bootstrap
```

Then build and run:

```sh
cargo run --bin zterm-oss --features gui
```

For the full engineering guide — including platform-specific setup, architecture notes, and debugging tips — see `ZTERM.md`.

### How do I run the tests?

```sh
cargo test --workspace
```

Or use `cargo nextest run` if you have `cargo-nextest` installed for faster parallel test execution.

### How do I lint and format the code?

```sh
cargo fmt
cargo clippy --workspace --all-targets --all-features --tests -- -D warnings
```

Both must pass cleanly before a pull request will be merged. You can also run `./script/presubmit` to execute formatting, linting, and tests in one step.

### What platforms are supported?

Zterm targets macOS and Linux. Windows support is not guaranteed. See `ZTERM.md` for platform-specific notes.

---

## Licensing

### What license is Zterm under?

Zterm uses a dual-license model:

| Crate | License |
|---|---|
| `zterm_ui_core` | MIT |
| `zterm_ui` | MIT |
| Everything else | AGPL v3 |

### Why AGPL for the application?

The GNU Affero General Public License v3 (AGPL v3) is a strong copyleft license. It means that if you distribute or run a modified version of Zterm — including over a network — you must make the source of your modifications available under the same license.

This choice protects the community: anyone who builds on Zterm must contribute their improvements back. It prevents proprietary forks from taking the codebase private without contributing back.

### Why MIT for the UI crates?

The `zterm_ui_core` and `zterm_ui` crates contain the UI framework components — layout primitives, rendering helpers, widget abstractions — that are useful independently of the terminal application itself. Publishing them under MIT allows other projects to use and build on these components freely, without being subject to the AGPL's network-service provisions.

The goal is to give back a broadly reusable UI toolkit to the Rust ecosystem while keeping the terminal application itself protected by the AGPL.

### Can I use Zterm in a commercial product?

You can use Zterm as a terminal in a commercial environment without restriction. However, if you distribute a **modified** version of Zterm or run a modified version as a hosted service, the AGPL requires you to make the source of your modifications available to users.

The MIT-licensed crates (`zterm_ui_core`, `zterm_ui`) can be used in any product, including closed-source commercial products, without restriction.

---

## Architecture

### What parts of the codebase are open source?

The entire Zterm client is open source and lives in this repository. This includes:

- The terminal emulator and PTY layer
- The GPU-rendered UI (built on the `zterm_ui` framework)
- Shell integration and completion logic
- AI feature integrations (prompt construction, response rendering)
- Build and release scripts

There is no proprietary server, cloud backend, or authentication layer included in this repository. Zterm is a fully local, self-contained application.

### Does Zterm have a cloud backend?

No. Unlike the upstream Warp project, Zterm does not include or require any cloud backend. All features run locally. There is no account system, no telemetry, and no hosted service.

### What shell does Zterm work with?

Zterm is designed to work with common shells (bash, zsh, fish) and integrates completion specs from the Fig Completion Specs project. See `ZTERM.md` for shell integration details.

---

## Security

### How do I report a security vulnerability?

Do **not** open a public issue. Instead, open a [GitHub Security Advisory](../../security/advisories/new) on this repository so the issue can be assessed and addressed privately before public disclosure. See [SECURITY.md](SECURITY.md) for the full security policy.

---

## Miscellaneous

### Where is the full engineering guide?

See `ZTERM.md` at the root of this repository. It covers architecture, coding conventions, testing, platform-specific notes, and more.

### Where can I get help?

Open a [GitHub issue](../../issues/new) with the `question` label. Maintainers and community members monitor issues and will do their best to help.
