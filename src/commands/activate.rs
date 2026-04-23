//! # Activate Command Handler
//!
//! Enter the Nix development shell for the current flake.

use crate::commands::profile_cache::profile_cache_inputs;
use anyhow::{Context, Result};
use colored::Colorize;
use flk::flake::parsers::utils::resolve_profile;
use std::process::Command;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

/// Enter the Nix development shell for the resolved profile.
///
/// Reuses a cached `nix develop --profile` environment when the relevant flake
/// files are unchanged, and refreshes that cache when the environment
/// definition changes.
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

    let shell = env::var("SHELL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "/bin/sh".to_string());
    let profile_path = profile_cache_path(&profile);
    let stamp_path = profile_cache_stamp_path(&profile);
    let use_cached_profile = profile_cache_is_fresh(&profile, &profile_path, &stamp_path)?;
    let mut cmd = Command::new("nix");
    cmd.arg("develop");
    if use_cached_profile {
        cmd.arg(&profile_path);
    } else {
        cmd.arg(format!(".#{}", profile));
    }
    cmd.arg("--impure");
    if !use_cached_profile {
        cmd.arg("--profile");
        cmd.arg(&profile_path);
        cmd.env("FLK_PROFILE_PATH", &profile_path);
        cmd.env("FLK_PROFILE_STAMP", &stamp_path);
        cmd.env("FLK_SHELL_CMD", &shell);
    }
    cmd.arg("-c");
    if use_cached_profile {
        cmd.arg(shell);
    } else {
        cmd.arg("/bin/sh");
        cmd.arg("-c");
        cmd.arg(
            "if [ -e \"$FLK_PROFILE_PATH\" ]; then \
             mkdir -p \"$(dirname \"$FLK_PROFILE_STAMP\")\"; \
             touch \"$FLK_PROFILE_STAMP\" || exit 1; \
             fi; \
             exec \"$FLK_SHELL_CMD\"",
        );
    }

    let status = cmd.status().with_context(|| {
        format!(
            "Failed to start nix develop shell for profile '{}'",
            profile
        )
    })?;
    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "nix develop shell exited with status: {}",
            status
        ))
    }
}

fn profile_cache_path(profile: &str) -> PathBuf {
    Path::new(".flk").join(format!(".nix-profile-{profile}"))
}

fn profile_cache_stamp_path(profile: &str) -> PathBuf {
    Path::new(".flk").join(format!(".nix-profile-{profile}.stamp"))
}

fn profile_cache_is_fresh(profile: &str, profile_path: &Path, stamp_path: &Path) -> Result<bool> {
    if !profile_path.exists() || !stamp_path.exists() {
        return Ok(false);
    }

    let stamp_modified = fs::metadata(stamp_path)
        .with_context(|| {
            format!(
                "Failed to read metadata for profile cache stamp '{}'",
                stamp_path.display()
            )
        })?
        .modified()
        .with_context(|| {
            format!(
                "Failed to read modification time for profile cache stamp '{}'",
                stamp_path.display()
            )
        })?;

    for path in profile_cache_inputs(profile) {
        if !path.exists() {
            continue;
        }

        let modified = fs::metadata(&path)
            .with_context(|| format!("Failed to read metadata for '{}'", path.display()))?
            .modified()
            .with_context(|| {
                format!("Failed to read modification time for '{}'", path.display())
            })?;

        if modified >= stamp_modified {
            return Ok(false);
        }
    }

    Ok(true)
}
