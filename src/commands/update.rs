// src/commands/update.rs
use anyhow::Result;
use colored::Colorize;
use std::process::Command;

pub fn run_update(packages: Vec<String>) -> Result<()> {
    if !packages.is_empty() {
        // For now, error on specific packages
        anyhow::bail!(
            "Updating specific packages requires version pinning (see issue #7). Use 'flk update' to update all packages."
        );
    }

    println!("{}", "Updating flake inputs...".bold().cyan());

    // Run nix flake update
    let output = Command::new("nix").args(&["flake", "update"]).output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to update flake: {}", error);
    }

    println!("{}", "✓ Flake updated successfully!".green());
    println!("\nRun {} to see what changed.", "flk show".bold());

    Ok(())
}
