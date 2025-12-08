use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::{fs, path};

use crate::flake::parsers::{
    overlays::add_pinned_package,
    packages::{add_package_to_profile, package_exists},
    utils::get_default_shell_profile,
};
use crate::nix::run_nix_command;
use crate::utils::visual::with_spinner;

pub fn run_add(package: &str, version: Option<String>) -> Result<()> {
    let flake_path = path::Path::new(".flk/profiles/").join(format!(
        "{}.nix",
        get_default_shell_profile().context("Could not find default shell profile")?
    ));
    let flake_content = fs::read_to_string(&flake_path)?;

    if package.trim().is_empty() {
        bail!("Package name cannot be empty");
    }

    with_spinner("Validating package...", || validate_package_exists(package))?;

    let (package_to_add, package_pin) = if let Some(ver) = &version {
        println!(
            "{} Adding package: {} (pinned to version {})",
            "→".blue().bold(),
            package.green(),
            ver.yellow()
        );

        let full_pin = with_spinner("Fetching nixpkgs pin for the package...", || {
            get_nix_package_pin_full(package, ver)
        })?;

        (format!("{}@{}", package, ver), Some(full_pin))
    } else {
        println!("{} Adding package: {}", "→".blue().bold(), package.green());
        (package.to_string(), None)
    };

    // Check if package already exists in profile
    if package_exists(&flake_content, &package_to_add, None)? {
        bail!(
            "Package '{}' is already in the packages declaration",
            package_to_add
        );
    }

    // If pinned, add to pins.nix
    if let Some(pin_info) = package_pin {
        let pins_content = fs::read_to_string(".flk/pins.nix")?;

        let updated_pins = add_pinned_package(
            &pins_content,
            &pin_info.hash,
            &pin_info.full_ref,
            package,
            version.as_ref().unwrap(),
        )?;

        fs::write(".flk/pins.nix", updated_pins)?;
    }

    // Add to profile
    let updated_content = add_package_to_profile(&flake_content, &package_to_add, None)?;
    fs::write(flake_path, updated_content)?;

    println!(
        "{} Package '{}' added successfully!",
        "✓".green().bold(),
        package_to_add
    );
    Ok(())
}

struct PinInfo {
    hash: String,
    full_ref: String,
}

fn get_nix_package_pin_full(package: &str, version: &str) -> Result<PinInfo> {
    let package_with_version = format!("{}@{}", package, version);
    let (stdout, stderr, success) = run_nix_command(&[
        "run",
        "github:vic/nix-versions",
        &package_with_version,
        "--",
        "--one",
    ])
    .context("Failed to execute nix eval")?;

    if !success || stderr.contains("no packages found") {
        bail!(
            "Package {} does not exist or is marked as insecure. Aborting",
            package
        );
    }

    // extract the nixpkgs pin from the stdout
    let stdout = stdout
        .lines()
        .find(|line| line.contains("nixpkgs"))
        .context("Failed to find nixpkgs pin in nix search output")?;

    // get the pin value
    let pin = stdout
        .split_whitespace()
        .nth(2)
        .context("Failed to extract nixpkgs pin value")
        .map(|s| s.trim().to_string())?;

    let pin_hash = pin
        .split('/')
        .nth(1)
        .and_then(|s| s.split('#').next())
        .context("Failed to extract nixpkgs pin from output")?
        .trim()
        .to_string();

    let pin_ref = format!("github:NixOS/nixpkgs/{}", pin_hash);

    Ok(PinInfo {
        hash: pin_hash,
        full_ref: pin_ref,
    })
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
