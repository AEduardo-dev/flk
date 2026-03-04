# Custom Commands

Custom commands let you define reusable scripts in your flk environment. They become available as shell functions when the development environment is activated.

## Basic Usage

```bash
flk cmd add dev "npm run dev"
flk cmd add test "cargo test --all"
flk cmd add lint "cargo clippy -- -D warnings"
flk cmd list
flk cmd remove dev
```

## Loading from Files

For complex or multiline commands, use the `--file` flag:

```bash
flk cmd add deploy --file scripts/deploy.sh
```

This reads the command body from the specified file instead of inline text.

## Profile Targeting

Commands are stored per-profile. Use `--profile` to target a specific one:

```bash
flk cmd add build "cargo build --release" --profile backend
flk cmd list --profile frontend
```

## How It Works

Commands are stored in the `commands` block of your profile file (`.flk/profiles/<profile>.nix`):

```nix
commands = [
  { name = "dev"; script = ''npm run dev''; }
  { name = "test"; script = ''cargo test --all''; }
];
```

Each command becomes a shell function (via `writeShellScriptBin`) when you activate the environment.

## Naming Rules

Command names must follow these rules:
- Letters, numbers, hyphens, and underscores only
- Cannot start with a hyphen
- Examples: `dev`, `run-tests`, `build_release`

## Examples

```bash
# Web development commands
flk cmd add dev "npm run dev"
flk cmd add build "npm run build && npm run typecheck"
flk cmd add db:migrate "npx prisma migrate dev"

# Rust project commands
flk cmd add watch "cargo watch -x 'run -- --port 8080'"
flk cmd add bench "cargo bench --all-features"

# Complex command from a file
echo '#!/bin/bash
echo "Running full CI pipeline..."
cargo fmt --check
cargo clippy -- -D warnings
cargo test --all' > scripts/ci.sh
flk cmd add ci --file scripts/ci.sh
```

## See Also

- [flk cmd command reference](./commands/cmd.md)
- [Core Concepts — Custom Commands](./concepts.md#custom-commands)
