use anyhow::Result;
use colored::Colorize;
use flk::flake::parsers::utils::get_default_shell_profile;
use std::process::Command;

pub fn run_activate(current_profile: Option<String>) -> Result<()> {
    // Decide which profile to use
    let profile = match current_profile {
        Some(p) => p,
        None => get_default_shell_profile()?, // fallback
    };

    println!(
        "Activating nix develop shell with profile: {}.",
        profile.cyan()
    );

    // Build nix develop command
    let mut cmd = Command::new("nix");
    cmd.arg("develop");
    cmd.arg(format!(".#{}", profile));
    cmd.arg("--impure");

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
