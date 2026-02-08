# flk cmd

Manage custom shell commands for your flk environment.

```bash
flk cmd add dev "npm run dev"
flk cmd add test "cargo test --all"
flk cmd list
flk cmd remove dev
```

**Subcommands**
- `add <NAME> <COMMAND> [--file <PATH>]`: add a command (inline or from file)
- `remove <NAME>`: delete a command
- `list`: list all custom commands

**Notes**
- Command names: letters, numbers, hyphens, underscores; cannot start with hyphen.
