use anyhow::{Context, Result, bail};
use colored::Colorize;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

use crate::utils::backup::{create_backup, get_backup_dir};

/// Show detailed lock file information
pub fn show() -> Result<()> {
    let lock_path = Path::new("flake.lock");

    if !lock_path.exists() {
        bail!(
            "No flake.lock found in current directory. Run {} first.",
            "nix flake lock".yellow()
        );
    }

    // Read and parse the lock file
    let lock_content = fs::read_to_string(lock_path).context("Failed to read flake.lock")?;
    let lock_data: Value =
        serde_json::from_str(&lock_content).context("Failed to parse flake.lock")?;

    println!("{}", "═══════════════════════════════════════".cyan());
    println!("{}", "Flake Lock File Information".bold().cyan());
    println!("{}", "═══════════════════════════════════════".cyan());
    println!();

    // Display lock file version
    if let Some(version) = lock_data["version"].as_i64() {
        println!(
            "{} {}",
            "Lock Version:".bold(),
            version.to_string().dimmed()
        );
        println!();
    }

    // Display all inputs
    if let Some(nodes) = lock_data["nodes"].as_object() {
        let mut inputs: Vec<_> = nodes.iter().filter(|(name, _)| *name != "root").collect();

        inputs.sort_by(|a, b| a.0.cmp(b.0));

        if inputs.is_empty() {
            println!("{}", "No inputs found in lock file.".yellow());
        } else {
            println!(
                "{} {}",
                "Locked Inputs:".bold().yellow(),
                format!("({})", inputs.len()).dimmed()
            );
            println!();

            for (name, data) in inputs {
                display_input_info(name, data);
            }
        }
    }

    println!("{}", "═══════════════════════════════════════".cyan());

    Ok(())
}

/// Display information about a single input
fn display_input_info(name: &str, data: &Value) {
    println!("  {} {}", "•".green(), name.cyan().bold());

    if let Some(locked) = data["locked"].as_object() {
        // Display type
        if let Some(input_type) = locked.get("type").and_then(|v| v.as_str()) {
            println!("    {} {}", "Type:".dimmed(), input_type);
        }

        // Display owner/repo for GitHub inputs
        if let (Some(owner), Some(repo)) = (
            locked.get("owner").and_then(|v| v.as_str()),
            locked.get("repo").and_then(|v| v.as_str()),
        ) {
            println!("    {} {}/{}", "Source:".dimmed(), owner, repo);
        }

        // Display revision
        if let Some(rev) = locked.get("rev").and_then(|v| v.as_str()) {
            let short_rev = if rev.len() >= 12 { &rev[..12] } else { rev };
            println!("    {} {}", "Revision:".dimmed(), short_rev.yellow());
        }

        // Display last modified
        if let Some(modified) = locked.get("lastModified").and_then(|v| v.as_i64()) {
            let datetime = chrono::DateTime::from_timestamp(modified, 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                .unwrap_or_else(|| modified.to_string());
            println!("    {} {}", "Modified:".dimmed(), datetime);
        }

        // Display narHash
        if let Some(hash) = locked.get("narHash").and_then(|v| v.as_str()) {
            println!("    {} {}", "Hash:".dimmed(), hash.dimmed());
        }
    }

    println!();
}

/// Show lock file backup history
pub fn history() -> Result<()> {
    let backup_dir = get_backup_dir()?;

    if !backup_dir.exists() {
        println!("{}", "No lock file backups found.".yellow());
        println!(
            "\nBackups will be created automatically when you run {} or modify the lock file.",
            "flk update".cyan()
        );
        return Ok(());
    }

    // Find all lock file backups
    let mut backups: Vec<PathBuf> = fs::read_dir(&backup_dir)
        .context("Failed to read backup directory")?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with("flake.lock.")
        })
        .map(|entry| entry.path())
        .collect();

    if backups.is_empty() {
        println!("{}", "No lock file backups found.".yellow());
        return Ok(());
    }

    backups.sort_by(|a, b| {
        b.metadata()
            .and_then(|m| m.modified())
            .into_iter()
            .cmp(a.metadata().and_then(|m| m.modified()))
    });

    println!("{}", "═══════════════════════════════════════".cyan());
    println!(
        "{} {}",
        "Lock File Backup History".bold().cyan(),
        format!("({})", backups.len()).dimmed()
    );
    println!("{}", "═══════════════════════════════════════".cyan());
    println!();

    for (i, backup_path) in backups.iter().enumerate() {
        let file_name = backup_path.file_name().unwrap().to_string_lossy();
        let timestamp = file_name.strip_prefix("flake.lock.").unwrap_or("unknown");

        let metadata = fs::metadata(backup_path)?;
        let modified = metadata.modified()?;
        let datetime: chrono::DateTime<chrono::Utc> = modified.into();

        let marker = if i == 0 { "→" } else { " " };
        let number = format!("{}.", i + 1);

        println!(
            "  {} {} {} {}",
            marker.blue().bold(),
            number.dimmed(),
            timestamp.yellow(),
            format!("({})", datetime.format("%Y-%m-%d %H:%M:%S UTC")).dimmed()
        );
    }

    println!();
    println!("{}", "═══════════════════════════════════════".cyan());
    println!(
        "\nRestore a backup with: {}",
        "flk lock restore <timestamp>".cyan()
    );

    Ok(())
}

/// Restore lock file from a backup
pub fn restore(backup_id: &str) -> Result<()> {
    let backup_dir = get_backup_dir()?;

    if !backup_dir.exists() {
        bail!("No backups directory found.");
    }

    // Handle "latest" keyword
    let backup_path = if backup_id == "latest" {
        find_latest_backup(&backup_dir)?
    } else {
        // Try to find the backup by timestamp
        let candidate = backup_dir.join(format!("flake.lock.{}", backup_id));
        if !candidate.exists() {
            bail!(
                "Backup '{}' not found. Run {} to see available backups.",
                backup_id,
                "flk lock history".cyan()
            );
        }
        candidate
    };

    println!(
        "{} Restoring lock file from backup: {}",
        "→".blue().bold(),
        backup_path.file_name().unwrap().to_string_lossy().yellow()
    );

    // Create a backup of the current lock file before restoring
    let current_lock = Path::new("flake.lock");
    if current_lock.exists() {
        create_backup(current_lock)?;
    }

    // Restore the backup
    fs::copy(&backup_path, current_lock).context("Failed to restore backup")?;

    println!("{}", "✓ Lock file restored successfully!".green().bold());
    println!(
        "\nRun {} to see the current lock state.",
        "flk lock show".cyan()
    );

    Ok(())
}

/// Find the latest backup in a directory
fn find_latest_backup(backup_dir: &Path) -> Result<PathBuf> {
    let mut backups: Vec<PathBuf> = fs::read_dir(backup_dir)
        .context("Failed to read backup directory")?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with("flake.lock.")
        })
        .map(|entry| entry.path())
        .collect();

    if backups.is_empty() {
        bail!("No backups found.");
    }

    backups.sort_by(|a, b| {
        b.metadata()
            .and_then(|m| m.modified())
            .into_iter()
            .cmp(a.metadata().and_then(|m| m.modified()))
    });

    Ok(backups[0].clone())
}
