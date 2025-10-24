use anyhow::{Context, Result, bail};
use colored::Colorize;
use std::process::Command;

pub async fn run_search(query: &str, limit: usize) -> Result<()> {
    println!(
        "{} Searching nixpkgs for: {}",
        "→".blue().bold(),
        query.green()
    );
    let search_query = format!("^{}", query);

    // Use nix search command with JSON output
    let output = Command::new("nix")
        .args(&["search", "nixpkgs", &search_query, "--json"])
        .output()
        .context("Failed to execute nix search. Is nix installed?")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        bail!("Nix search failed: {}", error);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    if stdout.trim().is_empty() || stdout.trim() == "{}" {
        println!(
            "{} No packages found matching '{}'",
            "✗".red().bold(),
            query
        );
        println!("\n{} Try:", "💡".bold());
        println!("  • Using different search terms");
        println!("  • Searching on https://search.nixos.org");
        return Ok(());
    }

    let results: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(&stdout).context("Failed to parse nix search output")?;

    let mut results_vec: Vec<_> = results.iter().collect();
    results_vec.truncate(limit);

    if results_vec.is_empty() {
        println!(
            "{} No packages found matching '{}'",
            "✗".red().bold(),
            query
        );
        return Ok(());
    }

    println!(
        "\n{} Found {} package(s) (showing {}):\n",
        "✓".green().bold(),
        results.len(),
        results_vec.len()
    );

    for (i, (attr_name, info)) in results_vec.iter().enumerate() {
        println!(
            "{} {}",
            format!("[{}]", i + 1).cyan().bold(),
            attr_name.green()
        );

        if let Some(version) = info.get("version").and_then(|v| v.as_str()) {
            println!("  {} {}", "Version:".bold(), version.yellow());
        }

        if let Some(desc) = info.get("description").and_then(|v| v.as_str()) {
            let trimmed = desc.trim();
            if !trimmed.is_empty() {
                let display_desc = if trimmed.len() > 80 {
                    format!("{}...", &trimmed[..77])
                } else {
                    trimmed.to_string()
                };
                println!("  {} {}", "Description:".bold(), display_desc);
            }
        }

        println!();
    }

    if results.len() > limit {
        println!(
            "{} Showing {} of {} results. Refine your search for fewer results.",
            "ℹ".blue(),
            limit,
            results.len()
        );
    }

    println!(
        "\n{} Add a package with: {}",
        "💡".bold(),
        format!("flk add <package-name>").cyan()
    );

    Ok(())
}

pub async fn run_deep_search(package: &str, show_versions: bool) -> Result<()> {
    println!(
        "{} Getting details for: {}",
        "→".blue().bold(),
        package.green()
    );

    let output = Command::new("nix")
        .args(&["search", "nixpkgs", package, "--json"])
        .output()
        .context("Failed to execute nix search. Is nix installed?")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        bail!("Nix search failed: {}", error);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let results: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(&stdout).context("Failed to parse nix search output")?;

    if results.is_empty() {
        println!("{} Package '{}' not found", "✗".red().bold(), package);
        println!(
            "\n{} Try searching first: {}",
            "💡".bold(),
            format!("flk search {}", package).cyan()
        );
        return Ok(());
    }

    // Find exact match or use first result
    let (attr_name, info) = results
        .iter()
        .find(|(name, _)| name.contains(package))
        .or_else(|| results.iter().next())
        .unwrap();

    println!("\n{}", "Package Details:".bold().underline());
    println!("{} {}", "Name:".bold(), attr_name.green());

    if let Some(version) = info.get("version").and_then(|v| v.as_str()) {
        println!(
            "{} {} (latest in nixpkgs-unstable)",
            "Version:".bold(),
            version.yellow()
        );
    }

    if let Some(desc) = info.get("description").and_then(|v| v.as_str()) {
        println!("{} {}", "Description:".bold(), desc);
    }

    if show_versions {
        let pkg_name = attr_name.split('.').last().unwrap_or(package);

        println!("\n{}", "Version Pinning:".bold());
        println!(
            "{} To pin to a specific version, search for it at:",
            "ℹ".blue()
        );
        println!(
            "  {}",
            format!("https://lazamar.co.uk/nix-versions/?package={}", pkg_name)
                .cyan()
                .underline()
        );
        println!(
            "  {}",
            format!("https://www.nixhub.io/packages/{}", pkg_name)
                .cyan()
                .underline()
        );
    }

    Ok(())
}
