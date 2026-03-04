//! # Custom Command Handler
//!
//! Add, remove, and list custom shell commands in the development environment.
//!
//! Custom commands become available as shell functions when the dev shell
//! is activated.

use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

use flk::flake::parsers::{
    commands::{add_shell_hook_command, parse_shell_hook_section, remove_shell_hook_command},
    utils::resolve_profile,
};

/// Add a custom shell command to the development environment.
///
/// The command becomes available as a shell function when the dev shell
/// is activated. Can accept inline command text or read from a file.
///
/// # Arguments
///
/// * `name` - Command name (alphanumeric, hyphens, underscores)
/// * `command` - Command body (bash code)
/// * `file` - Optional path to read command body from a file instead
/// * `target_profile` - Optional profile override
pub fn run_add(
    name: &str,
    command: &str,
    file: Option<String>,
    target_profile: Option<String>,
) -> Result<()> {
    let profile = resolve_profile(target_profile)?;
    let flake_path = Path::new(".flk/profiles/").join(format!("{}.nix", profile));

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
    let section = parse_shell_hook_section(&flake_content)
        .context("Failed to parse shellHook section in flake.nix")?;

    // Check if command already exists
    if section.command_exists(name) {
        bail!(
            "Command '{}' already exists. Remove it with: {}",
            name.cyan(),
            format!("flk remove-command {}", name).yellow()
        );
    }

    // Add the command to shellHook
    let updated_content = add_shell_hook_command(&flake_content, name, &command_content)
        .context("Failed to add command to shellHook")?;

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

/// Remove a custom shell command from the development environment.
///
/// # Arguments
///
/// * `name` - Name of the command to remove
/// * `target_profile` - Optional profile override
pub fn run_remove(name: &str, target_profile: Option<String>) -> Result<()> {
    let profile = resolve_profile(target_profile)?;
    let flake_path = Path::new(".flk/profiles/").join(format!("{}.nix", profile));

    if !flake_path.exists() {
        bail!("No flake.nix found.");
    }

    println!("{} Removing command: {}", "→".blue().bold(), name.yellow());

    // Read the current flake.nix
    let flake_content = fs::read_to_string(&flake_path).context("Failed to read flake.nix")?;
    let section = parse_shell_hook_section(&flake_content)
        .context("Failed to parse shellHook section in flake.nix")?;

    // Check if command exists
    if !section.command_exists(name) {
        bail!("Command '{}' not found in flake.nix", name.cyan());
    }

    // Remove the command from shellHook
    let updated_content = remove_shell_hook_command(&flake_content, name)
        .context("Failed to remove command from shellHook")?;

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
pub fn list(target_profile: Option<String>) -> Result<()> {
    let profile = resolve_profile(target_profile)?;
    let flake_path = Path::new(".flk/profiles/").join(format!("{}.nix", profile));
    let flake_content = fs::read_to_string(&flake_path).context("Failed to read flake.nix")?;
    let section = parse_shell_hook_section(&flake_content)
        .context("Failed to parse shellHook section in flake.nix")?;

    if section.entries.is_empty() {
        println!(
            "{} No commands found in the current profile.",
            "✗".red().bold()
        );
        return Ok(());
    }

    for entry in section.entries {
        println!("{} {}", "•".green(), entry.name.bold());
    }

    Ok(())
}

fn is_valid_command_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        && !name.starts_with('-')
}
