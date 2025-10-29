# flk 🚀

> A modern CLI tool for managing Nix flake development environments with the simplicity of Devbox

[![Crates.io](https://img.shields.io/crates/v/flk.svg)](https://crates.io/crates/flk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

**flk** makes managing Nix flakes feel like using a package manager. No more manually editing `flake.nix` files—just use simple commands to add packages, create custom shell commands, and manage your development environment.

## ✨ Features

- 🎯 **Smart Initialization** - Auto-detects your project type (Rust, Python, Node.js, Go) and creates the right template
- 🔍 **Package Search** - Search nixpkgs directly from your terminal
- 📦 **Easy Package Management** - Add and remove packages with simple commands
- ⚡ **Custom Shell Commands** - Define reusable commands for your development workflow
- 🌍 **Environment Variables** - Manage environment variables through the CLI
- 🔒 **Lock File Management** - View, backup, and restore your flake.lock with ease
- 🎨 **Language Templates** - Pre-configured templates for popular languages and frameworks

## 📦 Installation

### Prerequisites

- [Nix](https://nixos.org/download.html) with flakes enabled
- Rust 1.70+ (if building from source)

### From Source

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

### 1. Initialize Your Project

```bash
# Auto-detect project type and create flake.nix
flk init

# Or specify a template
flk init --template rust
flk init --template python
flk init --template node
flk init --template go
```

**Supported auto-detection:**

- `Cargo.toml` → Rust template
- `package.json` → Node.js template
- `pyproject.toml` or `requirements.txt` → Python template
- `go.mod` → Go template

### 2. Add Packages

```bash
# Search for packages
flk search ripgrep

# Get detailed package info
flk deep-search ripgrep --versions

# Add packages to your environment
flk add ripgrep
flk add git
flk add neovim
```

### 3. Add Custom Commands

```bash
# Add inline commands
flk add-command test "cargo test --all"
flk add-command dev "npm run dev"

# Source commands from a file
flk add-command scripts --file ./scripts/dev.sh
```

### 4. Manage Environment Variables

```bash
# Add environment variables
flk env add DATABASE_URL "postgresql://localhost/mydb"
flk env add API_KEY "your-api-key"

# List all environment variables
flk env list

# Remove an environment variable
flk env remove API_KEY
```

### 5. Enter Your Development Environment

```bash
nix develop
```

Your custom commands and environment variables will be automatically available!

### 6. Generate completions

```bash
# Generates the completion file and prints it
flk completions

# Install the generated completions to the detected shell
flk completions --install
```

Follow the instructions after the command to make the completions available for you.

## 📖 Command Reference

### Project Management

#### `flk init [OPTIONS]`

Initialize a new `flake.nix` in the current directory.

**Options:**

- `-t, --template <TYPE>` - Project type: `rust`, `python`, `node`, `go`, or `generic`
- `-f, --force` - Overwrite existing `flake.nix`

**Examples:**

```bash
flk init                    # Auto-detect project type
flk init --template rust    # Use Rust template
flk init --force            # Overwrite existing flake.nix
```

#### `flk show`

Display the contents and configuration of your `flake.nix` in a human-readable format.

```bash
flk show
```

#### `flk list`

List all packages in your development environment.

```bash
flk list
```

### Package Management

#### `flk search <QUERY> [OPTIONS]`

Search for packages in nixpkgs.

**Options:**

- `-l, --limit <NUMBER>` - Limit number of results (default: 10)

**Examples:**

```bash
flk search ripgrep
flk search python --limit 20
```

#### `flk deep-search <PACKAGE> [OPTIONS]`

Get detailed information about a specific package.

**Options:**

- `-v, --versions` - Show version pinning information

**Examples:**

```bash
flk deep-search ripgrep
flk deep-search python311 --versions
```

#### `flk add <PACKAGE>`

Add a package to your `flake.nix`.

**Examples:**

```bash
flk add ripgrep
flk add git
flk add nodejs
```

**Note:** Version pinning is planned for a future release (see [issue #5](https://github.com/AEduardo-dev/flk/issues/5)).

#### `flk remove <PACKAGE>`

Remove a package from your `flake.nix`.

**Examples:**

```bash
flk remove ripgrep
```

### Custom Commands

#### `flk add-command <NAME> <COMMAND> [OPTIONS]`

Add a custom shell command to your development environment.

**Options:**

- `-f, --file <PATH>` - Source commands from a file

**Examples:**

```bash
# Inline command
flk add-command test "cargo test --all"
flk add-command dev "npm run dev -- --watch"

# Multi-line command
flk add-command deploy "cargo build --release && scp target/release/app server:/opt/"

# Source from file
flk add-command scripts --file ./dev-scripts.sh
```

**Command naming rules:**

- Must contain only letters, numbers, hyphens, and underscores
- Cannot start with a hyphen
- Examples: `test`, `dev-server`, `build_prod`

#### `flk remove-command <NAME>`

Remove a custom command from your dev shell.

**Examples:**

```bash
flk remove-command test
```

### Environment Variables

#### `flk env add <NAME> <VALUE>`

Add an environment variable to your dev shell.

**Examples:**

```bash
flk env add DATABASE_URL "postgresql://localhost:5432/mydb"
flk env add NODE_ENV "development"
flk env add API_KEY "sk-..."
```

**Variable naming rules:**

- Must start with a letter or underscore
- Can only contain letters, numbers, and underscores
- Examples: `MY_VAR`, `_private`, `API_KEY_2`

#### `flk env remove <NAME>`

Remove an environment variable from your dev shell.

**Examples:**

```bash
flk env remove DATABASE_URL
```

#### `flk env list`

List all environment variables in your dev shell.

```bash
flk env list
```

### Lock File Management

#### `flk lock show`

Display detailed information about your `flake.lock` file.

```bash
flk lock show
```

#### `flk lock history`

Show backup history of your lock file.

```bash
flk lock history
```

#### `flk lock restore <BACKUP>`

Restore a previous version of your lock file.

**Examples:**

```bash
flk lock restore latest                    # Restore most recent backup
flk lock restore 2025-01-27_14-30-00      # Restore specific backup
```

### Updates

#### `flk update [OPTIONS]`

Update all flake inputs to their latest versions.

**Options:**

- `--show` - Preview updates without applying them

**Examples:**

```bash
flk update              # Update all inputs
flk update --show       # Preview available updates
```

**Note:** A backup of your `flake.lock` is automatically created before updating.

## 💡 Usage Examples

### Python Data Science Environment

```bash
flk init --template python
flk add python311Packages.numpy
flk add python311Packages.pandas
flk add python311Packages.matplotlib
flk add jupyter

flk add-command notebook "jupyter notebook --port=8888"
flk env add JUPYTER_CONFIG_DIR "./.jupyter"

nix develop
notebook  # Your custom command is ready!
```

### Rust Web Development

```bash
flk init --template rust
flk add postgresql
flk add redis

flk add-command dev "cargo watch -x run"
flk add-command migrate "sqlx migrate run"
flk env add DATABASE_URL "postgresql://localhost/myapp"

nix develop
dev      # Start development server with auto-reload
migrate  # Run database migrations
```

### Node.js Full-Stack Project

```bash
flk init --template node
flk add postgresql
flk add docker-compose

flk add-command dev "npm run dev"
flk add-command db "docker-compose up -d postgres"
flk env add NODE_ENV "development"

nix develop
db   # Start database
dev  # Start development server
```

### Go Microservice

```bash
flk init --template go
flk add protobuf
flk add grpcurl

flk add-command build "go build -o bin/service ./cmd/service"
flk add-command proto "protoc --go_out=. --go-grpc_out=. api/*.proto"
flk env add GO_ENV "development"

nix develop
proto  # Generate protobuf code
build  # Build the service
```

## 🛠️ Development

### Prerequisites

- Rust 1.70+
- Nix with flakes enabled

### Building

```bash
git clone https://github.com/AEduardo-dev/flk.git
cd flk
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run with output
cargo test -- --nocapture
```

### Installing Locally

```bash
cargo install --path .
```

## 🏗️ Project Structure

```
flk/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── commands/            # Command implementations
│   │   ├── init.rs          # Initialize flake
│   │   ├── add.rs           # Add packages
│   │   ├── remove.rs        # Remove packages
│   │   ├── search.rs        # Search packages
│   │   ├── add_command.rs   # Add custom commands
│   │   ├── remove_command.rs
│   │   ├── env.rs           # Environment variable management
│   │   ├── lock.rs          # Lock file management
│   │   ├── update.rs        # Update flake inputs
│   │   ├── show.rs          # Display flake config
│   │   └── list.rs          # List packages
│   ├── flake/               # Flake parsing and generation
│   │   ├── parser.rs        # Parse flake.nix
│   │   ├── generator.rs     # Generate flake.nix
│   │   └── interface.rs     # Data structures
│   ├── nix/                 # Nix command wrappers
│   │   └── mod.rs
│   └── utils/               # Utility functions
│       └── backup.rs        # Backup management
├── templates/               # Flake templates
│   ├── default_flake.nix
│   ├── rust_flake.nix
│   ├── python_flake.nix
│   ├── node_flake.nix
│   └── go_flake.nix
└── tests/                   # Test files
    ├── integration_tests.rs
    └── unit_tests.rs
```

## 🗺️ Roadmap

[Roadmap](https://github.com/AEduardo-dev/flk/issues/6)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### How to Contribute

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 🐛 Bug Reports

If you find a bug, please open an issue with:

- A clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Your environment (OS, Nix version, etc.)

## 🔗 Related Projects

- [Devbox](https://github.com/jetify-com/devbox) - Instant, portable dev environments (inspiration for flk)
- [devenv](https://devenv.sh/) - Fast, declarative developer environments
- [Flox](https://flox.dev/) - Developer environments you can take with you
- [direnv](https://direnv.net/) - Shell extension for loading environments

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- The Nix community for creating an amazing ecosystem
- Jetify for the Devbox inspiration and showing what's possible
- All contributors and users of flk

## 📞 Support

- 📧 Open an [issue](https://github.com/AEduardo-dev/flk/issues) for bug reports or feature requests
- 💬 Start a [discussion](https://github.com/AEduardo-dev/flk/discussions) for questions or ideas
- ⭐ Star the repository if you find it useful!

---

**Made with ❤️ by [AEduardo-dev](https://github.com/AEduardo-dev)**

**Note:** This project is under active development (v0.1.0). While all core features are implemented and working, some advanced features like version pinning are still in progress. See the [roadmap](#-roadmap) for details.
