//! # Show Command Handler
//!
//! Pretty-print the current flake configuration for inspection.

use std::path::Path;

use anyhow::{Context, Result};

use flk::flake::parsers::flake::parse_flake;

/// Display the current flake configuration in a human-readable format.
pub fn run_show() -> Result<()> {
    let flake_path = Path::new("flake.nix");
    let flake_info = parse_flake(flake_path.to_str().context("Invalid path encoding")?)?;

    println!("{}", flake_info);

    Ok(())
}
