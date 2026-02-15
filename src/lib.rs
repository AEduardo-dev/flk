//! # flk - Core Library
//!
//! This crate provides the core functionality for the `flk` CLI tool, which manages
//! Nix flake-based development environments.
//!
//! ## Modules
//!
//! - [`flake`] - Flake file generation, parsing, and manipulation
//!   - [`flake::generator`] - Template-based flake generation
//!   - [`flake::parsers`] - Nom-based parsers for Nix file sections
//!   - [`flake::interfaces`] - Data structures representing flake components
//!   - [`flake::nix_render`] - Safe Nix string/attribute rendering
//!
//! - [`utils`] - Shared utilities
//!   - [`utils::backup`] - Lock file backup and restore functionality
//!   - [`utils::visual`] - Terminal output formatting and progress indicators
//!
//! ## Example
//!
//! ```rust,ignore
//! use flk::flake::parsers::flake::parse_flake;
//! use flk::flake::generator::generate_flake;
//!
//! // Parse an existing flake configuration
//! let config = parse_flake("flake.nix")?;
//!
//! // Generate a new profile template
//! let rust_profile = generate_flake("rust")?;
//! ```

pub mod flake;
pub mod utils;
