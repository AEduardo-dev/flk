use anyhow::{bail, Context, Result};
use colored::Colorize;
use flk::flake::generator::generate_flake;
use std::fs;
use std::path::Path;

use crate::flake::generator;

pub fn run(template: Option<String>, force: bool) -> Result<()> {
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

    // Generate flake.nix content
    let root_flake_content = generator::generate_root_flake()?;
    let helper_content = generator::generate_helper_module()?;
    let importer_content = generator::generate_importer_module()?;
    let profile_content = generator::generate_flake(&project_type)?;
    let hook_content = generator::generate_hooks();

    fs::create_dir_all(".flk/profiles")
        .context("Failed to create .flk and profiles directories")?;
    fs::write(".flk/hooks.sh", hook_content).context("Failed to write hook script")?;

    // Write to file
    fs::write(flake_path, root_flake_content).context("Failed to write flake.nix")?;
    fs::write(
        format!(".flk/profiles/{}.nix", project_type),
        profile_content,
    )
    .context("Failed to write flake.nix")?;
    fs::write(".flk/default.nix", helper_content).context("Failed to write flake.nix")?;
    fs::write(".flk/profiles/default.nix", importer_content)
        .context("Failed to write flake.nix")?;

    println!(
        "{} Created flk environment successfully!",
        "✓".green().bold()
    );
    println!("\n{}", "Next steps:".bold());
    println!("  1. Review and customize your profiles in .flk/profiles");
    println!("  2. Run {} to add packages", "flk add <package>".cyan());
    println!("  3. Run {} to enter the dev shell", "flk activate".cyan());
    println!(
        " or create {} profiles with {}",
        "more".purple(),
        "flk profile add".cyan()
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
