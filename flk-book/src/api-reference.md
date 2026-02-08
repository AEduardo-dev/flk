# API Documentation

The Rust API is documented via `cargo doc`. Generate it locally with:

```bash
cargo doc --all-features --open
```

Key crates/modules:
- `flk::flake`: flake generation, parsing, and interfaces (profiles, overlays, shell hooks)
- `flk::commands` (binary crate): CLI entrypoints and command dispatch
- `flk::utils`: backups, visuals/spinners, helpers

When contributing:
- Keep public items documented with `///`.
- Prefer examples in doc comments when behavior is non-obvious.
- Run `cargo doc --all-features` in CI or locally before publishing major API changes.
