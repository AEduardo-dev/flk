# flk search

Search nixpkgs for packages.

```bash
flk search ripgrep
flk search python --limit 20
```

**Options**
- `-l, --limit <NUMBER>`: number of results (default 10)

**Notes**
- Uses `nix-versions` under the hood.
- For detailed info, use `flk deep-search <PACKAGE>`.
