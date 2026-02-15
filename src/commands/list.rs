//! # List Command Handler
//!
//! List packages in the active development profile.

use colored::Colorize;
use std::path::Path;

use anyhow::{Context, Result};

use flk::flake::parsers::{packages::parse_packages_section, utils::resolve_profile};

pub fn run_list(target_profile: Option<String>) -> Result<()> {
    let profile = resolve_profile(target_profile)?;
    let flake_path = Path::new(".flk/profiles/").join(format!("{}.nix", profile));

    let flake_content = std::fs::read_to_string(&flake_path).context("Failed to read flake.nix")?;
    let section = parse_packages_section(&flake_content)
        .context("Failed to parse packages section in flake.nix")?;
    let packages_info = section.to_packages();

    if packages_info.is_empty() {
        println!(
            "{} No packages found in the current profile.",
            "✗".red().bold()
        );
        return Ok(());
    }

    for pkg in packages_info {
        println!("{} {}", "•".green(), pkg.name);
    }

    Ok(())
}
