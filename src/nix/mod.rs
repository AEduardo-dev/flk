//! # Nix Command Interface
//!
//! Low-level interface for executing Nix commands from the CLI.
//!
//! This module provides functions for running Nix commands and checking
//! Nix availability, abstracting the subprocess management details.

use anyhow::{Context, Result};
use std::process::Command;

/// Execute a Nix command with the given arguments.
///
/// Returns a tuple of (stdout, stderr, success) where success indicates
/// whether the command exited with status code 0.
///
/// # Arguments
///
/// * `args` - Command-line arguments to pass to `nix`
///
/// # Example
///
/// ```rust,ignore
/// let (stdout, stderr, success) = run_nix_command(&["search", "nixpkgs", "ripgrep"])?;
/// if success {
///     println!("Found packages: {}", stdout);
/// }
/// ```
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

/// Check if Nix is available on the system.
///
/// Returns `true` if the `nix --version` command executes successfully,
/// indicating that Nix is installed and accessible in PATH.
pub fn check_nix_available() -> bool {
    Command::new("nix").arg("--version").output().is_ok()
}
