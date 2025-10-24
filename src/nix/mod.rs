use anyhow::Result;
use std::process::Command;

/// Execute a nix command
pub fn run_nix_command(args: &[&str]) -> Result<String> {
    let output = Command::new("nix").args(args).output()?;

    if !output.status.success() {
        anyhow::bail!(
            "Nix command  nix {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8(output.stdout)?)
}

/// Check if nix is available
pub fn check_nix_available() -> bool {
    Command::new("nix").arg("--version").output().is_ok()
}
