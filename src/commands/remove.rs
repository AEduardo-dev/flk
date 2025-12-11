use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::flake::parsers::{packages::parse_packages_section, utils::get_default_shell_profile};

pub fn run_remove(package: &str) -> Result<()> {
    let flake_path = Path::new(".flk/profiles/").join(format!(
        "{}.nix",
        get_default_shell_profile().context("Could not find default shell profile (flake.nix)")?
    ));

    if package.trim().is_empty() {
        bail!("Package name cannot be empty!");
    }

    let flake_content = fs::read_to_string(&flake_path).context("Failed to read flake.nix")?;
    let section = parse_packages_section(&flake_content)?;

    if !section.package_exists(package) {
        bail!(
            "Package '{}' is not present in the packages declaration",
            package
        );
    }

    let updated_content = section.remove_package(&flake_content, package)?;
    fs::write(flake_path, updated_content).context("Failed to write flake.nix")?;

    println!(
        "{} Package '{}' removed successfully!",
        "✓".green().bold(),
        package
    );

    Ok(())
}
