# flk 🚀

> A modern CLI tool for managing Nix flake development environments with the simplicity of Devbox

[![Crates.io](https://img.shields.io/crates/v/flk.svg)](https://crates.io/crates/flk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.83%2B-blue.svg)](https://www.rust-lang.org)
[![codecov](https://codecov.io/gh/AEduardo-dev/flk/graph/badge.svg)](https://codecov.io/gh/AEduardo-dev/flk)

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

### Hooks

If you would like to use hot reloading and switching features, you will need to add the following shell hook to your shell configuration file (`~/.bashrc`, `~/.zshrc`, etc.):

Example for **bash**:

```sh
# flk shell hook
if command -v flk &> /dev/null; then
  eval "$(flk hook bash)"
fi
```

Support for other shells (zsh and fish) is also available via `flk hook <shell>`.

### From Source

```bash
git clone https://github.com/AEduardo-dev/flk.git
cd flk
cargo build --release
sudo cp target/release/flk /usr/local/bin/
```

### From Cargo (crates.io)

```bash
cargo install flk
```

### From Release Binaries

1. Go to https://github.com/AEduardo-dev/flk/releases
2. Download the archive for your OS/arch (Linux x86_64, macOS Intel, macOS ARM).
3. Unpack and place `flk` in your PATH.

### Nix (with Cachix binaries)

This flake is prebuilt and published to Cachix.

1. Install Cachix (once):

```sh
nix profile install nixpkgs#cachix
```

2. Trust the cache:

```sh
cachix use flk-cache
```

or add the following substituters and trusted-public-keys to your `nix.conf` content:

```
substituters = https://flk-cache.cachix.org  ...
trusted-public-keys = flk-cache.cachix.org-1:6xobbpP9iIK5sIH/75DQrsJYKN/61nTOChcH9MJnBR0=  ...
```

3. Use the flake:

- Run (no install): `nix run github:AEduardo-dev/flk#flk`
- Install to your profile: `nix profile install github:AEduardo-dev/flk#flk`

### Nix – Using as a flake input

You can consume `flk` from another flake either directly or via the overlay.

**Direct (no overlay):**

```nix
{
  description = "My NixOS config with flk";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flk.url = "github:AEduardo-dev/flk";
  };

  outputs = { self, nixpkgs, flk, ... }:
    let
      system = "x86_64-linux"; # set per host
      pkgs = import nixpkgs { inherit system; };
    in {
      nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
        inherit system;
        modules = [
          {
            environment.systemPackages = [
              flk.packages.${system}.flk
            ];
          }
        ];
      };
    };
}
```

**With overlay (exposes `pkgs.flk`):**

```nix
{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.flk.url = "github:AEduardo-dev/flk";

  outputs = { self, nixpkgs, flk, ... }:
    let
      system = "x86_64-linux"; # set per host

      pkgs = import nixpkgs {
        inherit system;
        overlays = [ flk.overlay ];
      };
    in {
      nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
        inherit system;
        modules = [
          { environment.systemPackages = [ pkgs.flk ]; }
        ];
      };
    };
}
```

**Home Manager example (per-user install via flake):**

```nix
{
  inputs.flk.url = "github:AEduardo-dev/flk";

  outputs = { self, flk, ... }: {
    homeConfigurations.myhost = {
      # ...
      home.packages = [ flk.packages.${system}.flk ];
    };
  };
}
```

### Architectures covered by the cache

- x86_64-linux
- x86_64-darwin
- aarch64-darwin
- aarch64-linux (built via qemu on CI; may be slower/occasional misses)

Other architectures will fall back to building from source.

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

**Options:**

- `-p, --profile <NAME>` - Activate a specific profile (defaults to current active profile)

#### `flk show`

Display the contents and configuration of your `flake.nix` in a human-readable format.

```bash
flk show
```

#### `flk list`

List all packages in your development environment.

```bash
flk list
flk list --profile rust
flk list -p rust
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

**Examples:**

````bash
flk deep-search ripgrep
flk deep-search python311
```

#### `flk add <PACKAGE>`

Add a package to your `flake.nix`.

**Examples:**

```bash
flk add ripgrep
flk add git
flk add nodejs
flk add ripgrep --profile rust
flk add ripgrep -p rust
````

Or add a specific version:

```bash
flk add ripgrep --version '15.1.0'
flk add git --version '2.42.0'
flk add git --version '2.42.0' --profile rust
flk add git --version '2.42.0' -p rust
```

#### `flk remove <PACKAGE>`

Remove a package from your `flake.nix`.

**Examples:**

```bash
flk remove ripgrep
flk remove ripgrep --profile rust
flk remove ripgrep -p rust
```

### Custom Commands

#### `flk command add <NAME> <COMMAND> [OPTIONS]`

Add a custom shell command to your development environment.

**Examples:**

```bash
# Inline command
flk command add test "cargo test --all"
flk command add dev "npm run dev -- --watch"
flk command --profile rust add test "cargo test --all"
flk command -p rust add test "cargo test --all"

# Multi-line command
flk command add deploy "cargo build --release && scp target/release/app server:/opt/"

```

**Command naming rules:**

- Must contain only letters, numbers, hyphens, and underscores
- Cannot start with a hyphen
- Examples: `test`, `dev-server`, `build_prod`

#### `flk command remove <NAME>`

Remove a custom command from your dev shell.

**Examples:**

```bash
flk command remove test
flk command --profile rust remove test
flk command -p rust remove test
```

### Environment Variables

#### `flk env add <NAME> <VALUE>`

Add an environment variable to your dev shell.

**Examples:**

```bash
flk env add DATABASE_URL "postgresql://localhost:5432/mydb"
flk env add NODE_ENV "development"
flk env add API_KEY "sk-..."
flk env --profile rust add DATABASE_URL "postgresql://localhost:5432/mydb"
flk env -p rust add DATABASE_URL "postgresql://localhost:5432/mydb"
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
flk env --profile rust remove DATABASE_URL
flk env -p rust remove DATABASE_URL
```

#### `flk env list`

List all environment variables in your dev shell.

```bash
flk env list
flk env --profile rust list
flk env -p rust list
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
flk export --format json --profile rust
flk export --format json -p rust
```

### Profiles

#### `flk profile add <NAME> [OPTIONS]`

Create a new profile.

**Options:**

- `-t, --template <TYPE>` - Template to use (`base`, `rust`, `python`, `node`, `go`, `generic`)
- `-f, --force` - Overwrite profile if it exists

**Examples:**

```bash
flk profile add rust --template rust
flk profile add api --template base
```

#### `flk profile remove <NAME>`

Remove an existing profile.

```bash
flk profile remove rust
```

#### `flk profile list`

List available profiles.

```bash
flk profile list
```

#### `flk profile set-default <NAME>`

Set the default profile used when no `--profile` is supplied.

```bash
flk profile set-default rust
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

flk command add notebook "jupyter notebook --port=8888"
flk env add JUPYTER_CONFIG_DIR "./.jupyter"

flk activate
notebook  # Your custom command is ready!
```

### Rust Web Development

```bash
flk init --template rust
flk add postgresql
flk add redis

flk command add dev "cargo watch -x run"
flk command add migrate "sqlx migrate run"
flk env add DATABASE_URL "postgresql://localhost/myapp"

flk activate
dev      # Start development server with auto-reload
migrate  # Run database migrations
```

### Node.js Full-Stack Project

```bash
flk init --template node
flk add postgresql
flk add docker-compose

flk command add dev "npm run dev"
flk command add db "docker-compose up -d postgres"
flk env add NODE_ENV "development"

flk activate
db   # Start database
dev  # Start development server
```

### Go Microservice

```bash
flk init --template go
flk add protobuf
flk add grpcurl

flk command add build "go build -o bin/service ./cmd/service"
flk command add proto "protoc --go_out=. --go-grpc_out=. api/*.proto"
flk env add GO_ENV "development"

flk activate
proto  # Generate protobuf code
build  # Build the service
```

## ⚡ Speeding Up Shell Activation

By default, `nix develop` re-evaluates and may rebuild your dev shell on every invocation. flk includes built-in optimizations, plus you can add more for near-instant activation.

### GC Root Pinning (Built-in)

`flk activate` automatically creates a GC root at `.flk/.nix-profile-<profile>` via `nix develop --profile`. This prevents Nix from garbage-collecting your shell closure, so after a system restart, re-activation is instant (no rebuild needed).

> **Tip:** Avoid `nix-collect-garbage -d` which removes all GC roots. Prefer `nix-collect-garbage --delete-older-than 30d` to keep recent environments cached.

### nix-direnv (Recommended)

For the best experience, install [nix-direnv](https://github.com/nix-community/nix-direnv). It caches devShell evaluations and only re-evaluates when your flake inputs or `.flk/` config files actually change.

1. Install nix-direnv following [their instructions](https://github.com/nix-community/nix-direnv#installation)
2. Run `flk direnv init` — flk automatically generates `watch_file` directives for your `.flk/` config files
3. Run `direnv allow`

With nix-direnv, entering your project directory activates the cached environment in milliseconds.

### Cachix Binary Cache

flk publishes prebuilt binaries to [Cachix](https://app.cachix.org/cache/flk-cache). To skip building flk itself from source:

```bash
cachix use flk-cache
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

 .
├──  Cargo.lock
├──  Cargo.toml
├──  CHANGELOG.md
├──  cliff.toml
├──  CODE_OF_CONDUCT.md
├──  CONTRIBUTING.md
├──  dist-workspace.toml
├──  flake.lock
├──  flake.nix
├──  LICENSE
├── 󰂺 README.md
├──  release-plz.toml
├── 󰣞 src
│   ├──  commands
│   │   ├──  activate.rs
│   │   ├──  add.rs
│   │   ├──  command.rs
│   │   ├──  completions.rs
│   │   ├──  direnv.rs
│   │   ├──  env.rs
│   │   ├──  export.rs
│   │   ├──  init.rs
│   │   ├──  list.rs
│   │   ├──  lock.rs
│   │   ├──  mod.rs
│   │   ├──  remove.rs
│   │   ├──  search.rs
│   │   ├──  show.rs
│   │   └──  update.rs
│   ├──  flake
│   │   ├──  generator.rs
│   │   ├──  interfaces
│   │   │   ├──  mod.rs
│   │   │   ├──  overlays.rs
│   │   │   ├──  profiles.rs
│   │   │   ├──  shellhooks.rs
│   │   │   └──  utils.rs
│   │   ├──  mod.rs
│   │   ├──  nix_render.rs
│   │   └──  parsers
│   │       ├──  commands.rs
│   │       ├──  env.rs
│   │       ├──  flake.rs
│   │       ├──  mod.rs
│   │       ├──  overlays.rs
│   │       ├──  packages.rs
│   │       └──  utils.rs
│   ├──  lib.rs
│   ├──  main.rs
│   ├──  nix
│   │   └──  mod.rs
│   └──  utils
│       ├──  backup.rs
│       ├──  mod.rs
│       └──  visual.rs
├──  templates
│   ├──  default.nix
│   ├──  flake.nix
│   ├──  overlays.nix
│   ├──  pins.nix
│   └──  profiles
│       ├──  base.nix
│       ├──  default.nix
│       ├──  go.nix
│       ├──  node.nix
│       ├──  python.nix
│       └──  rust.nix
├──  tests
│   ├──  flake_tests.nix
│   ├──  integration_tests.rs
│   ├──  pins_tests.nix
│   ├──  profile_tests.nix
│   └──  unit_tests.rs
└──  wix
    └──  main.wxs
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

**Note:** This project is under active development. While all core features are implemented and working, some advanced features are still in progress and will be subject to change.
