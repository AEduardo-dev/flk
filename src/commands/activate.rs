use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::Result;
use colored::Colorize;

#[cfg(unix)]
use signal_hook::consts::signal::SIGUSR1;
#[cfg(unix)]
use signal_hook::flag;

pub fn run_activate() -> Result<()> {
    #[cfg(unix)]
    {
        run_activate_unix()
    }

    #[cfg(windows)]
    {
        run_activate_windows()
    }
}

#[cfg(unix)]
fn run_activate_unix() -> Result<()> {
    // Set up signal handler for SIGUSR1
    let refresh_requested = Arc::new(AtomicBool::new(false));
    flag::register(SIGUSR1, Arc::clone(&refresh_requested))?;

    let mut current_profile: Option<String> = None;

    loop {
        if let Some(ref profile) = current_profile {
            println!("Activating nix develop shell with profile: {}. Type 'refresh' to reload, 'switch <profile>' to change profile, or 'exit' to leave.", profile.cyan());
        } else {
            println!("Activating nix develop shell. Type 'refresh' to reload, 'switch <profile>' to change profile, or 'exit' to leave.");
        }

        // Reset refresh flag
        refresh_requested.store(false, Ordering::Relaxed);

        // Build nix develop command
        let mut cmd = Command::new("nix");
        cmd.arg("develop");
        if let Some(ref profile) = current_profile {
            cmd.arg(format!(".#{}", profile));
        }
        cmd.arg("--impure");

        let status = cmd.status().expect("Failed to start nix develop shell");

        // Check if refresh was requested via signal
        if refresh_requested.load(Ordering::Relaxed) {
            let action_file = "/tmp/devshell-expected-profile";

            if let Ok(new_profile) = std::fs::read_to_string(action_file) {
                let new_profile = new_profile.trim().to_string();
                println!("🔄 Switching to profile: {}", new_profile);
                current_profile = Some(new_profile);
                std::fs::remove_file(action_file).ok();
            } else {
                println!("🔄 Reloading current shell...");
            }
            continue;
        }

        // Backward compatibility for exit codes
        match status.code() {
            Some(100) => {
                // Backward compatibility for exit code 100
                println!("🔄 Reloading nix develop shell...");
                continue;
            }
            Some(127) | Some(0) => {
                println!("✅ Exiting shell wrapper.");
                break;
            }
            Some(code) => {
                println!("Shell exited with code: {}", code);
                break;
            }
            None => {
                println!("Shell terminated by signal.");
                break;
            }
        }
    }
    Ok(())
}

#[cfg(windows)]
fn run_activate_windows() -> Result<()> {
    // Windows version without signal handling
    // Note: SIGUSR1 doesn't exist on Windows, so we use exit codes only
    let mut current_profile: Option<String> = None;

    loop {
        if let Some(ref profile) = current_profile {
            println!("Activating nix develop shell with profile: {}. Type 'refresh' to reload, 'switch <profile>' to change profile, or 'exit' to leave.", profile.cyan());
        } else {
            println!("Activating nix develop shell. Type 'refresh' to reload, 'switch <profile>' to change profile, or 'exit' to leave.");
        }

        // Build nix develop command
        let mut cmd = Command::new("nix");
        cmd.arg("develop");
        if let Some(ref profile) = current_profile {
            cmd.arg(format!(". #{}", profile));
        }
        cmd.arg("--impure");

        let status = cmd.status().expect("Failed to start nix develop shell");

        // Windows: rely on exit codes for profile switching
        // Check for action file (can still work on Windows if written by shell hooks)
        let action_file = "C:\\tmp\\devshell-expected-profile"; // Windows path
        if std::path::Path::new(action_file).exists() {
            if let Ok(new_profile) = std::fs::read_to_string(action_file) {
                let new_profile = new_profile.trim().to_string();
                println!("🔄 Switching to profile: {}", new_profile);
                current_profile = Some(new_profile);
                std::fs::remove_file(action_file).ok();
                continue;
            }
        }

        // Backward compatibility for exit codes
        match status.code() {
            Some(100) => {
                println!("🔄 Reloading nix develop shell...");
                continue;
            }
            Some(127) | Some(0) => {
                println!("✅ Exiting shell wrapper.");
                break;
            }
            Some(code) => {
                println!("Shell exited with code: {}", code);
                break;
            }
            None => {
                println!("Shell terminated by signal.");
                break;
            }
        }
    }
    Ok(())
}
