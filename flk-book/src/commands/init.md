# flk init

Initialize a new `flake.nix` for your project.

```bash
flk init                    # Auto-detect project type
flk init --template rust    # Use Rust template
flk init --force            # Overwrite existing flake.nix
```

**Options**

- `-t, --template <TYPE>`: `rust`, `python`, `node`, `go`, `generic` (auto-detect if omitted)
- `-f, --force`: overwrite an existing `flake.nix`

**What it does**

- Creates `flake.nix`, `.flk/` helper files, and a default profile under `.flk/profiles/`
- Auto-detects project type from common files (Cargo.toml, package.json, pyproject/requirements, go.mod)
- Prints next steps, including adding `flk hook <shell>` to your shell config
