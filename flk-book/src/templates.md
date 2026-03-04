# Project Templates

flk uses templates to scaffold development environments tailored to your project's language and toolchain.

## Using Templates

```bash
flk init --template rust
flk init --template python
flk init --template node
flk init --template go
flk init --template generic
```

## Auto-Detection

If you omit `--template`, flk auto-detects the project type by looking for:

| File | Detected Template |
|---|---|
| `Cargo.toml` | `rust` |
| `package.json` | `node` |
| `pyproject.toml` or `requirements.txt` | `python` |
| `go.mod` | `go` |
| _(none found)_ | `generic` |

## Available Templates

### `base`

Minimal template with no language-specific tools. Used as the default when creating profiles via `flk profile add`.

### `rust`

Includes Rust toolchain essentials:
- Rust compiler and Cargo
- Common development tools (rust-analyzer, clippy, rustfmt)
- Build dependencies (pkg-config, openssl)

### `python`

Includes Python development essentials:
- Python interpreter
- pip and virtualenv support
- Common development tools

### `node`

Includes Node.js development essentials:
- Node.js runtime
- npm package manager
- Common development tools

### `go`

Includes Go development essentials:
- Go compiler
- Go tools (gopls, etc.)

### `generic`

A minimal starting point with basic utilities. Use this when your project doesn't fit a specific language category or when you want to build a custom environment from scratch.

## Template Files

Templates are stored in the `templates/` directory of the flk source and are embedded at compile time:

- `templates/flake.nix` — Root flake template
- `templates/default.nix` — Profile loader/importer
- `templates/pins.nix` — Version pinning structure
- `templates/overlays.nix` — Package overlays
- `templates/profiles/*.nix` — Language-specific profile templates

## Creating Additional Profiles

After initialization, you can create additional profiles from any template:

```bash
flk profile add backend --template rust
flk profile add frontend --template node
flk profile add scripts --template python
```

## See Also

- [flk init command reference](./commands/init.md)
- [flk profile command reference](./commands/profile.md)
- [Architecture — Template System](./architecture.md#template-system)
