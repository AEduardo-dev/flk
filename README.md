# flk 🚀

> A modern CLI tool for managing Nix flake development environments with the simplicity of Devbox

[![Crates.io](https://img.shields.io/crates/v/flk.svg)](https://crates.io/crates/flk)
[![CI](https://github.com/AEduardo-dev/flk/actions/workflows/ci.yml/badge.svg)](https://github.com/AEduardo-dev/flk/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.83%2B-blue.svg)](https://www.rust-lang.org)
[![codecov](https://codecov.io/gh/AEduardo-dev/flk/graph/badge.svg)](https://codecov.io/gh/AEduardo-dev/flk)

**flk** makes managing Nix flakes feel like using a package manager. No more manually editing `flake.nix` files — just use simple commands to add packages, create custom shell commands, and manage your development environment.

## ✨ Features

- 🎯 **Smart Initialization** — Auto-detects your project type (Rust, Python, Node.js, Go)
- 📦 **Easy Package Management** — Add/remove packages, pin specific versions
- ⚡ **Custom Shell Commands** — Define reusable commands for your workflow
- 🌍 **Environment Variables** — Manage per-project variables through the CLI
- 👤 **Multi-Profile Support** — Maintain separate configurations within one project
- 🔒 **Lock File Management** — Backup, preview, and restore your `flake.lock`
- 🐳 **Container Export** — Export environments to Docker, Podman, or JSON
- 🔄 **Hot Reload** — Shell hooks for instant `refresh` and `switch` between profiles

## ⚡ Quick Start

```bash
cargo install flk          # install via cargo (or see table below)

flk init                   # scaffold a flake (auto-detects language)
flk add ripgrep            # add a package
flk cmd add test "cargo test --all"
flk env add DB_URL "postgres://localhost/dev"
flk activate               # enter the dev shell
```

See the [Getting Started guide](https://aeduardo-dev.github.io/flk/getting-started.html) for a full walkthrough.

## 📦 Installation

**Prerequisites:** [Nix](https://nixos.org/download.html) with flakes enabled — recommended via [Lix](https://lix.systems/install/) or the [Determinate installer](https://determinate.systems/nix-installer/).

| Method | Command |
|--------|---------|
| **Cargo** | `cargo install flk` |
| **Nix (run)** | `nix run github:AEduardo-dev/flk#flk` |
| **Nix (install)** | `nix profile install github:AEduardo-dev/flk#flk` |
| **From source** | `git clone … && cd flk && cargo install --path .` |
| **Binaries** | [GitHub Releases](https://github.com/AEduardo-dev/flk/releases) |
| **Cachix** | `cachix use flk-cache` (prebuilt Nix binaries) |

For Nix flake input, overlay, and Home Manager options see the [Installation guide](https://aeduardo-dev.github.io/flk/installation.html).

### Shell Hook (optional)

Enable hot-reload (`refresh`) and profile switching (`switch`):

```sh
eval "$(flk hook bash)"   # or zsh; Fish: flk hook fish | source
```

## 📖 Commands

| Command | Description |
|---------|-------------|
| `flk init` | Initialize a new flake environment |
| `flk add` / `remove` | Add or remove packages (`--version` to pin) |
| `flk search` / `deep-search` | Search nixpkgs |
| `flk list` / `show` | List packages or pretty-print full config |
| `flk cmd add\|remove\|list` | Manage custom shell commands |
| `flk env add\|remove\|list` | Manage environment variables |
| `flk profile add\|remove\|list\|set-default` | Manage profiles |
| `flk activate` | Enter the dev shell |
| `flk update` | Update flake inputs (auto-backup) |
| `flk lock show\|history\|restore` | Manage flake.lock snapshots |
| `flk export` | Export to Docker, Podman, or JSON |
| `flk direnv init\|attach\|detach` | Direnv integration |
| `flk hook <shell>` | Generate shell hooks |
| `flk completions` | Generate shell completions |

Most commands accept `-p, --profile <NAME>` to target a specific profile.

Full command reference → [Commands documentation](https://aeduardo-dev.github.io/flk/commands/overview.html)

## 🛠️ Development

```bash
git clone https://github.com/AEduardo-dev/flk.git && cd flk
nix develop                # or ensure Rust 1.83+ is available

cargo build                # debug build
cargo test                 # all tests
cargo fmt --all -- --check && cargo clippy -- -D warnings  # lint
```

See the [Development guide](https://aeduardo-dev.github.io/flk/development.html) for project structure, testing details, and CI info.

## 📚 Documentation

| Resource | Link |
|----------|------|
| **User Guide** (mdBook) | [aeduardo-dev.github.io/flk](https://aeduardo-dev.github.io/flk/) |
| **API Reference** | [docs.rs/flk](https://docs.rs/flk) |

**Build locally:**

```bash
# User guide — serve with live reload
cd flk-book && mdbook serve        # http://localhost:3000

# Rust API docs
cargo doc --no-deps --open

# Both via Nix
nix build .#docs
```

## 🗺️ Roadmap

[Roadmap](https://github.com/AEduardo-dev/flk/issues/6)

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for details.
For bugs, please open an [issue](https://github.com/AEduardo-dev/flk/issues) with reproduction steps and environment info.

## 🔗 Related Projects

- [Devbox](https://github.com/jetify-com/devbox) — Instant, portable dev environments (inspiration for flk)
- [devenv](https://devenv.sh/) — Fast, declarative developer environments
- [Flox](https://flox.dev/) — Developer environments you can take with you
- [direnv](https://direnv.net/) — Shell extension for loading environments

## 📄 License

MIT — see [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- The Nix community for creating an amazing ecosystem
- Jetify for the Devbox inspiration
- Special mention to @vic for [nix-versions](https://github.com/vic/nix-versions)
- All contributors and users of flk

---

**Made with ❤️ by [AEduardo-dev](https://github.com/AEduardo-dev)**
