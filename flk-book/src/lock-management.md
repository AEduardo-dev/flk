# Lock File Management

flk protects your `flake.lock` file with automatic backups, preview-before-update, and easy rollback.

## Commands

```bash
flk lock show              # inspect current lock file
flk lock history           # list available backups
flk lock restore latest    # restore most recent backup
flk update                 # update inputs (creates backup first)
flk update --show          # preview updates without applying
```

## How It Works

### Automatic Backups

Every time you run `flk update`, a backup of the current `flake.lock` is created before any changes are made. Backups are stored in `.flk/backups/` with timestamped filenames:

```
.flk/backups/
├── flake.lock.2025-01-27_14-30-00
├── flake.lock.2025-02-15_09-45-22
└── flake.lock.2025-03-01_16-00-00
```

### Preview Updates

Use `flk update --show` to see what would change without modifying your lock file:

```bash
flk update --show
```

This temporarily updates the lock file, shows the diff, then restores the original — a safe way to check for upstream changes.

### Inspecting the Lock File

```bash
flk lock show
```

Displays structured information about each input in your `flake.lock`, including:
- Input name and type (e.g., github, indirect)
- Source URL
- Current revision/commit
- Last modified date

### Viewing Backup History

```bash
flk lock history
```

Lists all available backups with their timestamps and sizes, so you can identify which snapshot to restore.

### Restoring a Backup

```bash
flk lock restore latest                    # most recent backup
flk lock restore 2025-01-27_14-30-00       # specific timestamp
```

Replaces the current `flake.lock` with the selected backup.

## Interaction with Version Pinning

When you pin a package version with `flk add --version`, the pinning data is stored in `.flk/pins.nix` (not in `flake.lock`). Lock file backups and restores do not affect version pins — they only manage the Nix input lock state.

## Best Practices

- Run `flk update --show` before `flk update` to review changes
- Use `flk lock restore latest` immediately if an update causes issues
- Backups accumulate over time; periodically clean old ones from `.flk/backups/` if needed

## See Also

- [flk lock command reference](./commands/lock.md)
- [flk update command reference](./commands/update.md)
- [Core Concepts — Lock Files](./concepts.md#lock-files)
