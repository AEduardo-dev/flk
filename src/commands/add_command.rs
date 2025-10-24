use anyhow::{Context, Result, bail};
use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn run(name: &str, command: &str, file: Option<String>) -> Result<()> {
    let flake_path = Path::new("flake.nix");

    if !flake_path.exists() {
        bail!("No flake.nix found. Run {} first.", "flk init".yellow());
    }

    // Validate command name
    if !is_valid_command_name(name) {
        bail!(
            "Invalid command name '{}'. Use only letters, numbers, hyphens, and underscores.",
            name
        );
    }

    println!("{} Adding command: {}", "→".blue().bold(), name.green());

    let command_content = if let Some(filepath) = file {
        println!("  Sourcing from: {}", filepath.cyan());
        fs::read_to_string(&filepath)
            .with_context(|| format!("Failed to read file: {}", filepath))?
    } else {
        command.to_string()
    };

    if command_content.trim().is_empty() {
        bail!("Command cannot be empty");
    }

    // Read the current flake.nix
    let flake_content = fs::read_to_string(flake_path).context("Failed to read flake.nix")?;

    // Check if command already exists
    if flake_content.contains(&format!("# flk-command: {}", name)) {
        bail!(
            "Command '{}' already exists. Remove it with: {}",
            name,
            format!("flk remove-command {}", name).cyan()
        );
    }

    // Find the shellHook section and add the command
    let updated_content = add_to_shell_hook(&flake_content, name, &command_content)?;

    // Write back to file
    fs::write(flake_path, updated_content).context("Failed to write flake.nix")?;

    println!(
        "{} Command '{}' added successfully!",
        "✓".green().bold(),
        name
    );
    println!("\n{}", "Next steps:".bold());
    println!("  1. Run {} to enter the dev shell", "nix develop".cyan());
    println!("  2. Use your command: {}", name.cyan());

    Ok(())
}

fn add_to_shell_hook(flake_content: &str, name: &str, command: &str) -> Result<String> {
    // Find the shellHook section
    let shell_hook_start = flake_content
        .find("shellHook = ''")
        .or_else(|| flake_content.find("shellHook=''"))
        .context("Could not find 'shellHook' in flake.nix. Is this a valid flake?")?;

    // Find the closing of shellHook
    let search_start = shell_hook_start + "shellHook = ''".len();
    let shell_hook_end = flake_content[search_start..]
        .find("'';")
        .context("Could not find closing \"'';\") for shellHook")?;

    let insertion_point = search_start + shell_hook_end;

    // Always create a function (cleaner and supports multiline)
    let command_block = format!(
        "\n            # flk-command: {}\n            {} () {{\n{}\n            }}\n",
        name,
        name,
        indent_lines(command.trim(), 14)
    );

    // Insert the command before the closing ''
    let mut result = String::new();
    result.push_str(&flake_content[..insertion_point]);
    result.push_str(&command_block);
    result.push_str(&flake_content[insertion_point..]);

    Ok(result)
}

fn indent_lines(text: &str, spaces: usize) -> String {
    let indent = " ".repeat(spaces);
    text.lines()
        .map(|line| {
            if line.trim().is_empty() {
                String::new()
            } else {
                format!("{}{}", indent, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn is_valid_command_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        && !name.starts_with('-')
}
