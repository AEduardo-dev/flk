use anyhow::{Context, Result};
use std::process::Command;

/// Execute a nix command
pub fn run_nix_command(args: &[&str]) -> Result<(String, String, bool)> {
    let output = Command::new("nix")
        .args(args)
        .output()
        .context("Failed to execute nix command")?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    let success = output.status.success();

    Ok((stdout, stderr, success))
}

/// Check if nix is available
pub fn check_nix_available() -> bool {
    Command::new("nix").arg("--version").output().is_ok()
}
