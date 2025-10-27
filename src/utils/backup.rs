use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Get the backup directory path (.flk/backups in current directory)
pub fn get_backup_dir() -> Result<PathBuf> {
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    Ok(current_dir.join(".flk").join("backups"))
}

/// Create a timestamped backup of a file
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

/// Ensure .flk directory structure exists
pub fn ensure_flk_dir() -> Result<()> {
    let backup_dir = get_backup_dir()?;
    fs::create_dir_all(&backup_dir).context("Failed to create .flk directory structure")?;

    // Add .flk to .gitignore if not already there
    add_to_gitignore()?;

    Ok(())
}

/// Add .flk directory to .gitignore
fn add_to_gitignore() -> Result<()> {
    let gitignore_path = Path::new(".gitignore");

    let mut content = if gitignore_path.exists() {
        fs::read_to_string(gitignore_path).context("Failed to read .gitignore")?
    } else {
        String::new()
    };

    // Check if .flk is already in .gitignore
    if !content
        .lines()
        .any(|line| line.trim() == ".flk" || line.trim() == ".flk/")
    {
        if !content.ends_with('\n') && !content.is_empty() {
            content.push('\n');
        }
        content.push_str("\n# flk backup directory\n.flk/\n");
        fs::write(gitignore_path, content).context("Failed to update .gitignore")?;
    }

    Ok(())
}
