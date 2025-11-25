use std::path::Path;

use anyhow::{Context, Result};

use crate::flake::parser;

pub fn run_list() -> Result<()> {
    let flake_path = Path::new(".flk/profiles/").join(format!(
        "{}.nix",
        parser::get_default_shell_profile()
            .context("Could not find default shell profile (flake.nix)")?
    ));

    let flake_info = parser::parse_flake(flake_path.to_str().context("Invalid path encoding")?)?;

    flake_info.display_packages();
    Ok(())
}
