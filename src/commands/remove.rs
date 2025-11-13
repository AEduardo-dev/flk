use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::flake::parser;

pub fn run_remove(package: &str) -> Result<()> {
    let flake_path = Path::new("flake.nix");

    if !flake_path.exists() {
        bail!(
            "No flake.nix found in current directory. Run {} first if not initialized yet.",
            "flk init".yellow()
        );
    }

    if package.trim().is_empty() {
        bail!("Package name cannot be empty!");
    }

    let flake_content = fs::read_to_string(flake_path).context("Failed to read flake.nix")?;

    if !parser::package_exists(&flake_content, package, None)? {
        bail!(
            "Package '{}' is not present in the packages declaration",
            package
        );
    }

    let updated_content = parser::remove_package_from_profile(&flake_content, package, None)?;
    fs::write(flake_path, updated_content).context("Failed to write flake.nix")?;

    println!(
        "{} Package '{}' removed successfully!",
        "✓".green().bold(),
        package
    );

    Ok(())
}
