# flk profile

Manage profiles for your flk project. Profiles let you maintain separate sets of packages, commands, and environment variables within the same project.

## Subcommands

### `flk profile add`

Create a new profile from a template.

```bash
flk profile add backend
flk profile add frontend --template node
flk profile add ci --template base --force
```

**Options**
- `<NAME>`: Profile name (alphanumeric, hyphens, underscores only)
- `-t, --template <TYPE>`: Template to use (`base`, `rust`, `python`, `node`, `go`, `generic`). Defaults to `base`
- `-f, --force`: Overwrite if profile already exists

**Behavior**
- Creates `.flk/profiles/<NAME>.nix` from the selected template
- Profile names are validated — no path separators or spaces allowed
- Fails if the profile already exists unless `--force` is used

### `flk profile remove`

Remove an existing profile.

```bash
flk profile remove frontend
```

**Behavior**
- Deletes `.flk/profiles/<NAME>.nix`
- Cannot remove the profile that is currently set as default — change the default first with `flk profile set-default`

### `flk profile list`

List all available profiles.

```bash
flk profile list
```

**Behavior**
- Lists all `.nix` files in `.flk/profiles/` (excluding `default.nix`)
- Profiles are sorted alphabetically

### `flk profile set-default`

Set which profile is used when no `--profile` flag is provided.

```bash
flk profile set-default backend
```

**Behavior**
- Updates the `defaultShell` attribute in `.flk/default.nix`
- The specified profile must already exist
- Affects all commands that use profile resolution (`add`, `remove`, `list`, `activate`, `export`, `cmd`, `env`)

## Profile Resolution

When you run a command without `--profile`, flk resolves the profile in this order:

1. Explicit `--profile` / `-p` argument
2. `FLK_FLAKE_REF` environment variable
3. `defaultShell` in `.flk/default.nix`
4. First available profile in `.flk/profiles/`

## Examples

```bash
# Set up a multi-profile project
flk init --template generic

# Create specialized profiles
flk profile add backend --template rust
flk profile add frontend --template node

# Add packages to specific profiles
flk add cargo-watch --profile backend
flk add nodejs_20 --profile frontend

# Switch the default
flk profile set-default backend

# Now commands target "backend" by default
flk add ripgrep          # goes to backend profile
flk add eslint --profile frontend  # explicit override
```

## See Also

- [Core Concepts — Profiles](../concepts.md#profiles)
- [flk activate](./activate.md)
- [flk hook (switch command)](./switch.md)
