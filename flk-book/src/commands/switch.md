# flk switch / refresh

Shell commands for hot-reloading and profile switching within an active development environment. These are provided by the [flk hook](./hook.md) and are not standalone `flk` subcommands.

## Setup

Add the shell hook to your profile:

```bash
# Bash
eval "$(flk hook bash)"

# Zsh
eval "$(flk hook zsh)"

# Fish
flk hook fish | source
```

## `refresh`

Reload the current environment to pick up configuration changes.

```bash
# Make changes
flk add ripgrep
flk env add MY_VAR "hello"

# Apply without leaving the shell
refresh
```

**Behavior**
- With direnv: runs `direnv reload`
- Without direnv: reuses a saved `nix develop` profile when it is current
- Without direnv: refreshes that saved profile when the environment definition changes
- Reads the active profile from `FLK_FLAKE_REF` environment variable

## `switch <profile>`

Switch to a different profile and reload the environment.

```bash
switch backend
switch frontend
```

**Behavior**
- Validates the profile name (alphanumeric, `-`, `_` only)
- Sets `FLK_FLAKE_REF` to the new profile reference
- Reloads via direnv or `nix develop` as appropriate, reusing the saved profile cache when possible

## Direnv Integration

When direnv is available and `.envrc` exists, both `refresh` and `switch` use `direnv reload` for a seamless experience. Without direnv, they use `exec nix develop`, which replaces the current shell process.

**Notes**
- Supported shells: bash, zsh, fish
- Requires the flk shell hook to be sourced in your shell profile
- See also: [flk hook](./hook.md), [flk activate](./activate.md)
