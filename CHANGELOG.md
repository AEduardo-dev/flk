# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2](https://github.com/AEduardo-dev/flk/compare/v0.1.1...v0.1.2) - 2025-10-29

### Added

- add tests for completions generation
- add clap_complete for autogeneration of completions
- add completions command

### Other

- update readme
- add issue templates
- add contributing guidelines
- add code of conduct
- add issue template

## [0.1.1](https://github.com/AEduardo-dev/flk/compare/v0.1.0...v0.1.1) - 2025-10-28

### Fixed

- apply clippy suggestions for struct initialization
- add clippy suggestions
- correct typo for actions input

### Other

- fix token permissions for release management
- insert escape chars to make command name be parsed
- update version of nix installer

### Planned

- Version pinning support for packages
- Interactive TUI mode
- Plugin system
- Flake templates marketplace

## [0.1.0] - 2025-01-28

### Added

- Initial release of flk
- Project initialization with `flk init`
  - Auto-detection for Rust, Python, Node.js, and Go projects
  - Manual template selection with `--template` flag
  - Force overwrite with `--force` flag
- Package management
  - `flk search` - Search nixpkgs packages
  - `flk deep-search` - Get detailed package information
  - `flk add` - Add packages to flake.nix
  - `flk remove` - Remove packages from flake.nix
  - `flk list` - List all installed packages
- Custom command management
  - `flk add-command` - Add custom shell commands
  - `flk remove-command` - Remove custom commands
  - Support for inline commands and file sourcing
- Environment variable management
  - `flk env add` - Add environment variables
  - `flk env remove` - Remove environment variables
  - `flk env list` - List all environment variables
- Lock file management
  - `flk lock show` - Display lock file information
  - `flk lock history` - Show backup history
  - `flk lock restore` - Restore from backups
  - Automatic backup creation on updates
- Update functionality
  - `flk update` - Update all flake inputs
  - `flk update --show` - Preview updates without applying
- Display commands
  - `flk show` - Display flake configuration
- Pre-configured templates for multiple languages
  - Generic/default template
  - Rust template with rust-overlay
  - Python template with poetry
  - Node.js template with npm/pnpm/yarn
  - Go template with standard toolchain
- Comprehensive error handling and user-friendly messages
- Colored terminal output for better UX
- Backup system in `.flk/backups` directory
- Automatic `.gitignore` management for backup directory

### Technical

- Built with Clap for CLI parsing
- Async runtime with Tokio
- Nix command integration
- Flake.nix parsing and manipulation
- Package validation through nix eval

[Unreleased]: https://github.com/AEduardo-dev/flk/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/AEduardo-dev/flk/releases/tag/v0.1.0
