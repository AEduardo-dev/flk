//! # Flake Generator
//!
//! Template-based generation of Nix files for flk-managed projects.
//!
//! This module loads Nix templates from the `templates/` directory (embedded at compile time)
//! and provides functions to generate the various files that make up a flk project structure.
//!
//! ## Generated Files
//!
//! - `flake.nix` - Root flake that delegates to `.flk/default.nix`
//! - `.flk/default.nix` - Orchestrator that loads profiles and generates outputs
//! - `.flk/profiles/default.nix` - Auto-imports all profile files
//! - `.flk/profiles/<type>.nix` - Language-specific profile (rust, python, etc.)
//! - `.flk/overlays.nix` - Overlay configuration for version pinning
//! - `.flk/pins.nix` - Version pinning sources

use anyhow::{Ok, Result};

const ROOT_FLAKE_TEMPLATE: &str = include_str!("../../templates/flake.nix");
const HELPER_TEMPLATE: &str = include_str!("../../templates/default.nix");
const IMPORTER_TEMPLATE: &str = include_str!("../../templates/profiles/default.nix");
const OVERLAYS_TEMPLATE: &str = include_str!("../../templates/overlays.nix");
const PINS_TEMPLATE: &str = include_str!("../../templates/pins.nix");

const GENERIC_TEMPLATE: &str = include_str!("../../templates/profiles/base.nix");
const RUST_TEMPLATE: &str = include_str!("../../templates/profiles/rust.nix");
const PYTHON_TEMPLATE: &str = include_str!("../../templates/profiles/python.nix");
const NODE_TEMPLATE: &str = include_str!("../../templates/profiles/node.nix");
const GO_TEMPLATE: &str = include_str!("../../templates/profiles/go.nix");

/// Generate the root `flake.nix` content.
///
/// This creates the top-level flake that delegates to `.flk/default.nix`.
pub fn generate_root_flake() -> Result<String> {
    Ok(ROOT_FLAKE_TEMPLATE.to_string())
}

/// Generate the `.flk/default.nix` helper module.
///
/// This orchestrator file loads profiles, overlays, and generates the
/// development shell outputs.
pub fn generate_helper_module() -> Result<String> {
    Ok(HELPER_TEMPLATE.to_string())
}

/// Generate the `.flk/profiles/default.nix` auto-importer.
///
/// This file automatically discovers and imports all `.nix` files
/// in the profiles directory.
pub fn generate_importer_module() -> Result<String> {
    Ok(IMPORTER_TEMPLATE.to_string())
}

/// Generate a language-specific profile template.
///
/// # Arguments
///
/// * `project_type` - One of: "rust", "python", "node", "go", or any other value for generic
///
/// # Returns
///
/// The Nix content for the profile template.
pub fn generate_flake(project_type: &str) -> Result<String> {
    let template = match project_type {
        "rust" => RUST_TEMPLATE,
        "python" => PYTHON_TEMPLATE,
        "node" => NODE_TEMPLATE,
        "go" => GO_TEMPLATE,
        _ => GENERIC_TEMPLATE,
    };

    Ok(template.to_string())
}

/// Generate the `.flk/overlays.nix` overlay configuration.
///
/// This file configures Nix overlays for version-pinned packages
/// and the rust-overlay.
pub fn generate_overlays() -> Result<String> {
    Ok(OVERLAYS_TEMPLATE.to_string())
}

/// Generate the `.flk/pins.nix` version pinning configuration.
///
/// This file stores sources and pinned package mappings for
/// version-specific package installations.
pub fn generate_pins() -> Result<String> {
    Ok(PINS_TEMPLATE.to_string())
}
