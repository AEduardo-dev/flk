//! # Flake Data Structures
//!
//! This module defines the core data types used to represent flake configuration
//! elements throughout the `flk` codebase.
//!
//! ## Modules
//!
//! - [`profiles`] - Profile, Package, EnvVar, and FlakeConfig types
//! - [`overlays`] - Pinned package and overlay configuration types
//! - [`shellhooks`] - Custom command/shell hook types
//! - [`utils`] - Shared constants (indentation levels)
//!
//! ## Type Hierarchy
//!
//! ```text
//! FlakeConfig
//! ├── inputs: Vec<String>
//! └── profiles: Vec<Profile>
//!     ├── name: String
//!     ├── packages: Vec<Package>
//!     ├── env_vars: Vec<EnvVar>
//!     └── shell_hook: ShellHookSection
//!         └── entries: Vec<ShellHookEntry>
//! ```

pub mod overlays;
pub mod profiles;
pub mod shellhooks;
pub mod utils;
