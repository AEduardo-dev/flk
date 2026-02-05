# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.1.3] - 2025-10-29

### 🚀 Features

- Add visual spinner for command or method wrapping
- Wrap long running nix commands with spinner output

### 🐛 Bug Fixes

- Make release-plz create only tags

### 💼 Other

- Remove release override in workspace config

### ⚙️ Miscellaneous Tasks

- Update issue templates
- Remove previous templates
- Add git cliff tool for changelog management
- Update release-plz configuration to use git-cliff
- Point roadmap to roadmap issue
- Make changelog follow git-cliff format
- Update lockfile
## [unreleased]
## [0.5.3] - 2026-02-05

### ⚙️ Miscellaneous Tasks

- Add dependabot for weekly dep checks
## [0.5.2] - 2026-01-22

### 🚀 Features

- Add shell hook command for refresh/switch in Bash, Zsh, and Fish
- Support FLK_PROFILE and custom flake refs in direnv and hooks
- Improve activation and init messages for clarity and shell integration
- Update direnv integration tests for new flake profile handling

### 📚 Documentation

- Add documentation for added hook command
## [0.5.1] - 2026-01-20

### 🚀 Features

- Add Rust package and app outputs using flake-utils
- *(templates)* Update python profile and nixpkgs version

### 🐛 Bug Fixes

- Correct package reference to use pkgs.<package> format
- *(ci)* Add --impure flag to nix build in release-cache workflow

### 💼 Other

- *(template)* Add python hook for better venv management

### 📚 Documentation

- Expand installation instructions with Nix and release binaries

### 🧪 Testing

- *(integration)* Relax python version check in template assertion

### ⚙️ Miscellaneous Tasks

- Remove Cargo.lock from ignored files
- Add release cache workflow and update Nix install action
- Bump flk to 0.5.0 and clean up unused import in integration tests
- *(ci)* Update release-cache workflow runners and remove qemu setup
- *(github)* Switch to cachix/install-nix-action for Nix installation in release-cache workflow
- *(ci)* Update release-cache workflow runners and trigger conditions
## [0.5.0] - 2026-01-18

### 🚀 Features

- Update parsers implementation to use nom
- Add utils for nom parsing
- Update parsers for nom usage
- Update packages listing method with new parsing
- Update search operations with new parsing
- Calculate insertion point for indentation consistency
- Add skeleton for overlays parser and structure of future flow
- Introduce overlays and pins parsers
- Remove 'with pkgs' to support pin syntax
- Pin packages in pins file
- Update parsers and tests for overlays
- Use new parsers for versioned packages input
- Subdivide interfaces for easier usage
- Expand parsing utilities
- Use new parsing utilities for packages parsing
- Adjust overlays file parsing for all sections
- Add rendering utilities for nix file dumping
- Update add and remove package commands to support pins interaction
- Remove shell wrapper implementation
- Remove gitignore for backup of flake lock
- Add shellhook interfaces and update imports and mods
- Add commands section to profiles
- Parse commands section and render back full section
- Use new methods in cli command implementation
- Move mutation helpers to interfaces module
- Refactor shell_hook to use ShellHookSection struct
- Remove redundant comments from nix_render and overlays modules
- Add direnv specific commands
- Update readme with project struct and commands

### 🐛 Bug Fixes

- Dead code warnings
- Remove curl from packages list for nix darwin compat
- Use proper format for system args
- Dead code warnings
- Python and node template dependencies mismatches

### 🚜 Refactor

- Remove input loop and simplify activation flow

### 📚 Documentation

- Update README to include upgrade notes for v0.5.0
- *(readme)* Add direnv commands docs and remove deprecated install

### 🧪 Testing

- Improve error handling in add_command tests

### ⚙️ Miscellaneous Tasks

- Adjust packages tests to use new parsing struct
- Add nom dependency
- Update tests for new parsers
- Remove tokio dep
- Introduce overlays and pins tests
- Update env
- Update structure of control files for testing
- Update unit tests
- Update profile test data with commands section
- Update tests for new commands section parsing checks
- Update python template tests
## [0.4.0] - 2025-12-04

### 🚀 Features

- Adapt search methods to use nix-version
- Display search results as list in base search
- Migrate all parsers to separate files
- Update commands for new parsers struct
- Update tests for new parsers struct

### 🐛 Bug Fixes

- Format issues

### ⚙️ Miscellaneous Tasks

- Add acknowledgements for nix-versions tool
- Update filetree in README
## [0.3.1] - 2025-11-27

### 💼 Other

- Add system as argument
## [0.3.0] - 2025-11-27

### 🚀 Features

- [**breaking**] #17 multi profile support ([#51](https://github.com/AEduardo-dev/flk/pull/51))
- [**breaking**] Add split of commands ([#53](https://github.com/AEduardo-dev/flk/pull/53))
- Move and adapt flake template implementations
- Simplify base flake to follow import early approach
- Generate profile names and permutations
- Add auto import all files in the directory non-recursively
- Update init command to new dendritic approach
- Update parsers implementation (work in progress)
- Use signal instead of exit code for refresh and switch
- Use string literal for indent constants
- Implement consistent brace find approach for parsing
- Update implementation for command addition
- Adjust base commands to manage default env
- Fix package insertion to provide consistent indentation
- Add profile name support for all base commands
- Improve parsing for flake operations
- Add profile support and improve export logic
- Import and apply overlay files
- Add overlay and pins for version pinning
- Update tests
- Allow usage of utils in tests
- Make activation command unix and windows compatible

### 🐛 Bug Fixes

- Pass system input for rust overlay
- Clippy suggestions
- Parsing of packages with prefix

### ⚙️ Miscellaneous Tasks

- Remove gitignore of flk dir
- Remove description field from flake config
- Correct integration tests to follow new paths
- Update unit tests for new struct
- Add missing cargo packages
- Make default shell extract public and independent
- Add visual feedback during package validation
- Remove helper hooks from flk
- Update dev environment to new structure
- Add impure flag to activation commands
- Update lockfile ref
- Adjust test to new profile outputs
- Update documentation
- Update tool descriptor
- Update dist
## [0.2.0] - 2025-11-12

### 🚀 Features

- #40 env activation and hot-reload ([#49](https://github.com/AEduardo-dev/flk/pull/49))
- [**breaking**] Modify templates to manage devShell and Containers
- [**breaking**] Add export command for docker container exporting
- Add export to podman and json formats
- Add serialize and deserialize traits for json export/import
- [**breaking**] Update parsers for structure json dumping and new flake struct
- Add podman image export output to flake
- Adapt parsers to new flake structure
- Add podman export build section for all templates

### 🐛 Bug Fixes

- Update to python313 and remove poetry plugins ([#43](https://github.com/AEduardo-dev/flk/pull/43))
- Make version command report cargo information ([#44](https://github.com/AEduardo-dev/flk/pull/44))
- DevEnv import in shell
- Unit tests flake mocks
- Update parsers to work with test scenarios
- Update unit tests
- Clippy suggestions

### ⚙️ Miscellaneous Tasks

- Update minimal rust version
- Update dev env
- Update gitignore for json exports
- Update parsers for new flake syntax
- Update integration tests to new outputs
- Update tests for new parsers
- Update documentation with new command

### ⚙️ Miscellaneous Tasks

- Update issue templates
- Remove previous templates

## [0.1.2] - 2025-10-29

### 🚀 Features

- Add completions command
- Add clap_complete for autogeneration of completions
- Add tests for completions generation

### ⚙️ Miscellaneous Tasks

- Add issue template
- Add code of conduct
- Add contributing guidelines
- Add issue templates
- Update readme
- Release v0.1.2

## [0.1.1] - 2025-10-28

### 🐛 Bug Fixes

- Correct typo for actions input
- Add clippy suggestions
- Apply clippy suggestions for struct initialization

### ⚙️ Miscellaneous Tasks

- Update version of nix installer
- Insert escape chars to make command name be parsed
- Fix token permissions for release management
- Release v0.1.1

## [0.1.0] - 2025-10-28

### 🚀 Features

- Add remove pckg command functionality
- Add parsers for package deletion
- Improve python template
- Add list command implementation
- Optimize parsers for flake info extraction
- Improve parsing and printing of flake information
- Add show command
- Utilize flake_info helper methods for printing of info
- Add update command
- Add environment variables related commands
- Add lockfile operations and backup
- Add cargo dist configuration
- Add release-plz configuration

### 🐛 Bug Fixes

- Correct format flake after insertions
- Add context to errors

### 💼 Other

- Use standard method for env vars print out
- Remove folder
- Cargo format issues

### ⚙️ Miscellaneous Tasks

- Improve logging for errors
- Add comment to isolate user packages section
- Update README
- Update documentation
- Update dev flake lockfile
- Update documentation
- Add missing fields
- Add missing fields
- Remove dead code
- Add tests
- Update documentation for release
- Improve writing of README
- Add workflows for test release and automated release
- Add base changelog
- Add unit tests
- Update development environment
- Update template for rust lang
