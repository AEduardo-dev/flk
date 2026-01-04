use colored::Colorize;
use std::path::Path;

use anyhow::{Context, Result};

use flk::flake::parsers::{packages::parse_packages_section, utils::get_default_shell_profile};

pub fn run_list() -> Result<()> {
    let flake_path = Path::new(".flk/profiles/").join(format!(
        "{}.nix",
        get_default_shell_profile().context("Could not find default shell profile (flake.nix)")?
    ));

    let flake_content = std::fs::read_to_string(&flake_path).context("Failed to read flake.nix")?;
    let section = parse_packages_section(&flake_content)
        .context("Failed to parse packages section in flake.nix")?;
    let packages_info = section.to_packages();

    if packages_info.is_empty() {
        println!(
            "{} No packages found in the current profile.",
            "✗".red().bold()
        );
        return Ok(());
    }

    for pkg in packages_info {
        println!("{} {}", "•".green(), pkg.name);
    }

    Ok(())
}
