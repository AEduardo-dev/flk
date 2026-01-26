# 🚀 Getting Started

## 1. Initialize Your Project

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

## 2. Add Packages

```bash
# Search for packages
flk search ripgrep

# Get detailed package info and versions
flk deep-search ripgrep

# Add packages to your environment
flk add ripgrep
flk add git
flk add neovim

# Or add pinned versions
flk add ripgrep --version '15.1.0'
flk add git --version '2.42.0'
```

## 3. Add Custom Commands

```bash
# Add inline commands
flk command add test "cargo test --all"
flk command add dev "npm run dev"
```

## 4. Manage Environment Variables

```bash
# Add environment variables
flk env add DATABASE_URL "postgresql://localhost/mydb"
flk env add API_KEY "your-api-key"

# List all environment variables
flk env list

# Remove an environment variable
flk env remove API_KEY
```

## 5. Enter Your Development Environment

```bash
flk activate
```

Your custom commands and environment variables will be automatically available!

## 6. Generate completions

```bash
# Generates the completion file and prints it
flk completions

# Install the generated completions to the detected shell
flk completions --install
```

Follow the instructions after the command to make the completions available for you.

## 7. Attach to your direnv (optional)

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

## 8. Switch / Refresh your environment

Add the following to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.) to enable switching and refreshing of your flk environment when you navigate between project directories:
e.g. Bash

```bash
eval "$(flk hook bash)"
```

Currently supported shells are: bash, zsh and fish.

## Next Steps

[Core Concepts](./concepts.md)
[Commands](./commands/init.md)
