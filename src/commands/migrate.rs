//! # Migrate Command Handler
//!
//! Convert a legacy flk layout (with `.flk/{default,overlays}.nix` and
//! `.flk/profiles/default.nix`) to the slim layout that uses
//! `flk.lib.mkProject` as a remote driver.

use anyhow::{bail, Context, Result};
use colored::Colorize;
use regex::Regex;
use std::fs;
use std::path::Path;

use flk::flake::generator;
use flk::flake::parsers::config as flk_config;

/// Migrate the current project to the slim layout.
///
/// Steps:
/// 1. Detect layout. If already slim, no-op with a message.
/// 2. Extract `defaultShell` from `.flk/default.nix` (if any).
/// 3. Write slim `flake.nix` + `.flk/config.nix`.
/// 4. Delete legacy driver files (`.flk/{default,overlays}.nix`,
///    `.flk/profiles/default.nix`).
/// 5. Preserve `.flk/pins.nix`, `.flk/profiles/<name>.nix`, `flake.lock`.
pub fn run() -> Result<()> {
    let flake_path = Path::new("flake.nix");
    let legacy_driver = Path::new(".flk/default.nix");
    let legacy_overlays = Path::new(".flk/overlays.nix");
    let legacy_importer = Path::new(".flk/profiles/default.nix");

    if !flake_path.exists() {
        bail!("No flake.nix in the current directory. Run 'flk init' first.");
    }

    if flk_config::exists() && !legacy_driver.exists() {
        println!(
            "{} Project is already on the slim layout — nothing to do.",
            "ℹ".blue()
        );
        return Ok(());
    }

    if !legacy_driver.exists() {
        bail!(
            ".flk/default.nix not found — cannot detect layout. \
             If this is a fresh project, run 'flk init' instead."
        );
    }

    // Extract defaultShell from legacy driver if present.
    let legacy_content =
        fs::read_to_string(legacy_driver).context("Failed to read .flk/default.nix")?;
    let default_profile = extract_default_shell(&legacy_content).unwrap_or_default();

    // Write new slim files.
    let slim_flake = generator::generate_slim_flake()?;
    let config_content = generator::generate_config(&default_profile)?;

    fs::write(flake_path, slim_flake).context("Failed to write slim flake.nix")?;
    fs::write(".flk/config.nix", config_content).context("Failed to write .flk/config.nix")?;

    // Remove legacy driver files. Tolerate already-missing ones.
    remove_if_exists(legacy_driver)?;
    remove_if_exists(legacy_overlays)?;
    remove_if_exists(legacy_importer)?;

    println!(
        "{} Migrated to the slim layout.\n  - wrote {}\n  - wrote {}\n  - removed {}\n  - removed {}\n  - removed {}",
        "✓".green().bold(),
        "flake.nix".cyan(),
        ".flk/config.nix".cyan(),
        ".flk/default.nix".yellow(),
        ".flk/overlays.nix".yellow(),
        ".flk/profiles/default.nix".yellow(),
    );

    if default_profile.is_empty() {
        println!(
            "{} No defaultShell was set in the legacy driver — run {} to pick one.",
            "ℹ".blue(),
            "flk profile set-default <name>".cyan()
        );
    } else {
        println!(
            "{} Preserved default profile: {}",
            "ℹ".blue(),
            default_profile.cyan()
        );
    }

    println!(
        "{} Run {} to refresh inputs against the remote driver.",
        "ℹ".blue(),
        "flk update".cyan()
    );

    Ok(())
}

fn extract_default_shell(content: &str) -> Option<String> {
    let re = Regex::new(r#"(?m)defaultShell\s*=\s*"([^"]*)"\s*;"#).unwrap();
    re.captures(content)
        .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
        .filter(|s| !s.trim().is_empty())
}

fn remove_if_exists(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_file(path).with_context(|| format!("Failed to remove {}", path.display()))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const LEGACY_DRIVER: &str = r#"inputs: let
  inherit (inputs) flake-utils nixpkgs profile-lib;
in
  flake-utils.lib.eachDefaultSystem (
    system: let
      defaultShell = "rust";
    in
      profileLib.mkProfileOutputs { profileDefinitions; maxCombinations = 3; }
  )
"#;

    #[test]
    fn extracts_default_shell() {
        assert_eq!(
            extract_default_shell(LEGACY_DRIVER),
            Some("rust".to_string())
        );
    }

    #[test]
    fn returns_none_when_default_shell_is_empty() {
        let driver = LEGACY_DRIVER.replace(r#""rust""#, r#""""#);
        assert_eq!(extract_default_shell(&driver), None);
    }

    #[test]
    fn returns_none_when_default_shell_absent() {
        assert_eq!(extract_default_shell("inputs: { }"), None);
    }
}
