# flk

[![crates.io](https://img.shields.io/crates/v/flk.svg)](https://crates.io/crates/flk)  
[![docs.rs](https://docs.rs/flk/badge.svg)](https://docs.rs/flk)  
[![build](https://github.com/AEduardo-dev/flk/actions/workflows/ci.yml/badge.svg)](https://github.com/AEduardo-dev/flk/actions)  
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Flk** — Manage your [Nix flakes](https://nixos.org/manual/nix/stable/command-ref/new-cli/nix3-flake/)-based development environments with a friendly CLI.

> ⚠️ **Note:** Flk is under active development. Expect rapid changes and new features in upcoming releases.

---

## ✨ Why Flk?

When using Nix flakes for dev environments, you often write `flake.nix` templates from scratch or copy boilerplate.  
**Flk** makes it easy to initialise, search, add packages, manage commands, and configure environments — all from a single tool.

---

## Prerequisites

- [Nix](https://nixos.org/download.html) with flakes enabled
- Basic familiarity with Nix and flakes
- Rust toolchain (for building from source)

---

## ✨ Features

- 🎯 **Easy Initialization**: Create project-specific flake templates with `flk init`
- 🔍 **Package Search**: Search nixpkgs directly from the CLI
- 📦 **Simple Package Management**: Add packages with optional version pinning (Future feature)
- ⚡ **Custom Commands**: Add shell functions and scripts to your dev environment
- 🎨 **Language Templates**: Pre-configured templates for Rust, Python, Node.js, Go, and more

---

## 🧩 Installation

### From crates.io

```bash
cargo install flk
```

### From source

```bash
git clone https://github.com/AEduardo-dev/flk.git
cd flk
cargo build --release
sudo cp target/release/flk /usr/local/bin/
```

### Using Nix

```bash
nix profile install github:AEduardo-dev/flk
```

---

## ⚡ Getting Started

### 1. Initialise a new flake

```bash
# Auto-detect project type
flk init

# Or specify a template
flk init --template rust
flk init --template python
```

### 2. Search for packages

```bash
flk search ripgrep
flk deep-search ripgrep --versions
```

### 3. Add packages

```bash
flk add ripgrep
flk add python311 --version 3.11.6
```

### 4. Add custom commands

```bash
flk add-command test "cargo test --all"
flk add-command scripts --file ./scripts/dev.sh
```

### 5. Enter the development environment

```bash
nix develop
```

---

## 💡 Examples

### Python — Data Science Environment

```bash
flk init --template python
flk add python311Packages.numpy
flk add python311Packages.pandas
flk add python311Packages.matplotlib
flk add jupyter
flk add-command notebook "jupyter notebook --port=8888"
```

### Rust — Web Development

```bash
flk init --template rust
flk add postgresql
flk add redis
flk add-command dev "cargo watch -x run"
flk add-command migrate "sqlx migrate run"
```

---

## 🛣️ Roadmap

- [x] Project scaffolding and CLI structure (#1)
- [x] Implement `init` command (#2)
- [x] Implement `search` and `deep-search` commands (#3)
- [x] Implement `add` and `add-command` commands (#4)
- [x] Documentation and examples (#5)
- [x] Implement `remove` and `remove-command` commands (#6)
- [x] Implement `update`, `show`, and `list` commands (#7)
- [ ] CI/CD and releases
- [ ] Package registry integration
- [ ] Interactive TUI mode
- [ ] Flake templates marketplace

---

## 🤝 Contributing

Contributions are **very welcome**!

To get started:

```bash
git clone https://github.com/AEduardo-dev/flk.git
cd flk
cargo build
cargo test
```

Please read `CONTRIBUTING.md` (if present) for guidelines on coding style, testing, and pull requests.  
You can also browse [open issues](https://github.com/AEduardo-dev/flk/issues) for ideas or to help others.

### 🛠️ Development Setup

```bash
# Clone the repo
git clone https://github.com/AEduardo-dev/flk.git
cd flk

# Build
cargo build

# Run tests
cargo test

# Install locally
cargo install --path .
```

---

## 📝 Examples

### Python Data Science Environment

```bash
flk init --template python
flk add python311Packages.numpy
flk add python311Packages.pandas
flk add python311Packages.matplotlib
flk add jupyter
flk add-command notebook "jupyter notebook --port=8888"
```

### Rust Web Development

```bash
flk init --template rust
flk add postgresql
flk add redis
flk add-command dev "cargo watch -x run"
flk add-command migrate "sqlx migrate run"
```

---

## 🔗 Inspiration

- [Devbox](https://github.com/jetify-com/devbox) - Instant, portable dev environments
- [devenv](https://devenv.sh/) - Fast, declarative developer environments
- [Flox](https://flox.dev/) - Developer environments you can take with you

## 📄 License

MIT License - see [LICENSE](LICENSE) for details

## 🙏 Acknowledgments

- The Nix community for the amazing ecosystem
- Jetify for the Devbox inspiration
- All contributors and users of flk

---

Made with ❤️ by [AEduardo-dev](https://github.com/AEduardo-dev)
