# flk hook

Generate shell hooks that enable the `refresh` and `switch` commands for hot-reloading your development environment.

```bash
flk hook bash
flk hook zsh
flk hook fish
```

**Usage**

Add the hook output to your shell profile to enable `refresh` and `switch`:

```bash
# Bash (~/.bashrc)
eval "$(flk hook bash)"

# Zsh (~/.zshrc)
eval "$(flk hook zsh)"

# Fish (~/.config/fish/config.fish)
flk hook fish | source
```

**Supported Shells**
- `bash`
- `zsh`
- `fish`

**Commands Provided by the Hook**

Once sourced, two shell functions become available:

### `refresh`

Reload the current development environment. Picks up changes you've made (added packages, env vars, commands) without leaving the shell.

```bash
# After adding a package
flk add ripgrep
refresh
```

- If direnv is present and `.envrc` exists, runs `direnv reload`
- Otherwise, runs `nix develop` with the current profile
- Uses the `FLK_FLAKE_REF` environment variable to determine the active profile

### `switch <profile>`

Switch to a different profile and reload the environment.

```bash
switch backend
switch frontend
```

- Validates the profile name before switching
- Updates `FLK_FLAKE_REF` and reloads via direnv or `nix develop`

**Notes**
- The hook integrates with direnv automatically — if `.envrc` is present, it uses `direnv reload` instead of `exec nix develop`
- Profile names must be alphanumeric (with `-` or `_`)
- See also: [flk activate](./activate.md), [flk direnv](./direnv.md)
