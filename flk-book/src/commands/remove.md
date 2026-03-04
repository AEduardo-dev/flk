# flk remove

Remove a package from your `flake.nix`.

```bash
flk remove ripgrep
flk remove ripgrep --profile backend
```

**Options**
- `-p, --profile <PROFILE>`: target a specific profile instead of the default

**Behavior**
- Removes from `.flk/profiles/<profile>.nix`.
- Cleans up pinned entries from `.flk/pins.nix` when needed.
- Errors if the package is not present.
