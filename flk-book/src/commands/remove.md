# flk remove

Remove a package from your `flake.nix`.

```bash
flk remove ripgrep
```

**Behavior**
- Removes from `.flk/profiles/<profile>.nix`.
- Cleans up pinned entries from `.flk/pins.nix` when needed.
- Errors if the package is not present.
