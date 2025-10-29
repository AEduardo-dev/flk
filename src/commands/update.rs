// src/commands/update.rs
use anyhow::{Context, Result};
use colored::Colorize;
use serde_json::Value;
use std::fs;

use crate::nix::run_nix_command;
use crate::utils::backup;
use crate::utils::visual::with_spinner;

pub fn run_update(packages: Vec<String>, show: bool) -> Result<()> {
    if !packages.is_empty() {
        anyhow::bail!(
            "Updating specific packages requires version pinning (see issue #7). Use 'flk update' to update all packages."
        );
    }

    if show {
        show_update_preview()?;
    } else {
        perform_update()?;
    }

    Ok(())
}

/// Show what would be updated without actually updating
fn show_update_preview() -> Result<()> {
    println!("{}", "Checking for updates...".bold().cyan());
    println!();

    // Check if flake.lock exists
    if !std::path::Path::new("flake.lock").exists() {
        anyhow::bail!("flake.lock not found. Run 'nix flake lock' first.");
    }

    // Get current lock file
    let current_lock = read_lock_file()?;

    // Create a temporary backup
    fs::copy("flake.lock", "flake.lock.tmp")?;

    // Run the update
    let (_, stderr, success) =
        run_nix_command(&["flake", "update"]).context("Failed to check for updates")?;

    if !success {
        // Restore from temp backup if update failed
        fs::rename("flake.lock.tmp", "flake.lock")?;
        anyhow::bail!("Failed to check for updates: {}", stderr);
    }

    // Get updated lock file
    let updated_lock = read_lock_file()?;

    // Restore the original lock file since this is just a preview
    fs::rename("flake.lock.tmp", "flake.lock")?;

    // Compare and display differences
    display_update_diff(&current_lock, &updated_lock)?;

    println!();
    println!(
        "{}",
        "No changes were made. Run 'flk update' to apply these updates.".dimmed()
    );

    Ok(())
}

/// Perform the actual update
fn perform_update() -> Result<()> {
    println!("{}", "Updating flake inputs...".bold().cyan());

    // Ensure .flk directory exists
    backup::ensure_flk_dir()?;

    // Create a backup of the current lock file BEFORE updating
    if std::path::Path::new("flake.lock").exists() {
        let backup_path = backup::create_backup(std::path::Path::new("flake.lock"))?;
        println!(
            "{} Created backup: {}",
            "→".blue().bold(),
            backup_path.file_name().unwrap().to_string_lossy().dimmed()
        );
    }

    // Run the update
    let (stdout, stderr, success) = with_spinner("Updating flake...", || {
        run_nix_command(&["flake", "update"]).context("Failed to execute nix flake update")
    })?;

    if !success {
        anyhow::bail!("Failed to update flake: {}", stderr);
    }

    if !stdout.trim().is_empty() {
        println!("{}", stdout);
    }

    println!("{}", "✓ Flake updated successfully!".green().bold());
    println!("\n{}", "Next steps:".bold());
    println!(
        "  • Run {} to see the updated configuration",
        "flk show".cyan()
    );
    println!(
        "  • Run {} to see lock file details",
        "flk lock show".cyan()
    );
    println!(
        "  • Run {} if you need to rollback",
        "flk lock restore latest".cyan()
    );

    Ok(())
}

/// Read and parse the flake.lock file
fn read_lock_file() -> Result<Value> {
    let lock_content = fs::read_to_string("flake.lock").context("Failed to read flake.lock")?;

    let lock_data: Value =
        serde_json::from_str(&lock_content).context("Failed to parse flake.lock")?;

    Ok(lock_data)
}

/// Display the differences between current and updated lock files
fn display_update_diff(current: &Value, updated: &Value) -> Result<()> {
    println!("{}", "═══════════════════════════════════════".cyan());
    println!("{}", "Update Preview".bold().cyan());
    println!("{}", "═══════════════════════════════════════".cyan());
    println!();

    let current_nodes = &current["nodes"];
    let updated_nodes = &updated["nodes"];

    if let (Some(current_obj), Some(updated_obj)) =
        (current_nodes.as_object(), updated_nodes.as_object())
    {
        let mut changes_found = false;

        for (input_name, _) in current_obj.iter() {
            // Skip root and other non-input nodes
            if input_name == "root" {
                continue;
            }

            let current_info = &current_obj[input_name]["locked"];
            let updated_info = &updated_obj[input_name]["locked"];

            // Only show if there's an actual change
            if current_info != updated_info && !current_info.is_null() && !updated_info.is_null() {
                changes_found = true;
                display_input_change(input_name, current_info, updated_info);
            }
        }

        if !changes_found {
            println!(
                "{}",
                "  No updates available. All inputs are up to date! ✓".green()
            );
        }
    } else {
        println!("{}", "  Unable to compare lock files".yellow());
    }

    println!();
    println!("{}", "═══════════════════════════════════════".cyan());

    Ok(())
}

/// Display changes for a single input
fn display_input_change(name: &str, current: &Value, updated: &Value) {
    println!("{} {}", "Input:".bold(), name.cyan());

    // Show type if available
    if let Some(input_type) = current["type"].as_str() {
        println!("  {} {}", "Type:".dimmed(), input_type);
    }

    // Show revision changes if available
    if let (Some(current_rev), Some(updated_rev)) =
        (current["rev"].as_str(), updated["rev"].as_str())
    {
        if current_rev != updated_rev {
            // Show shortened commit hashes (first 12 chars)
            let current_short = if current_rev.len() >= 12 {
                &current_rev[..12]
            } else {
                current_rev
            };
            let updated_short = if updated_rev.len() >= 12 {
                &updated_rev[..12]
            } else {
                updated_rev
            };

            println!("  {} {}", "From:".dimmed(), current_short.yellow());
            println!("  {} {}", "To:  ".dimmed(), updated_short.green());
        }
    }

    // Show lastModified changes if available
    if let (Some(current_modified), Some(updated_modified)) = (
        current["lastModified"].as_i64(),
        updated["lastModified"].as_i64(),
    ) {
        if current_modified != updated_modified {
            println!("  {} {}", "Last Modified:".dimmed(), "updated".green());
        }
    }

    // Show narHash changes if available
    if let (Some(current_hash), Some(updated_hash)) =
        (current["narHash"].as_str(), updated["narHash"].as_str())
    {
        if current_hash != updated_hash {
            println!("  {} {}", "Content:".dimmed(), "changed ✓".green());
        }
    }

    println!();
}
