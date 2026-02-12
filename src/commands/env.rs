use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

use flk::flake::parsers::{env::parse_env_vars_section, utils::resolve_profile};

/// Add an environment variable to the dev shell
pub fn add(name: &str, value: &str, target_profile: Option<String>) -> Result<()> {
    let profile_to_parse = resolve_profile(target_profile)?;
    let flake_path = Path::new(".flk/profiles/").join(format!("{}.nix", profile_to_parse));

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
    let flake_content = fs::read_to_string(&flake_path).context("Failed to read flake.nix")?;
    let section = parse_env_vars_section(&flake_content)?;

    if section.env_var_exists(name)? {
        bail!(
            "Environment variable '{}' already exists in profile '{}'",
            name.cyan(),
            profile_to_parse.yellow()
        );
    }

    // Add the environment variable to shellHook
    let updated_content = section.add_env_var(&flake_content, name, value);

    // Write back to file
    fs::write(flake_path, updated_content).context("Failed to write flake.nix")?;

    println!("\n{}", "Next steps:".bold());
    println!("  1. Run {} to update your shell", "nix develop".cyan());
    println!("  2. The variable will be available as ${}", name.cyan());

    Ok(())
}

/// Remove an environment variable from the dev shell
pub fn remove(name: &str, target_profile: Option<String>) -> Result<()> {
    let profile_to_parse = resolve_profile(target_profile)?;
    let flake_path = Path::new(".flk/profiles/").join(format!("{}.nix", profile_to_parse));

    println!(
        "{} Removing environment variable: {}",
        "→".blue().bold(),
        name.cyan()
    );

    // Read the current flake.nix
    let flake_content = fs::read_to_string(&flake_path).context("Failed to read flake.nix")?;
    let section = parse_env_vars_section(&flake_content)?;

    if !section.env_var_exists(name)? {
        bail!(
            "Environment variable '{}' does not exist in profile '{}'",
            name.cyan(),
            profile_to_parse.yellow()
        );
    }
    // Remove the environment variable
    let updated_content = section.remove_env_var(&flake_content, name)?;

    // Write back to file
    fs::write(flake_path, updated_content).context("Failed to write flake.nix")?;

    Ok(())
}

/// List all environment variables in the dev shell
pub fn list(target_profile: Option<String>) -> Result<()> {
    let profile = resolve_profile(target_profile)?;
    let flake_path = Path::new(".flk/profiles/").join(format!("{}.nix", profile));
    let flake_content = fs::read_to_string(&flake_path).context("Failed to read flake.nix")?;
    let section = parse_env_vars_section(&flake_content)?;
    let env_vars = section.to_env_vars();

    if env_vars.is_empty() {
        println!(
            "{} No environment variables found in the current profile.",
            "✗".red().bold()
        );
        return Ok(());
    }

    for env_var in env_vars {
        println!("{} {}", "•".green(), env_var);
    }

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
