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
## [0.1.4] - 2025-10-30

### 🐛 Bug Fixes

- Update to python313 and remove poetry plugins ([#43](https://github.com/AEduardo-dev/flk/pull/43))
- Make version command report cargo information ([#44](https://github.com/AEduardo-dev/flk/pull/44))

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
