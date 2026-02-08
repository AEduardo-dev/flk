# flk lock

Manage `flake.lock` backups and inspection.

```bash
flk lock show              # show lock info
flk lock history           # list backups
flk lock restore latest    # restore most recent backup
flk lock restore 2025-01-27_14-30-00
```

**Subcommands**
- `show`: pretty-prints lock details
- `history`: lists available backups
- `restore <BACKUP>`: restores a backup (timestamp or `latest`)
