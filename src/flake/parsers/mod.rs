//! # Nix File Parsers
//!
//! This module provides [nom]-based parsers for reading and modifying sections
//! of Nix files. Each parser is designed to:
//!
//! 1. Parse a specific section (packages, env vars, commands, etc.)
//! 2. Track byte positions for precise in-place modifications
//! 3. Support round-trip editing (parse → modify → render)
//!
//! ## Design Philosophy
//!
//! Rather than parsing entire Nix files (which would require a full Nix parser),
//! these parsers target specific, well-structured sections that flk generates
//! and manages. This allows for surgical edits while preserving comments,
//! formatting, and other content.
//!
//! ## Modules
//!
//! - [`packages`] - Parse and modify `packages = [ ... ];` sections
//! - [`mod@env`] - Parse and modify `envVars = { ... };` sections
//! - [`commands`] - Parse and modify `commands = [ ... ];` sections
//! - [`overlays`] - Parse and modify `pins.nix` overlays and sources
//! - [`flake`] - Parse top-level flake structure and inputs
//! - [`utils`] - Shared parsing utilities and combinators

pub mod commands;
pub mod env;
pub mod flake;
pub mod overlays;
pub mod packages;
pub mod utils;
