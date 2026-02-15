# Development Setup

This guide helps you set up a development environment for contributing to flk.

## Requirements

- **Nix** with flakes enabled (Lix or Determinate installer recommended)
- **Rust toolchain** (1.70+ recommended)
- **Git** for version control

## Quick Start with Nix

The easiest way to get started is using flk's own flake:

```bash
git clone https://github.com/AEduardo-dev/flk.git
cd flk
nix develop
```

This provides all required tools including Rust, cargo, and testing dependencies.

## Building from Source

```bash
# Clone the repository
git clone https://github.com/AEduardo-dev/flk.git
cd flk

# Debug build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release

# The binary is at target/release/flk
```

## Running Tests

```bash
# Run all tests
cargo test

# Run a specific test
cargo test test_add_package

# Run tests with output
cargo test -- --nocapture
```

## Linting and Formatting

```bash
# Check formatting
cargo fmt --all -- --check

# Apply formatting
cargo fmt --all

# Run clippy lints
cargo clippy -- -D warnings
```

## Running the CLI Locally

During development, use `cargo run`:

```bash
# Run flk commands
cargo run -- init --template rust
cargo run -- add ripgrep
cargo run -- list packages

# With release optimizations (faster execution)
cargo run --release -- search git
```

For shell integration while iterating:

```bash
# Build and install locally
cargo install --path .

# Or add an alias
alias flk="cargo run --release --"
```

## Building Documentation

### API Documentation (cargo doc)

```bash
# Generate and open API docs
cargo doc --no-deps --open

# Check for documentation warnings
cargo doc --no-deps 2>&1 | grep warning
```

### User Guide (mdbook)

```bash
# Install mdbook if needed
cargo install mdbook

# Serve the book locally (auto-reloads)
cd flk-book
mdbook serve
# Open http://localhost:3000

# Build static HTML
mdbook build
# Output in flk-book/book/
```

### Combined Documentation

The Nix flake can build both:

```bash
nix build .#docs
# Output in result/
```

## Project Structure

```
flk/
├── src/
│   ├── main.rs          # CLI entrypoint
│   ├── lib.rs           # Library exports
│   ├── commands/        # CLI command handlers
│   ├── flake/           # Flake generation and parsing
│   │   ├── generator.rs # Template instantiation
│   │   ├── parsers/     # Nix file parsers
│   │   └── interfaces/  # Data structures
│   ├── nix/             # Nix command wrappers
│   └── utils/           # Helpers (backup, visual)
├── templates/           # Nix templates (embedded at compile time)
├── tests/               # Integration tests
├── flk-book/            # mdbook documentation
└── flake.nix            # Nix flake for development
```

## Testing Changes

Before submitting a PR:

```bash
# Run the full test suite
cargo test

# Check formatting and lints
cargo fmt --all -- --check
cargo clippy -- -D warnings

# Build documentation without warnings
cargo doc --no-deps

# Test the binary manually
cargo run -- init --template rust
cargo run -- add ripgrep
cargo run -- list packages
```

## Debugging

For verbose output during development:

```bash
# Enable Rust backtraces
RUST_BACKTRACE=1 cargo run -- add ripgrep

# Debug Nix commands
RUST_LOG=debug cargo run -- search git
```
