use anyhow::{Context, Result};
use clap::builder::OsStr;
use std::{fs, path::PathBuf};

/// Find matching closing brace for an opening brace
pub fn find_matching_brace(
    content: &str,
    start: usize,
    open_char: u8,
    close_char: u8,
) -> Result<usize> {
    let mut depth = 0;
    let bytes = content.as_bytes();
    for (i, _) in bytes.iter().enumerate().skip(start) {
        match bytes[i] {
            c if c == open_char => {
                depth += 1;
            }
            c if c == close_char => {
                depth -= 1;
                if depth == 0 {
                    return Ok(i);
                }
            }
            _ => {}
        }
    }
    Err(anyhow::anyhow!("No matching closing brace found"))
}

/// Get the default shell profile from default.nix helper
pub fn get_default_shell_profile() -> Result<String> {
    let content = fs::read_to_string(".flk/default.nix").context("Failed to read flake.nix")?;
    if let Some(default_start) = content.find("defaultShell = \"") {
        let search_start = default_start + "defaultShell = \"".len();
        if let Some(end) = content[search_start..].find('"') {
            return Ok(content[search_start..search_start + end].to_string());
        }
    }
    // Fallback to first profile if no defaultShell set
    get_first_profile_name()
}

/// Get first profile name from pofiles directory
fn get_first_profile_name() -> Result<String> {
    let profiles = list_profiles()?;
    if let Some(first_profile) = profiles.first() {
        if let Some(name) = first_profile.file_stem() {
            return Ok(name.to_string_lossy().to_string());
        }
    }
    Err(anyhow::anyhow!("No profiles found in .flk/profiles/"))
}

/// List all profile names
pub fn list_profiles() -> Result<Vec<PathBuf>> {
    Ok(std::fs::read_dir(".flk/profiles")
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() == Some(&OsStr::from("nix")))
        .filter(|e| e.path().file_name() != Some(&OsStr::from("default.nix")))
        .map(|e| e.path())
        .collect())
}

/// Indent lines by a specified number of spaces
pub fn indent_lines(text: &str, spaces: usize) -> String {
    let indent = " ".repeat(spaces);
    text.lines()
        .map(|line| {
            if line.trim().is_empty() {
                String::new()
            } else {
                format!("{}{}", indent, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
