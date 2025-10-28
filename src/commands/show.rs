use std::path::Path;

use anyhow::{Context, Result};

use crate::flake::parser;

pub fn run_show() -> Result<()> {
    let flake_path = Path::new("flake.nix");
    let flake_info = parser::parse_flake(flake_path.to_str().context("Invalid path encoding")?)?;

    println!("{}", flake_info);

    Ok(())
}
