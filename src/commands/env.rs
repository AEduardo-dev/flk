use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::flake::parsers::{
    env::parse_env_vars_section, flake::parse_flake, utils::get_default_shell_profile,
};

/// Add an environment variable to the dev shell
pub fn add(name: &str, value: &str) -> Result<()> {
    let flake_path = Path::new(".flk/profiles/").join(format!(
        "{}.nix",
        get_default_shell_profile().context("Could not find default shell profile (flake.nix)")?
    ));
    let profile_to_parse =
        get_default_shell_profile().context("Could not find default shell profile (flake.nix)")?;

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
pub fn remove(name: &str) -> Result<()> {
    let flake_path = Path::new(".flk/profiles/").join(format!(
        "{}.nix",
        get_default_shell_profile().context("Could not find default shell profile (flake.nix)")?
    ));
    let profile_to_parse =
        get_default_shell_profile().context("Could not find default shell profile (flake.nix)")?;

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
pub fn list() -> Result<()> {
    let flake_path = Path::new("flake.nix");

    if !flake_path.exists() {
        bail!(
            "No flake.nix found in current directory. Run {} first.",
            "flk init".yellow()
        );
    }

    let flake_info = parse_flake(flake_path.to_str().unwrap())?;

    flake_info.display_env_vars();

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
