use anyhow::Result;
use colored::Colorize;
use std::process::Command;

pub fn run_activate() -> Result<()> {
    let current_profile: Option<String> = None;

    if let Some(ref profile) = current_profile {
        println!("Activating nix develop shell with profile: {}. Type 'refresh' to reload, 'switch <profile>' to change profile, or 'exit' to leave.", profile.cyan());
    } else {
        println!("Activating nix develop shell. Type 'refresh' to reload, 'switch <profile>' to change profile, or 'exit' to leave.");
    }

    // Build nix develop command
    let mut cmd = Command::new("nix");
    cmd.arg("develop");
    if let Some(ref profile) = current_profile {
        cmd.arg(format!(".#{}", profile));
    }
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
