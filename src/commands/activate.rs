use std::process::Command;

use anyhow::Result;

pub fn run_activate() -> Result<()> {
    loop {
        println!("Activating nix develop shell. Type 'refresh' to reload, or 'exit' to leave.");
        // Spawn nix develop shell with a bash wrapper to handle 'refresh' and 'exit'
        let status = Command::new("nix")
            .arg("develop")
            .status()
            .expect("Failed to start nix develop shell");

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
