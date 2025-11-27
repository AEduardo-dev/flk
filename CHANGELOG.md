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
