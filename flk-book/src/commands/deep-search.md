# flk deep-search

Get detailed version and attribute information about a specific package from nixpkgs.

```bash
flk deep-search ripgrep
flk deep-search python3
```

**Behavior**
- Queries nixpkgs using `nix-versions` under the hood
- Displays available versions, attribute paths, and package metadata
- More detailed than `flk search`, which only lists matching package names

**Example Output**

```
Package: ripgrep
  Version: 14.1.1
  Attribute: ripgrep
  Description: A utility that combines the usability of The Silver Searcher with the raw speed of grep
```

**When to Use**
- Use `flk search <term>` to find packages by name
- Use `flk deep-search <package>` to inspect a specific package in detail, especially to find available versions for pinning

**See Also**
- [flk search](./search.md)
- [flk add --version](./add.md)
