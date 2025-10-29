use anyhow::{anyhow, Result};
use clap::CommandFactory;
use clap_complete::{generate, generate_to, shells::Shell};
use std::{env, fs, io, path::PathBuf};

use crate::Cli;

/// Entry point called from main.rs
pub fn handle_completions(install: bool, shell: Option<Shell>) -> Result<()> {
    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();

    // Detect shell automatically if not provided
    let shell = shell.or_else(detect_shell).unwrap_or(Shell::Bash);

    if install {
        let path = get_completion_install_path(shell)?;
        fs::create_dir_all(path.parent().unwrap())?;
        generate_to(shell, &mut cmd, bin_name.clone(), path.parent().unwrap())?;
        println!(
            "✅ Installed completions for {shell:?} at {}",
            path.display()
        );
        print_post_install_message(shell);
    } else {
        generate(shell, &mut cmd, bin_name, &mut io::stdout());
    }

    Ok(())
}

/// Try to detect the current shell from the environment
fn detect_shell() -> Option<Shell> {
    env::var("SHELL").ok().and_then(|path| {
        if path.contains("bash") {
            Some(Shell::Bash)
        } else if path.contains("zsh") {
            Some(Shell::Zsh)
        } else if path.contains("fish") {
            Some(Shell::Fish)
        } else if path.contains("elvish") {
            Some(Shell::Elvish)
        } else if path.contains("powershell") {
            Some(Shell::PowerShell)
        } else {
            None
        }
    })
}

/// Figure out where to install the completion script depending on shell
fn get_completion_install_path(shell: Shell) -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow!("Could not detect home directory"))?;

    let path = match shell {
        Shell::Bash => home.join(".local/share/bash-completion/completions/flk"),
        Shell::Zsh => home.join(".zsh/completions/_flk"),
        Shell::Fish => home.join(".config/fish/completions/flk.fish"),
        Shell::PowerShell => home.join("Documents/PowerShell/Microsoft.PowerShell_profile.ps1"),
        Shell::Elvish => home.join(".config/elvish/lib/completions/flk.elv"),
        _ => return Err(anyhow!("Unsupported shell for auto-install")),
    };

    Ok(path)
}

/// Print helpful info after installing completions
fn print_post_install_message(shell: Shell) {
    match shell {
        Shell::Zsh => println!(
            "\nℹ️  Make sure your ~/.zshrc contains:\n  fpath+=~/.zsh/completions\n  autoload -Uz compinit && compinit"
        ),
        Shell::Bash => println!(
            "\nℹ️  You may need to reload your shell or run:\n  source ~/.bashrc"
        ),
        Shell::Fish => println!(
            "\nℹ️  Restart your terminal or run:\n  exec fish"
        ),
        _ => (),
    }
}
