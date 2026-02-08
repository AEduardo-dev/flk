# Environment Variables

Manage per-project variables without touching your global shell.

```bash
flk env add DATABASE_URL "postgresql://localhost:5432/mydb"
flk env remove DATABASE_URL
flk env list
```

Variables live in your `.flk/profiles/<profile>.nix` and load automatically on activation. Follow standard naming rules (start with letter/underscore; letters/numbers/underscores only). Avoid committing secrets—use environment-specific tooling or `direnv` if needed.
