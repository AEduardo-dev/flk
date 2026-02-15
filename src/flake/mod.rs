//! # Flake Management
//!
//! This module provides functionality for generating, parsing, and manipulating
//! Nix flake files and related configuration.
//!
//! ## Architecture
//!
//! The flk tool uses a "dendritic" architecture where the main `flake.nix` delegates
//! to a `.flk/` directory structure:
//!
//! ```text
//! project/
//! ├── flake.nix           # Root flake (delegates to .flk/default.nix)
//! └── .flk/
//!     ├── default.nix     # Orchestrator that loads profiles
//!     ├── overlays.nix    # Nix overlays configuration
//!     ├── pins.nix        # Version pinning for packages
//!     └── profiles/
//!         ├── default.nix # Auto-imports all profiles
//!         ├── rust.nix    # Language-specific profile
//!         └── python.nix  # Another profile
//! ```
//!
//! ## Modules
//!
//! - [`generator`] - Template-based generation of flake files from embedded templates
//! - [`parsers`] - Nom-based parsers for reading and modifying Nix file sections
//! - [`interfaces`] - Data structures representing flake configuration elements
//! - [`nix_render`] - Safe rendering of Nix strings and attributes

pub mod generator;
pub mod interfaces;
pub mod nix_render;
pub mod parsers;
