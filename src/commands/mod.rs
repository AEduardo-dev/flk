//! # Command Handlers
//!
//! This module contains the implementation of all CLI subcommands for `flk`.
//! Each submodule corresponds to a CLI command and contains the business logic
//! for that operation.
//!
//! ## Command Categories
//!
//! ### Project Setup
//! - [`init`] - Initialize a new flake environment with language detection
//! - [`activate`] - Enter the Nix development shell
//! - [`profiles`] - Create, remove, list, and set default profiles
//!
//! ### Package Management
//! - [`add`] - Add packages to the environment (with optional version pinning)
//! - [`remove`] - Remove packages from the environment
//! - [`search`] - Search nixpkgs for packages
//! - [`list`] - List installed packages
//!
//! ### Environment Customization
//! - [`command`] - Add/remove custom shell commands
//! - [`env`] - Add/remove environment variables
//!
//! ### State Management
//! - [`update`] - Update flake inputs
//! - [`lock`] - Manage flake.lock backups and restoration
//!
//! ### Integration & Export
//! - [`export`] - Export to Docker, Podman, or JSON
//! - [`direnv`] - Manage direnv integration
//! - [`hook`] - Generate shell hooks for bash/zsh/fish
//! - [`completions`] - Generate shell completions
//!
//! ### Display
//! - [`show`] - Pretty-print flake configuration

pub mod activate;
pub mod add;
pub mod command;
pub mod completions;
pub mod direnv;
pub mod env;
pub mod export;
pub mod hook;
pub mod init;
pub mod list;
pub mod lock;
pub mod profile_cache;
pub mod profiles;
pub mod remove;
pub mod search;
pub mod show;
pub mod update;
