use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::{fs, path};

use crate::flake::parsers::{
    packages::{add_package_to_profile, package_exists},
    utils::get_default_shell_profile,
};
use crate::nix::run_nix_command;
use crate::utils::visual::with_spinner;

pub fn run_add(package: &str, version: Option<String>) -> Result<()> {
    let flake_path = path::Path::new(".flk/profiles/").join(format!(
        "{}.nix",
        get_default_shell_profile().context("Could not find default shell profile (flake.nix)")?
    ));
    let flake_content = fs::read_to_string(&flake_path).with_context(|| {
        format!(
            "Failed to read flake file at {}",
            flake_path.to_str().unwrap()
        )
    })?;

    // Validate package name
    if package.trim().is_empty() {
        bail!("Package name cannot be empty");
    }

    with_spinner("Validating package...", || {
        validate_package_exists(package).context("Failed to execute nix search. Is nix installed?")
    })?;

    let package_to_add = if let Some(ver) = &version {
        println!(
            "{} Adding package: {} (pinned to version {})",
            "→".blue().bold(),
            package.green(),
            ver.yellow()
        );
        let package = format!("{}@{}", package, version.unwrap());
        package
    } else {
        println!("{} Adding package: {}", "→".blue().bold(), package.green());
        package.to_string()
    };

    // Check if package already exists
    if package_exists(&flake_content, &package_to_add, None)? {
        bail!(
            "Package '{}' is already in the packages declaration",
            package_to_add
        );
    }

    // Add the package to buildInputs
    let updated_content = add_package_to_profile(&flake_content, &package_to_add, None)?;

    // TODO: generate overlay for packade in overlays.nix
    // NOTE: package then needs to be generated under a name
    // so if possible let's use <package>@<version>

    // Write back to file
    fs::write(flake_path, updated_content).context("Failed to write flake.nix")?;

    println!(
        "{} Package '{}' added successfully!",
        "✓".green().bold(),
        package_to_add
    );
    println!("\n{}", "Next steps:".bold());
    println!("  1. Run {} to update your shell", "refresh".cyan());
    println!("  2. The package will be available in your environment");

    Ok(())
}

fn validate_package_exists(package: &str) -> Result<()> {
    let (_, stderr, success) = run_nix_command(&["run", "github:vic/nix-versions", package])
        .context("Failed to execute nix eval")?;

    if !success || stderr.contains("no packages found") {
        bail!(
            "Package {} does not exist or is marked as insecure. Aborting",
            package
        );
    }

    Ok(())
}
