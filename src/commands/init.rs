//! # Initialize Command Handler
//!
//! Initialize a new flk-managed development environment with project type detection.

use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

use flk::flake::generator;

/// Initialize a new flake environment in the current directory.
///
/// # Arguments
///
/// * `template` - Optional project type (rust, python, node, go, generic)
/// * `force` - If true, overwrite existing flake.nix
/// * `legacy` - If true, materialize the legacy in-repo driver instead of the slim layout
pub fn run(template: Option<String>, force: bool, legacy: bool) -> Result<()> {
    let flake_path = Path::new("flake.nix");

    // Check if flake.nix already exists
    if flake_path.exists() && !force {
        bail!(
            "flake.nix already exists! Use {} to overwrite.",
            "--force".yellow()
        );
    }

    // Detect project type if not specified
    let project_type = template.unwrap_or_else(detect_project_type);

    println!(
        "{} Initializing flake for {} project...",
        "→".blue().bold(),
        project_type.green()
    );

    let profile_content = generator::generate_flake(&project_type)?;
    let pins_content = generator::generate_pins()?;

    fs::create_dir_all(".flk/profiles")
        .context("Failed to create .flk and profiles directories")?;

    fs::write(
        format!(".flk/profiles/{}.nix", project_type),
        profile_content,
    )
    .context("Failed to write profile file")?;
    fs::write(".flk/pins.nix", pins_content).context("Failed to write pins.nix")?;

    if legacy {
        let root_flake_content = generator::generate_root_flake()?;
        let helper_content = generator::generate_helper_module()?;
        let importer_content = generator::generate_importer_module()?;
        let overlays_content = generator::generate_overlays()?;

        fs::write(flake_path, root_flake_content).context("Failed to write flake.nix")?;
        fs::write(".flk/default.nix", helper_content).context("Failed to write helper nix file")?;
        fs::write(".flk/overlays.nix", overlays_content).context("Failed to write overlays.nix")?;
        fs::write(".flk/profiles/default.nix", importer_content)
            .context("Failed to write importer nix file")?;
    } else {
        let slim_flake_content = generator::generate_slim_flake()?;
        let config_content = generator::generate_config(&project_type)?;

        fs::write(flake_path, slim_flake_content).context("Failed to write flake.nix")?;
        fs::write(".flk/config.nix", config_content).context("Failed to write .flk/config.nix")?;
    }

    println!(
        "{} Created flk environment successfully!",
        "✓".green().bold()
    );
    // Add message for adding `flk hook <shell>` to shell config
    println!(
        "\n{} To enable shell integration, add the hook to your shell configuration:",
        "ℹ".blue()
    );
    println!(
        "   - Add {} to your shell config file (e.g., .bashrc, .zshrc): {}",
        "flk hook <shell>".cyan(),
        "eval \"$(flk hook <shell>)\"".yellow()
    );

    Ok(())
}

fn detect_project_type() -> String {
    // Check for common project files
    if Path::new("Cargo.toml").exists() {
        println!("{} Detected Rust project", "ℹ".blue());
        return "rust".to_string();
    }
    if Path::new("package.json").exists() {
        println!("{} Detected Node.js project", "ℹ".blue());
        return "node".to_string();
    }
    if Path::new("pyproject.toml").exists() || Path::new("requirements.txt").exists() {
        println!("{} Detected Python project", "ℹ".blue());
        return "python".to_string();
    }
    if Path::new("go.mod").exists() {
        println!("{} Detected Go project", "ℹ".blue());
        return "go".to_string();
    }

    println!(
        "{} No specific project type detected, using generic template",
        "ℹ".blue()
    );
    "generic".to_string()
}
