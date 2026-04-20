//! # Activate Command Handler
//!
//! Enter the Nix development shell for the current flake.

use anyhow::Result;
use colored::Colorize;
use flk::flake::parsers::utils::resolve_profile;
use std::env;
use std::process::Command;

/// Enter the Nix development shell for the resolved profile.
///
/// Runs `nix develop` with `--impure` and GC root pinning for faster
/// re-activation on subsequent runs.
///
/// # Arguments
///
/// * `current_profile` - Optional profile override
pub fn run_activate(current_profile: Option<String>) -> Result<()> {
    let profile = resolve_profile(current_profile)?;

    println!(
        "Activating nix develop shell with profile: {}.",
        profile.cyan()
    );

    // Build nix develop command with GC root pinning for faster re-activation
    let profile_path = format!(".flk/.nix-profile-{}", profile);
    let shell = env::var("SHELL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "/bin/sh".to_string());
    let mut cmd = Command::new("nix");
    cmd.arg("develop");
    cmd.arg(format!(".#{}", profile));
    cmd.arg("--impure");
    cmd.arg("--profile");
    cmd.arg(&profile_path);
    cmd.arg("-c");
    cmd.arg(shell);

    let status = cmd.status().expect("Failed to start nix develop shell");
    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "nix develop shell exited with status: {}",
            status
        ))
    }
}
