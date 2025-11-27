use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::flake::parsers::{
    commands::{add_command_to_shell_hook, command_exists, remove_command_from_shell_hook},
    flake::parse_flake,
    utils::get_default_shell_profile,
};

pub fn run_add(name: &str, command: &str, file: Option<String>) -> Result<()> {
    let flake_path = Path::new(".flk/profiles/").join(format!(
        "{}.nix",
        get_default_shell_profile().context("Could not find default shell profile (flake.nix)")?
    ));

    // Validate command name
    if !is_valid_command_name(name) {
        bail!(
            "Invalid command name '{}'. Use only letters, numbers, hyphens, and underscores.",
            name
        );
    }

    println!("{} Adding command: {}", "→".blue().bold(), name.green());

    let command_content = if let Some(filepath) = file {
        println!("  Sourcing from: {}", filepath.cyan());
        fs::read_to_string(&filepath)
            .with_context(|| format!("Failed to read file: {}", filepath))?
    } else {
        command.to_string()
    };

    if command_content.trim().is_empty() {
        bail!("Command cannot be empty");
    }

    // Read the current flake.nix
    let flake_content = fs::read_to_string(&flake_path).context("Failed to read flake.nix")?;

    // Check if command already exists
    if command_exists(&flake_content, name) {
        bail!(
            "Command '{}' already exists. Remove it with: {}",
            name.cyan(),
            format!("flk remove-command {}", name).yellow()
        );
    }

    // Add the command to shellHook
    let updated_content = add_command_to_shell_hook(&flake_content, name, &command_content, None)?;

    // Write back to file
    fs::write(flake_path, updated_content).context("Failed to write flake.nix")?;

    println!(
        "{} Command '{}' added successfully!",
        "✓".green().bold(),
        name
    );
    println!("\n{}", "Next steps:".bold());
    println!("  1. Run {} to enter the dev shell", "nix develop".cyan());
    println!("  2. Use your command: {}", name.cyan());

    Ok(())
}

pub fn run_remove(name: &str) -> Result<()> {
    let flake_path = Path::new(".flk/profiles/").join(format!(
        "{}.nix",
        get_default_shell_profile().context("Could not find default shell profile (flake.nix)")?
    ));

    if !flake_path.exists() {
        bail!("No flake.nix found.");
    }

    println!("{} Removing command: {}", "→".blue().bold(), name.yellow());

    // Read the current flake.nix
    let flake_content = fs::read_to_string(&flake_path).context("Failed to read flake.nix")?;

    // Check if command exists
    if !command_exists(&flake_content, name) {
        bail!("Command '{}' not found in flake.nix", name.cyan());
    }

    // Remove the command from shellHook
    let updated_content = remove_command_from_shell_hook(&flake_content, name, None)?;

    // Write back to file
    fs::write(&flake_path, updated_content).context("Failed to write flake.nix")?;

    println!(
        "{} Command '{}' removed successfully!",
        "✓".green().bold(),
        name
    );

    Ok(())
}

/// List all environment variables in the dev shell
pub fn list() -> Result<()> {
    let flake_path = Path::new("flake.nix");

    if !flake_path.exists() {
        bail!(
            "No flake.nix found in current directory. Run {} first.",
            "flk init".yellow()
        );
    }

    let flake_info = parse_flake(flake_path.to_str().unwrap())?;

    flake_info.display_shell_hooks();

    Ok(())
}

fn is_valid_command_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        && !name.starts_with('-')
}
