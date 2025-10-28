use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::flake::parser;
use crate::nix::run_nix_command;

pub fn run_add(package: &str, version: Option<String>) -> Result<()> {
    let flake_path = Path::new("flake.nix");

    if !flake_path.exists() {
        bail!(
            "No flake.nix found in current directory. Run {} first if not initialized yet.",
            "flk init".yellow()
        );
    }

    // Validate package name
    if package.trim().is_empty() {
        bail!("Package name cannot be empty");
    }

    let _ = validate_package_exists(package);

    let package_to_add = if let Some(ver) = version {
        println!(
            "{} Adding package: {} (pinned to version {})",
            "→".blue().bold(),
            package.green(),
            ver.yellow()
        );
        // TODO: Implement version pinning in Issue #5
        bail!(
            "Version pinning is not yet implemented. Track progress at: https://github.com/AEduardo-dev/flk/issues/5"
        );
    } else {
        println!("{} Adding package: {}", "→".blue().bold(), package.green());
        package.to_string()
    };

    // Read the current flake.nix
    let flake_content = fs::read_to_string(flake_path).context("Failed to read flake.nix")?;

    // Check if package already exists
    if parser::package_exists(&flake_content, &package_to_add)? {
        bail!(
            "Package '{}' is already in the packages declaration",
            package_to_add
        );
    }

    // Add the package to buildInputs
    let updated_content = parser::add_package_inputs(&flake_content, &package_to_add)?;

    // Write back to file
    fs::write(flake_path, updated_content).context("Failed to write flake.nix")?;

    println!(
        "{} Package '{}' added successfully!",
        "✓".green().bold(),
        package_to_add
    );
    println!("\n{}", "Next steps:".bold());
    println!("  1. Run {} to update your shell", "nix develop".cyan());
    println!("  2. The package will be available in your environment");

    Ok(())
}

fn validate_package_exists(package: &str) -> Result<()> {
    let query = format!("nixpkgs#{}", package);

    let (_, stderr, success) =
        run_nix_command(&["eval", &query, "--json"]).context("Failed to execute nix eval")?;

    if !success || stderr.contains("error") {
        bail!(
            "Package {} does not exist or is marked as insecure. Aborting",
            package
        );
    }

    Ok(())
}
