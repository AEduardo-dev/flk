use anyhow::{Context, Result, bail};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::flake::parser;

pub fn run(name: &str) -> Result<()> {
    let flake_path = Path::new("flake.nix");

    if !flake_path.exists() {
        bail!("No flake.nix found.");
    }

    println!("{} Removing command: {}", "→".blue().bold(), name.yellow());

    // Read the current flake.nix
    let flake_content = fs::read_to_string(flake_path).context("Failed to read flake.nix")?;

    // Check if command exists
    if !parser::command_exists(&flake_content, name) {
        bail!("Command '{}' not found in flake.nix", name);
    }

    // Remove the command from shellHook
    let updated_content = parser::remove_command_from_shell_hook(&flake_content, name)?;

    // Write back to file
    fs::write(flake_path, updated_content).context("Failed to write flake.nix")?;

    println!(
        "{} Command '{}' removed successfully!",
        "✓".green().bold(),
        name
    );

    Ok(())
}
