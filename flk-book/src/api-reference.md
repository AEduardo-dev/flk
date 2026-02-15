# API Documentation

The flk library exposes Rust APIs for programmatic access to flake generation and parsing.

## Generating Documentation

Build the API documentation locally:

```bash
cargo doc --no-deps --open
```

Or with all features and dependencies:

```bash
cargo doc --all-features --open
```

## Module Overview

### `flk::flake`

Core flake functionality:

- **`flk::flake::generator`** - Template loading and flake generation
  - `generate_flake(project_type)` - Generate profile content for a project type
  - `generate_helper_module()` - Generate the `.flk/default.nix` loader
  - `generate_pins()` - Generate empty pins file

- **`flk::flake::parsers`** - Nix file parsing and modification
  - `packages` - Parse/modify `packages = [ ... ];` sections
  - `env` - Parse/modify `envVars = { ... };` sections  
  - `commands` - Parse/modify shell hook commands
  - `overlays` - Parse/modify `pins.nix` for version pinning
  - `flake` - Parse top-level flake structure
  - `utils` - Profile resolution and parsing helpers

- **`flk::flake::interfaces`** - Data structures
  - `FlakeConfig` - Complete flake configuration
  - `Profile` - Single profile with packages, commands, env vars
  - `Package` - Package entry with optional version
  - `EnvVar` - Environment variable key-value pair

- **`flk::flake::nix_render`** - Safe Nix syntax rendering
  - `nix_string(s)` - Escape string for Nix double-quoted strings
  - `nix_attr_key(s)` - Format attribute key (quote if needed)

### `flk::utils`

Utility functions:

- **`flk::utils::backup`** - Lockfile backup management
- **`flk::utils::visual`** - Spinner and progress display

## Example Usage

```rust
use flk::flake::generator::generate_flake;
use flk::flake::parsers::packages::parse_packages_section;

// Generate a Rust profile template
let profile_content = generate_flake("rust")?;

// Parse packages from an existing profile
let content = std::fs::read_to_string(".flk/profiles/rust.nix")?;
let section = parse_packages_section(&content)?;

// Add a package (returns new content directly)
let new_content = section.add_package(&content, "ripgrep", None);
std::fs::write(".flk/profiles/rust.nix", new_content)?;
```

## Contributing to the API

When contributing:

- Document all public items with `///` doc comments
- Include `# Arguments`, `# Returns`, and `# Errors` sections where applicable
- Add examples in doc comments for non-obvious behavior
- Run `cargo doc --no-deps` before submitting to check for warnings
- Keep internal/unstable functions prefixed with underscore (`_parse_*`)
