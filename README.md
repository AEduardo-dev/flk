# flk 🚀

> A CLI tool for managing `flake.nix` files as if they were Jetify Devbox environments

[![Crates.io](https://img.shields.io/crates/v/flk.svg)](https://crates.io/crates/flk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`flk` simplifies the management of Nix flakes for development environments, providing a user-friendly CLI similar to Devbox but for native `flake.nix` files.

## ✨ Features

- 🎯 **Easy Initialization**: Create project-specific flake templates with `flk init`
- 🔍 **Package Search**: Search nixpkgs directly from the CLI
- 📦 **Simple Package Management**: Add packages with optional version pinning
- ⚡ **Custom Commands**: Add shell functions and scripts to your dev environment
- 🎨 **Language Templates**: Pre-configured templates for Rust, Python, Node.js, Go, and more

## 📦 Installation

### From Source (Current)

```bash
git clone https://github.com/AEduardo-dev/flk.git
cd flk
cargo build --release
sudo cp target/release/flk /usr/local/bin/
```

### From Cargo (Coming Soon)

```bash
cargo install flk
```

### With Nix

```bash
nix profile install github:AEduardo-dev/flk
```

## 🚀 Quick Start

### 1. Initialize a new flake

```bash
# Auto-detect project type
flk init

# Or specify a template
flk init --template rust
flk init --template python
flk init --template node
```

### 2. Search for packages

```bash
# Search nixpkgs
flk search ripgrep

# Get detailed package info
flk deep-search ripgrep --versions
```

### 3. Add packages to your environment

```bash
# Add a package
flk add ripgrep

# Add with version pinning
flk add python311 --version 3.11.6
```

### 4. Add custom commands

```bash
# Add a simple command
flk add-command test "cargo test --all"

# Source commands from a file
flk add-command scripts --file ./scripts/dev.sh
```

### 5. Enter your dev environment

```bash
nix develop
```

## 📖 Commands

### `flk init`

Initialize a new `flake.nix` in the current directory.

```bash
flk init [OPTIONS]

Options:
  -t, --template <TYPE>  Project type (rust, python, node, go, generic)
  -f, --force           Overwrite existing flake.nix
```

**Auto-detection:**

- Detects `Cargo.toml` → Rust template
- Detects `package.json` → Node.js template
- Detects `pyproject.toml` or `requirements.txt` → Python template
- Detects `go.mod` → Go template

### `flk search`

Search for packages in nixpkgs.

```bash
flk search <QUERY> [OPTIONS]

Options:
  -l, --limit <NUMBER>  Limit results (default: 10)
```

### `flk deep-search`

Get detailed information about a specific package.

```bash
flk deep-search <PACKAGE> [OPTIONS]

Options:
  -v, --versions  Show version history for pinning
```

### `flk add`

Add a package to your `flake.nix`.

```bash
flk add <PACKAGE> [OPTIONS]

Options:
  -v, --version <VERSION>  Pin to specific version
```

### `flk add-command`

Add a custom command to your dev shell.

```bash
flk add-command <NAME> <COMMAND> [OPTIONS]

Options:
  -f, --file <PATH>  Source commands from a file
```

### `flk update`

Update all packages in your `flake.nix` to their latest versions.

```bash
flk update [OPTIONS]

Options:
  --show   Preview updates without applying them
```

### `flk show`

Display the contents and configuration of your current `flake.nix`.

```bash
flk show
```

### `flk list`

List all packages currently included in your `flake.nix` environment.

```bash
flk list
```

### `flk remove`

Remove a package from your `flake.nix` environment.

```bash
flk remove <PACKAGE>
```

### `flk remove-command`

Remove a custom command from your dev shell configuration.

```bash
flk remove-command <NAME>
```

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

## 🏗️ Project Structure

```
flk/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── commands/         # Command implementations
│   │   ├── init.rs
│   │   ├── add.rs
│   │   ├── search.rs
│   │   └── add_command.rs
│   ├── flake/            # Flake parsing and generation
│   │   ├── parser.rs
│   │   └── generator.rs
│   └── nix/              # Nix command wrappers
│       └── mod.rs
└── templates/            # Flake templates
    ├── default_flake.nix
    ├── rust_flake.nix
    ├── python_flake.nix
    ├── node_flake.nix
    └── go_flake.nix
```

## 🤝 Contributing

Contributions are welcome! Please check out our [issues](https://github.com/AEduardo-dev/flk/issues) to see what needs work.

### Development Setup

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

**Note:** This project is in early development (v0.1.0). Some features are still being implemented. See the [roadmap](#-roadmap) for current status.

Made with ❤️ by [AEduardo-dev](https://github.com/AEduardo-dev)
