# Lock File Management

flk keeps your `flake.lock` safe with built-in backups.

```bash
flk lock show
flk lock history
flk lock restore latest
flk update --show   # preview updates
flk update          # apply updates (creates backup)
```

- Backups are stored under `.flk/backups/`.
- `flk update` creates a backup before updating inputs.
- `flk lock restore <BACKUP>` rolls back to a chosen snapshot.
