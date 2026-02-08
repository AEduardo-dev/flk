use std::path::Path;

use anyhow::{Context, Result};

/// Pretty-print the current flake.nix for inspection.
use flk::flake::parsers::flake::parse_flake;

pub fn run_show() -> Result<()> {
    let flake_path = Path::new("flake.nix");
    let flake_info = parse_flake(flake_path.to_str().context("Invalid path encoding")?)?;

    println!("{}", flake_info);

    Ok(())
}
