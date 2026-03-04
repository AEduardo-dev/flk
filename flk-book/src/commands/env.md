# flk env

Manage environment variables for your dev shell.

```bash
flk env add DATABASE_URL "postgresql://localhost:5432/mydb"
flk env remove DATABASE_URL
flk env list
```

**Subcommands**
- `add <NAME> <VALUE>`: add/update a variable
- `remove <NAME>`: delete a variable
- `list`: show all configured variables

**Options**
- `-p, --profile <PROFILE>`: target a specific profile instead of the default

**Notes**
- Names must start with a letter/underscore and contain only letters, numbers, underscores.
