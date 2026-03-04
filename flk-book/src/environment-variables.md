# Environment Variables

Manage per-project environment variables without touching your global shell configuration.

## Basic Usage

```bash
flk env add DATABASE_URL "postgresql://localhost:5432/mydb"
flk env add NODE_ENV "development"
flk env remove DATABASE_URL
flk env list
```

## Profile Targeting

Environment variables are stored per-profile. Use `--profile` to target a specific one:

```bash
flk env add API_URL "http://localhost:3000" --profile backend
flk env list --profile frontend
```

## How It Works

Variables are stored in the `envVars` block of your profile file (`.flk/profiles/<profile>.nix`):

```nix
envVars = {
  DATABASE_URL = "postgresql://localhost:5432/mydb";
  NODE_ENV = "development";
};
```

They are automatically exported when you activate the environment via `flk activate`, `nix develop`, or direnv.

## Naming Rules

Variable names must follow these rules:
- Start with a letter or underscore
- Contain only letters, numbers, and underscores
- Examples: `DATABASE_URL`, `MY_VAR`, `_PRIVATE_KEY`
- Invalid: `123VAR`, `my-var`, `my var`

## Security Considerations

> **Warning:** Do not store secrets (API keys, passwords, tokens) directly in profile files, especially if your project is version-controlled.

For sensitive values, consider:
- Using [direnv](./commands/direnv.md) with a `.envrc.local` file (add to `.gitignore`)
- Referencing secrets from a secrets manager at runtime
- Using environment-specific tooling outside of flk

## See Also

- [flk env command reference](./commands/env.md)
- [Core Concepts — Environment Variables](./concepts.md#environment-variables)
