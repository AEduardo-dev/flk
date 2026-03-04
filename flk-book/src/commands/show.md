# flk show

Pretty-print the current flake configuration for inspection.

```bash
flk show
```

**Behavior**
- Parses `flake.nix` and all profiles in `.flk/profiles/`
- Displays a structured summary including:
  - Flake inputs (name, URL, type)
  - Each profile with its packages, environment variables, and custom commands
- Useful for verifying your configuration at a glance without reading raw Nix files

**Example Output**

```
Flake Inputs:
  nixpkgs: github:NixOS/nixpkgs/nixos-unstable (indirect)
  nix-profile-lib: github:AEduardo-dev/nix-profile-lib (github)

Profile: rust
  Packages:
    • ripgrep
    • cargo-watch
  Environment Variables:
    RUST_LOG = "debug"
  Commands:
    dev: cargo watch -x run
    test: cargo test --all
```

**Notes**
- Shows all profiles, not just the default
- Read-only — does not modify any files
