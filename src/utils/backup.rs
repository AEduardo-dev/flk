//! # Backup Utilities
//!
//! Functions for managing lock file backups.
//!
//! Backups are stored in `.flk/backups/` with timestamps for easy
//! identification and restoration.

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Get the backup directory path (`.flk/backups` in current directory).
pub fn get_backup_dir() -> Result<PathBuf> {
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    Ok(current_dir.join(".flk").join("backups"))
}

/// Create a timestamped backup of a file.
///
/// # Arguments
///
/// * `file_path` - Path to the file to backup
///
/// # Returns
///
/// The path to the created backup file.
///
/// # Errors
///
/// Returns an error if the file doesn't exist or cannot be copied.
pub fn create_backup(file_path: &Path) -> Result<PathBuf> {
    if !file_path.exists() {
        anyhow::bail!("File does not exist: {}", file_path.display());
    }

    let backup_dir = get_backup_dir()?;
    fs::create_dir_all(&backup_dir).context("Failed to create backup directory")?;

    let timestamp = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S");
    let backup_name = format!(
        "{}.{}",
        file_path.file_name().unwrap().to_string_lossy(),
        timestamp
    );
    let backup_path = backup_dir.join(backup_name);

    fs::copy(file_path, &backup_path).context("Failed to create backup")?;

    Ok(backup_path)
}

/// Ensure the `.flk` directory structure exists.
///
/// Creates `.flk/backups/` if it doesn't exist.
pub fn ensure_flk_dir() -> Result<()> {
    let backup_dir = get_backup_dir()?;
    fs::create_dir_all(&backup_dir).context("Failed to create .flk directory structure")?;

    Ok(())
}
