# Zterm

**Zterm** is an agentic terminal and development environment built for modern engineering workflows. It is designed to be fast, intelligent, and extensible — giving you powerful completions, command history, and an AI-assisted shell experience directly in your terminal.

---

## About

Zterm is a Rust-based terminal emulator and development environment. It combines a high-performance GPU-rendered UI with shell-level intelligence, allowing you to work faster with features like:

- Intelligent autocompletion powered by completion specs
- Command output blocks with structured navigation
- AI-assisted command generation and explanation
- A themeable, GPU-accelerated interface
- First-class support for modern shells

Zterm is a Rust-based terminal emulator and development environment, originally forked from [Warp](https://github.com/warpdotdev/warp) by Denver Technologies, Inc. The project has evolved into an independent, community-driven platform with its own roadmap, architecture, and feature set focused on agentic workflows and AI-assisted development.

---

## Building from Source

### Prerequisites

Run the bootstrap script to install all required platform dependencies:

```sh
./script/bootstrap
```

This will set up your Rust toolchain, system libraries, and any other platform-specific requirements. Refer to `ZTERM.md` for the full engineering guide, including detailed environment setup instructions for macOS and Linux.

### Running Zterm

Once your environment is bootstrapped, build and run Zterm with:

```sh
cargo run --bin zterm-oss --features gui
```

### Linting and Tests

Before opening a pull request, run the presubmit script to ensure your changes pass all checks:

```sh
./script/presubmit
```

This runs `cargo fmt`, `cargo clippy`, and the test suite. See `ZTERM.md` for more details on the full test and lint configuration.

---

## Licensing

Zterm uses a dual-license model:

| Crate | License |
|---|---|
| `zterm_ui_core` | [MIT](LICENSE-MIT) |
| `zterm_ui` | [MIT](LICENSE-MIT) |
| Everything else | [AGPL v3](LICENSE-AGPL) |

The `zterm_ui_core` and `zterm_ui` crates are published under the MIT license so that the UI framework components can be reused freely in other projects. The remainder of Zterm — the terminal application itself — is licensed under the GNU Affero General Public License v3 (AGPL v3).

The AGPL was chosen deliberately: if you run a modified version of Zterm as a service (e.g., a hosted terminal), you must release the source of those modifications. This protects the community while still allowing free use, modification, and distribution.

For full license texts see the `LICENSE-MIT` and `LICENSE-AGPL` files at the root of this repository.

---

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening issues or pull requests. The short version:

1. Fork the repo and create a feature branch.
2. Follow the code style enforced by `cargo fmt` and `cargo clippy`.
3. Add tests for any new behaviour.
4. Open a pull request against `main` with a clear description of your changes.

See `ZTERM.md` for the complete engineering and architecture guide.

---

## Code of Conduct

This project follows the [Contributor Covenant v2.1](CODE_OF_CONDUCT.md). By participating you agree to uphold a welcoming, harassment-free environment for everyone. Violations can be reported by opening a GitHub issue or discussion on this repository.

---

## Credits

### Upstream Project

Zterm is a fork of **[Warp](https://github.com/warpdotdev/warp)**, the agentic terminal created by **Denver Technologies, Inc.** The original Warp project provided the foundation — including the GPU-rendered UI, block-based output model, and overall architecture — upon which Zterm is built. We are grateful to the Warp team and all contributors to the upstream repository.

Original repository: https://github.com/warpdotdev/warp

### Open-Source Dependencies

Zterm stands on the shoulders of a large number of excellent open-source projects. Key dependencies include:

| Project | Use |
|---|---|
| [Tokio](https://tokio.rs/) | Async runtime |
| [NuShell](https://www.nushell.sh/) | Shell and data pipeline infrastructure |
| [Fig Completion Specs](https://github.com/withfig/autocomplete) | Shell completion specifications |
| [Alacritty](https://github.com/alacritty/alacritty) | Terminal emulation primitives |
| [Hyper](https://hyper.rs/) | HTTP client/server |
| [FontKit](https://github.com/linebender/fontkit) | Font loading and rasterization |
| [Core-foundation](https://github.com/servo/core-foundation-rs) | macOS system API bindings |
| [Smol](https://github.com/smol-rs/smol) | Lightweight async executor |

A full list of dependencies and their licenses can be found in `Cargo.lock` and via `cargo license`.
