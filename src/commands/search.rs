use anyhow::{bail, Context, Result};
use colored::Colorize;

use crate::flake::interface::Package;
use crate::flake::parsers::packages::extract_packages_from_output;
use crate::nix::{check_nix_available, run_nix_command};
use crate::utils::visual::{display_list, display_table, with_spinner};

pub fn run_search(query: &str, limit: usize) -> Result<bool> {
    println!(
        "{} Searching nixpkgs for: {}",
        "→".blue().bold(),
        query.green()
    );
    let search_query = format!("*{}*", query);

    if !check_nix_available() {
        bail!("Nix command is not available, is it installed on the system?");
    }

    // Use nix search command with JSON output
    let (stdout, _, _) = with_spinner("Searching packages...", || {
        run_nix_command(&[
            "run",
            "github:vic/nix-versions",
            &search_query,
            "--",
            "--one",
        ])
        .context("Failed to execute nix search. Is nix installed?")
    })?;

    let packages: Vec<Package> =
        extract_packages_from_output(&stdout).context("Failed to parse nix search output")?;

    if packages.is_empty() {
        println!(
            "{} No packages found for query '{}'",
            "✗".red().bold(),
            query
        );
        return Ok(false);
    }

    println!(
        "\n{} Found {} package(s) (showing {}):\n",
        "✓".green().bold(),
        packages.len(),
        packages.len()
    );

    println!("{}", display_list(&packages[..packages.len().min(limit)]));

    Ok(true)
}

pub fn run_deep_search(package: &str) -> Result<()> {
    println!(
        "{} Getting details for: {}",
        "→".blue().bold(),
        package.green()
    );

    if !check_nix_available() {
        bail!("Nix is not available!")
    }
    let (stdout, _, _) = with_spinner("Searching packages...", || {
        run_nix_command(&["run", "github:vic/nix-versions", package, "--", "--all"])
            .context("Failed to execute nix search. Is nix installed?")
    })?;

    let packages: Vec<Package> =
        extract_packages_from_output(&stdout).context("Failed to parse nix search output")?;
    if packages.is_empty() {
        println!("{} No packages found for '{}'", "✗".red().bold(), package);
        return Ok(());
    }

    println!("\n{}", "Search Results:".bold().underline());
    println!("{}", display_table(&packages));

    Ok(())
}
