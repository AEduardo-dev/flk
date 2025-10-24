use anyhow::{Context, Result, bail};
use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn run(name: &str) -> Result<()> {
    let flake_path = Path::new("flake.nix");

    if !flake_path.exists() {
        bail!("No flake.nix found.");
    }

    println!("{} Removing command: {}", "→".blue().bold(), name.yellow());

    // Read the current flake.nix
    let flake_content = fs::read_to_string(flake_path).context("Failed to read flake.nix")?;

    // Check if command exists
    if !flake_content.contains(&format!("# flk-command: {}", name)) {
        bail!("Command '{}' not found in flake.nix", name);
    }

    // Remove the command from shellHook
    let updated_content = remove_from_shell_hook(&flake_content, name)?;

    // Write back to file
    fs::write(flake_path, updated_content).context("Failed to write flake.nix")?;

    println!(
        "{} Command '{}' removed successfully!",
        "✓".green().bold(),
        name
    );

    Ok(())
}

fn remove_from_shell_hook(flake_content: &str, name: &str) -> Result<String> {
    let marker = format!("# flk-command: {}", name);

    // Find the marker
    let marker_start = flake_content
        .find(&marker)
        .context("Command marker not found")?;

    // Find the start of the line - include the preceding newline if it exists
    let line_start = if marker_start > 0 {
        flake_content[..marker_start].rfind('\n').unwrap_or(0)
    } else {
        0
    };

    // Find the end of the function (closing brace + newline)
    let search_start = marker_start + marker.len();
    let function_end = flake_content[search_start..]
        .find("            }\n")
        .context("Could not find closing brace for command function")?;

    // Include the newline after the closing brace
    let end_point = search_start + function_end + "            }\n".len();

    // Remove the entire command block including surrounding newlines
    let mut result = String::new();
    result.push_str(&flake_content[..line_start]);
    result.push_str(&flake_content[end_point..]);

    Ok(result)
}
