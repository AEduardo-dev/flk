use anyhow::{Context, Result, bail};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::flake::parser;

pub fn run(name: &str, command: &str, file: Option<String>) -> Result<()> {
    let flake_path = Path::new("flake.nix");

    if !flake_path.exists() {
        bail!("No flake.nix found. Run {} first.", "flk init".yellow());
    }

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
    let flake_content = fs::read_to_string(flake_path).context("Failed to read flake.nix")?;

    // Check if command already exists
    if parser::command_exists(&flake_content, name) {
        bail!(
            "Command '{}' already exists. Remove it with: {}",
            name,
            format!("flk remove-command {}", name).cyan()
        );
    }

    // Add the command to shellHook
    let updated_content =
        parser::add_command_to_shell_hook(&flake_content, name, &command_content)?;

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

fn is_valid_command_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        && !name.starts_with('-')
}
