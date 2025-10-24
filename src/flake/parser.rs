use anyhow::{Context, Result};
use std::fs;

/// Parse a flake.nix file and extract its components
pub fn parse_flake(path: &str) -> Result<FlakeConfig> {
    let content = fs::read_to_string(path)?;

    // TODO: Implement proper Nix expression parsing
    // This is a placeholder for issue #1

    Ok(FlakeConfig::default())
}

#[derive(Debug, Default)]
pub struct FlakeConfig {
    pub description: String,
    pub inputs: Vec<String>,
    pub packages: Vec<String>,
    pub shell_hook: String,
}

/// Find the shellHook section in a flake.nix content
pub fn find_shell_hook(flake_content: &str) -> Result<(usize, usize)> {
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

    Ok((shell_hook_start, insertion_point))
}

/// Find a command marker in the flake content
pub fn find_command(flake_content: &str, name: &str) -> Option<(usize, usize)> {
    let marker = format!("# flk-command: {}", name);

    // Find the marker
    let marker_start = flake_content.find(&marker)?;

    // Find the start of the line - include the preceding newline if it exists
    let line_start = if marker_start > 0 {
        flake_content[..marker_start].rfind('\n').unwrap_or(0)
    } else {
        0
    };

    // Find the end of the function (closing brace + newline)
    let search_start = marker_start + marker.len();
    let function_end = flake_content[search_start..].find("            }\n")?;

    // Include the newline after the closing brace
    let end_point = search_start + function_end + "            }\n".len();

    Some((line_start, end_point))
}

/// Check if a command exists in the flake
pub fn command_exists(flake_content: &str, name: &str) -> bool {
    flake_content.contains(&format!("# flk-command: {}", name))
}

/// Add a command to the shellHook section
pub fn add_command_to_shell_hook(flake_content: &str, name: &str, command: &str) -> Result<String> {
    let (_, insertion_point) = find_shell_hook(flake_content)?;

    // Create function with proper formatting
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

/// Remove a command from the shellHook section
pub fn remove_command_from_shell_hook(flake_content: &str, name: &str) -> Result<String> {
    let (line_start, end_point) =
        find_command(flake_content, name).context("Command marker not found")?;

    // Remove the entire command block including surrounding newlines
    let mut result = String::new();
    result.push_str(&flake_content[..line_start]);
    result.push_str(&flake_content[end_point..]);

    Ok(result)
}

/// Indent lines by a specified number of spaces
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
