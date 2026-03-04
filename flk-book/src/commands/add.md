# flk add

Add a package to your `flake.nix`.

```bash
flk add ripgrep
flk add git
flk add nodejs
flk add ripgrep --version '15.1.0'   # pinned version
flk add cargo-watch --profile backend  # specific profile
```

**Options**
- `-v, --version <VERSION>`: pin to a specific version
- `-p, --profile <PROFILE>`: target a specific profile instead of the default

**Behavior**
- Validates the package exists (`nix-versions`).
- Writes to `.flk/profiles/<profile>.nix`; updates `.flk/pins.nix` when pinning.
- Fails if the package is already present.
