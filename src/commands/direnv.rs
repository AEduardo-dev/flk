//! # Direnv Integration Handler
//!
//! Manage direnv integration for automatic environment activation.

use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

const DIRENV_FLK_DIRECTIVE: &str = r#"# Watch flk config files so nix-direnv re-evaluates on changes
watch_file .flk/default.nix
watch_file .flk/pins.nix
watch_file .flk/overlays.nix
for f in .flk/profiles/*.nix; do watch_file "$f"; done

use flake "${FLK_PROFILE:-.#}" --impure"#;

/// Create a new `.envrc` file with the flk directive.
pub fn direnv_init() -> Result<()> {
    let direnv_path = Path::new(".envrc");

    // Check if .envrc already exists
    if direnv_path.exists() {
        bail!(".envrc already exists! Please back it up before proceeding.");
    }

    fs::write(direnv_path, DIRENV_FLK_DIRECTIVE).context("Failed to write .envrc")?;

    println!(
        "{} Created .envrc for direnv successfully!",
        "✓".green().bold()
    );
    println!("\n{}", "Next steps:".bold());
    println!("  1. Review the .envrc file");
    println!("  2. Run {} to allow direnv", "direnv allow".cyan());
    println!("  3. Enjoy your development environment with direnv and flk!");

    Ok(())
}

/// Append the flk directive to an existing `.envrc` file.
pub fn direnv_attach() -> Result<()> {
    let direnv_path = Path::new(".envrc");

    // Check if .envrc exists
    if !direnv_path.exists() {
        bail!(".envrc does not exist! Please run 'flk direnv-init' instead.");
    }

    let mut direnv_content = fs::read_to_string(direnv_path).context("Failed to read .envrc")?;

    // Check if use flake directive already exists
    if direnv_content.contains("use flake") {
        bail!("The .envrc already contains the use flake directive!");
    }

    direnv_content.push('\n');
    direnv_content.push_str(DIRENV_FLK_DIRECTIVE);
    fs::write(direnv_path, direnv_content).context("Failed to update .envrc")?;

    println!(
        "{} Updated .envrc for direnv successfully!",
        "✓".green().bold()
    );
    println!("\n{}", "Next steps:".bold());
    println!("  1. Review the updated .envrc file");
    println!("  2. Run {} to reload direnv", "direnv reload".cyan());
    println!("  3. Enjoy your development environment with direnv and flk!");

    Ok(())
}

/// Remove the flk directive from an existing `.envrc` file.
pub fn direnv_detach() -> Result<()> {
    let direnv_path = Path::new(".envrc");
    // Check if .envrc exists
    if !direnv_path.exists() {
        bail!(".envrc does not exist!");
    }
    let direnv_content = fs::read_to_string(direnv_path).context("Failed to read .envrc")?;
    // Remove flk directives (use flake and watch_file for .flk/)
    let updated_content: String = direnv_content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed != r#"use flake "${FLK_PROFILE:-.#}" --impure"#
                && !trimmed.starts_with("watch_file .flk/")
                && !trimmed.starts_with("for f in .flk/profiles")
                && trimmed != "# Watch flk config files so nix-direnv re-evaluates on changes"
        })
        .map(|line| format!("{}\n", line))
        .collect();
    fs::write(direnv_path, updated_content).context("Failed to update .envrc")?;
    println!(
        "{} Removed use flake directive from .envrc successfully!",
        "✓".green().bold()
    );
    Ok(())
}
