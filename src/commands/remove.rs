use anyhow::{bail, Context, Result};
use colored::Colorize;
use flk::flake::parsers::overlays::remove_pinned_package_with_cleanup;
use std::fs;
use std::path::Path;

use flk::flake::parsers::{packages::parse_packages_section, utils::resolve_profile};

pub fn run_remove(package: &str, target_profile: Option<String>) -> Result<()> {
    let profile = resolve_profile(target_profile)?;
    let flake_path = Path::new(".flk/profiles/").join(format!("{}.nix", profile));

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

    // Check if package is pinned to a version
    if section
        .entries
        .iter()
        .any(|e| e.name == package && e.version.is_some())
    {
        let pins_path = ".flk/pins.nix";
        let pins_content = fs::read_to_string(pins_path).context("Failed to read pins.nix file")?;

        let updated_pins_content = remove_pinned_package_with_cleanup(&pins_content, package)?;
        fs::write(pins_path, updated_pins_content).context("Failed to write pins.nix")?;
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
