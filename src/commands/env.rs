use anyhow::{Context, Result, bail};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::flake::parser;

/// Add an environment variable to the dev shell
pub fn add(name: &str, value: &str) -> Result<()> {
    let flake_path = Path::new("flake.nix");

    if !flake_path.exists() {
        bail!(
            "No flake.nix found in current directory. Run {} first.",
            "flk init".yellow()
        );
    }

    // Validate variable name
    if name.trim().is_empty() {
        bail!("Environment variable name cannot be empty");
    }

    if !is_valid_env_var_name(name) {
        bail!(
            "Invalid environment variable name '{}'. Names should only contain letters, numbers, and underscores, and start with a letter or underscore.",
            name
        );
    }

    println!(
        "{} Adding environment variable: {} = {}",
        "→".blue().bold(),
        name.cyan(),
        value.green()
    );

    // Read the current flake.nix
    let flake_content = fs::read_to_string(flake_path).context("Failed to read flake.nix")?;

    // Check if variable already exists
    if parser::env_var_exists(&flake_content, name)? {
        bail!(
            "Environment variable '{}' already exists. Remove it first or use a different name.",
            name
        );
    }

    // Add the environment variable to shellHook
    let updated_content = parser::add_env_var(&flake_content, name, value)?;

    // Write back to file
    fs::write(flake_path, updated_content).context("Failed to write flake.nix")?;

    println!(
        "{} Environment variable '{}' added successfully!",
        "✓".green().bold(),
        name
    );
    println!("\n{}", "Next steps:".bold());
    println!("  1. Run {} to update your shell", "nix develop".cyan());
    println!("  2. The variable will be available as ${}", name.cyan());

    Ok(())
}

/// Remove an environment variable from the dev shell
pub fn remove(name: &str) -> Result<()> {
    let flake_path = Path::new("flake.nix");

    if !flake_path.exists() {
        bail!(
            "No flake.nix found in current directory. Run {} first.",
            "flk init".yellow()
        );
    }

    println!(
        "{} Removing environment variable: {}",
        "→".blue().bold(),
        name.cyan()
    );

    // Read the current flake.nix
    let flake_content = fs::read_to_string(flake_path).context("Failed to read flake.nix")?;

    // Check if variable exists
    if !parser::env_var_exists(&flake_content, name)? {
        bail!("Environment variable '{}' not found in flake.nix", name);
    }

    // Remove the environment variable
    let updated_content = parser::remove_env_var(&flake_content, name)?;

    // Write back to file
    fs::write(flake_path, updated_content).context("Failed to write flake.nix")?;

    println!(
        "{} Environment variable '{}' removed successfully!",
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

    // Read the current flake.nix
    let flake_content = fs::read_to_string(flake_path).context("Failed to read flake.nix")?;

    // Parse environment variables
    let env_vars = parser::parse_env_vars(&flake_content)?;

    if env_vars.is_empty() {
        println!("{}", "No environment variables found.".yellow());
        println!("\nAdd one with: {}", "flk env add <NAME> <VALUE>".cyan());
        return Ok(());
    }

    println!("{}", "═══════════════════════════════════════".cyan());
    println!(
        "{} {}",
        "Environment Variables".bold().cyan(),
        format!("({})", env_vars.len()).dimmed()
    );
    println!("{}", "═══════════════════════════════════════".cyan());
    println!();

    for (name, value) in env_vars {
        println!("  {} = {}", name.cyan().bold(), value.green());
    }

    println!();
    println!("{}", "═══════════════════════════════════════".cyan());

    Ok(())
}

/// Validate environment variable name
fn is_valid_env_var_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let first_char = name.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return false;
    }

    name.chars().all(|c| c.is_alphanumeric() || c == '_')
}
