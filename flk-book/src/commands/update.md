# flk update

Update all flake inputs (creates a lockfile backup first).

```bash
flk update           # apply updates
flk update --show    # preview without applying
```

**Options**
- `--show`: check for updates without modifying `flake.lock`

**Behavior**
- Backs up `flake.lock` to `.flk/backups` before applying.
- Uses `nix flake update`; preview restores the original lockfile after diffing.
