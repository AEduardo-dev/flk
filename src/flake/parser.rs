use anyhow::{Context, Ok, Result};
use std::fs;

/// Parse a flake.nix file and extract its components
pub fn parse_flake(path: &str) -> Result<FlakeConfig> {
    let content = fs::read_to_string(path)?;
    // let (start_hook, end_hook) = find_shell_hook(&content);
    // let (start_pkgs, end_pkgs, _) = find_packages_inputs(&content);

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

/// Find buildInputs section in a flake.nix content
pub fn find_packages_inputs(flake_content: &str) -> Result<(usize, usize, bool)> {
    // Find packages = with pkgs; [ or packages = [ section
    let (build_inputs_start, has_with_pkgs) = flake_content
        .find("packages = with pkgs; [")
        .map(|pos| (pos, true))
        .or_else(|| flake_content.find("packages = [").map(|pos| (pos, false)))
        .context("Could not find 'packages' in flake.nix")?;

    // Find the opening bracket
    let bracket_pos = flake_content[build_inputs_start..]
        .find('[')
        .context("Could not find opening bracket for packages section")?;

    let list_start = build_inputs_start + bracket_pos + 1;

    // Find the closing bracket
    let closing_bracket = flake_content[list_start..]
        .find("];")
        .context("Could not find closing bracket for packages section")?;

    let list_end = list_start + closing_bracket;

    Ok((list_start, list_end, has_with_pkgs))
}

/// Check if a package exists in buildInputs
pub fn package_exists(flake_content: &str, package: &str) -> Result<bool> {
    let (start, end, _) = find_packages_inputs(flake_content)?;
    let build_inputs_content = &flake_content[start..end];

    // Check if the package name appears in the buildInputs list
    // This is a simple check - could be improved with proper parsing
    for line in build_inputs_content.lines() {
        let trimmed = line.trim();
        if trimmed == package || trimmed.starts_with(&format!("{}.", package)) {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Add a package to buildInputs
pub fn add_package_inputs(flake_content: &str, package: &str) -> Result<String> {
    let (list_start, list_end, has_with_pkgs) = find_packages_inputs(flake_content)?;

    // Get the content inside buildInputs
    let build_inputs_content = &flake_content[list_start..list_end];

    // Determine indentation by looking at existing entries
    let indent = if let Some(first_line) = build_inputs_content.lines().nth(1) {
        // Count leading spaces
        first_line.len() - first_line.trim_start().len()
    } else {
        12 // Default indentation
    };

    let indent_str = " ".repeat(indent);

    // Check if buildInputs is empty or has items
    let is_empty = build_inputs_content.trim().is_empty();
    let mut prefix = "";
    if !has_with_pkgs {
        prefix = "pkgs."
    }

    let package_entry = if is_empty {
        format!("\n{}{}{}\n          ", indent_str, prefix, package)
    } else {
        format!("\n{}{}{}", indent_str, prefix, package)
    };

    // Insert before the closing bracket
    let mut result = String::new();
    result.push_str(&flake_content[..list_end]);
    result.push_str(&package_entry);
    result.push_str(&flake_content[list_end..]);

    Ok(result)
}

/// Remove a package from buildInputs
pub fn remove_package_inputs(flake_content: &str, package: &str) -> Result<String> {
    let (list_start, list_end, has_with_pkgs) = find_packages_inputs(flake_content)?;

    // Get the content inside buildInputs
    let build_inputs_content = &flake_content[list_start..list_end];

    // Determine if file is empty. If it is, return nothing
    let is_empty = build_inputs_content.trim().is_empty();
    let mut result = String::new();
    if is_empty {
        return Ok(result);
    }

    let mut prefix = "";
    if !has_with_pkgs {
        prefix = "pkgs."
    }

    let pckg = format!("{}{}", prefix, package);

    let pckg_start = build_inputs_content
        .find(&pckg)
        .context("Could not find package in the current list")?;

    let pckg_end = pckg_start + pckg.len();

    // Insert all content but removed package
    result.push_str(&flake_content[..pckg_start]);
    result.push_str(&flake_content[pckg_end..]);

    Ok(result)
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
