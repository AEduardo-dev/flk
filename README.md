# flk 🚀

> A modern CLI tool for managing Nix flake development environments with the simplicity of Devbox

[![Crates.io](https://img.shields.io/crates/v/flk.svg)](https://crates.io/crates/flk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.83%2B-blue.svg)](https://www.rust-lang.org)

**flk** makes managing Nix flakes feel like using a package manager. No more manually editing `flake.nix` files—just use simple commands to add packages, create custom shell commands, and manage your development environment with ease.

## ✨ Features

- 🎯 **Smart Initialization** - Auto-detects your project type (Rust, Python, Node.js, Go) and creates the right template
- 🔍 **Package Search** - Search nixpkgs directly from your terminal
- 📦 **Easy Package Management** - Add and remove packages with simple commands
- ⚡ **Custom Shell Commands** - Define reusable commands for your development workflow
- 🌍 **Environment Variables** - Manage environment variables through the CLI
- 🔒 **Lock File Management** - View, backup, and restore your flake.lock with ease
- 🎨 **Language Templates** - Pre-configured templates for popular languages and frameworks

## 📦 Installation

### Upgrading to v0.5.0 (switch/refresh changes)

**WARNING (pre v0.5.0 users):** If you are using `flk < v0.5.0` and you run `flk update` / `nix flake update`, your devshell `switch` / `refresh` behavior may break because the `nix-profile-lib` input may update to a newer version with different activation semantics.

If you intend to stay on `flk < v0.5.0`, use one of these options:

1. **Do not update flake inputs.** Avoid running `flk update` or `nix flake update`. If you already did, restore a previous lockfile backup with:

   ```bash
   flk lock restore <BACKUP>
   ```

2. **Pin `nix-profile-lib` to v0.1.0.** In your `flake.nix`:

   ```nix
   inputs = {
     nix-profile-lib.url = "github:AEduardo-dev/nix-profile-lib?ref=v0.1.0";
   };
   ```

   Then update the lock entry:

   ```bash
   nix flake lock --update-input nix-profile-lib
   ```

   (or `nix flake update --update-input nix-profile-lib`)

Once you upgrade to `flk v0.5.0+`, this restriction is lifted.

### Prerequisites

- [Nix](https://nixos.org/download.html) with flakes enabled
  - We recommend the Lix package manager for easy Nix installation: [Lix](https://lix.systems/install/) since it comes with flakes enabled by default.
  - Or using the Determinate System installer: [Determinate](https://determinate.systems/nix-installer/), as it provides a user-friendly way to install (and uninstall) Nix.
- Rust 1.83+ (if building from source)

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

### 7. Attach to your direnv (optional)

If you use `direnv`, you can set it up to automatically load your flk environment when you enter the project directory.

```bash
# Generates a .envrc file for direnv with use flake command
flk direnv init

#or

# Add the direnv hook to an existing project
flk direnv attach
```

if you ever want to detach the direnv hook, you can run:

```bash
flk direnv detach
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

#### `flk activate`

Activate the nix shell for the current shell session. This command sets up the necessary environment for your
project based on the `flake.nix` configuration. It also installs some convenience features, such as a shell hook to refresh.

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

### Exports

#### `flk export --format <FORMAT> [OPTIONS]`

Export the current flake configuration to different formats.
**Options:**

- `--format <FORMAT>` - Export format: `docker`, `podman`, `json`

**Examples:**

```bash
flk export --format docker     # Export as Dockerfile
flk export --format podman     # Export as Podmanfile
flk export --format json       # Export as JSON
```

### Direnv Integration

#### `flk direnv init`

Generate a `.envrc` file for direnv with `use flake` command.

```bash
flk direnv init
```

#### `flk direnv attach`

Add the direnv hook to an existing project.

```bash
flk direnv attach
```

#### `flk direnv detach`

Remove the direnv hook from the project.

```bash
flk direnv detach
```

## 💡 Usage Examples

### Python Data Science Environment

```bash
flk init --template python
flk add python312Packages.numpy
flk add python312Packages.pandas
flk add python312Packages.matplotlib
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

- Rust 1.83+
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

# Run unit tests only
cargo test --test unit_tests

# Run integration tests only
cargo test --test integration_tests

# Run with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

The test suite includes comprehensive unit tests for the parser, generator, and interface modules, as well as integration tests covering the complete CLI workflow including the dendritic `. flk/profiles/` architecture.

### Installing Locally

```bash
cargo install --path .
```

## 🏗️ Project Structure

```
flk/
├── src/
│   ├── main.rs               # CLI entry point
│   ├── commands/             # Command implementations
│   │   ├── activate.rs       # Activate dev shell
│   │   ├── add.rs            # Add packages
│   │   ├── add_command.rs    # Add custom commands
│   │   ├── completions.rs    # Shell completions
│   │   ├── env.rs            # Environment variable management
│   │   ├── export.rs         # Export flake config
│   │   ├── init.rs           # Initialize flake
│   │   ├── list.rs           # List packages
│   │   ├── lock.rs           # Lock file management
│   │   ├── mod.rs
│   │   ├── remove.rs         # Remove packages
│   │   ├── remove_command.rs # Remove custom commands
│   │   ├── search.rs         # Search packages
│   │   ├── show.rs           # Display flake config
│   │   └── update.rs         # Update flake inputs
│   ├── flake/                # Flake parsing and generation
│   │   ├── generator.rs      # Generate flake.nix
│   │   ├── interface.rs      # Data structures
│   │   ├── mod.rs
│   │   └── parsers/          # Parse flake.nix sections
│   │       ├── commands.rs   # Custom commands parser
│   │       ├── env.rs        # Environment variables parser
│   │       ├── flake.rs      # Flake parser
│   │       ├── overlays.rs   # Overlays parser
│   │       ├── packages.rs   # Packages parser
│   │       └── utils.rs      # Utility functions
│   ├── nix/                  # Nix command wrappers
│   │   └── mod.rs
│   └── utils/                # Utility functions
│       ├── backup.rs         # Backup management
│       ├── mod.rs
│       └── visual.rs         # Visual enhancements
├── templates/                # Flake templates
│   ├── flake.nix             # Root flake template
│   ├── default.nix           # Helper module
│   ├── overlays.nix          # Overlays configuration
│   ├── pins.nix              # Pin configuration
│   └── profiles/             # Profile templates
│       ├── base.nix          # Generic template
│       ├── default.nix       # Importer module
│       ├── rust.nix          # Rust template
│       ├── python.nix        # Python template
│       ├── node.nix          # Node.js template
│       └── go.nix            # Go template
└── tests/                    # Test files
    ├── integration_tests.rs  # CLI integration tests
    └── unit_tests.rs         # Module unit tests
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
- Special mention to @vic for creating [nix-versions](https://github.com/vic/nix-versions)

## 📞 Support

- 📧 Open an [issue](https://github.com/AEduardo-dev/flk/issues) for bug reports or feature requests
- 💬 Start a [discussion](https://github.com/AEduardo-dev/flk/discussions) for questions or ideas
- ⭐ Star the repository if you find it useful!

---

**Made with ❤️ by [AEduardo-dev](https://github.com/AEduardo-dev)**

**Note:** This project is under active development (v0.4.0). While all core features are implemented and working, some advanced features like version pinning are still in progress.
