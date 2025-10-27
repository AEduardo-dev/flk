use anyhow::{Context, Ok, Result};
use std::fs;

use crate::flake::interface::{FlakeConfig, Package};

/// Parse a flake.nix file and extract its components
pub fn parse_flake(path: &str) -> Result<FlakeConfig> {
    let content = fs::read_to_string(path).context("Failed to read flake.nix file")?;

    let mut config = FlakeConfig::default();

    // Parse description
    config.description = parse_description(&content);

    // Parse inputs (flake inputs/dependencies)
    config.inputs = parse_inputs(&content);

    // Parse packages from buildInputs
    config.packages = parse_packages(&content)?;

    // Parse shellHook content
    config.shell_hook = parse_shell_hook_content(&content)?;

    Ok(config)
}

/// Extract the description from the flake
fn parse_description(content: &str) -> String {
    // Look for description = "..."; pattern
    if let Some(start) = content.find("description = \"") {
        let search_start = start + "description = \"".len();
        if let Some(end) = content[search_start..].find("\";") {
            return content[search_start..search_start + end].to_string();
        }
    }
    String::new()
}

/// Extract flake inputs (dependencies like nixpkgs)
fn parse_inputs(content: &str) -> Vec<String> {
    let mut inputs = Vec::new();

    // Find the inputs section
    if let Some(inputs_start) = content.find("inputs = {") {
        let search_start = inputs_start + "inputs = {".len();
        if let Some(inputs_end) = content[search_start..].find("};") {
            let inputs_section = &content[search_start..search_start + inputs_end];

            // Parse each input (look for patterns like "nixpkgs.url = ...")
            for line in inputs_section.lines() {
                let trimmed = line.trim();
                if let Some(dot_pos) = trimmed.find('.') {
                    let input_name = trimmed[..dot_pos].trim();
                    if !input_name.is_empty() && !inputs.contains(&input_name.to_string()) {
                        inputs.push(input_name.to_string());
                    }
                }
            }
        }
    }

    inputs
}

/// Extract packages from the packages list
fn parse_packages(content: &str) -> Result<Vec<Package>> {
    let mut packages = Vec::new();

    // Use existing helper function to find packages section
    let (list_start, list_end, has_with_pkgs) = match find_packages_inputs(content) {
        Result::Ok(result) => result,
        Err(_) => return Ok(packages), // Return empty if no packages section found
    };

    let packages_content = &content[list_start..list_end];

    // Parse each package line
    for line in packages_content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Remove "pkgs." prefix if it exists and not using "with pkgs;"
        let package_name = if !has_with_pkgs && trimmed.starts_with("pkgs.") {
            trimmed.strip_prefix("pkgs.").unwrap_or(trimmed)
        } else {
            trimmed
        };

        if !package_name.is_empty() {
            packages.push(Package::new(package_name.to_string()));
        }
    }

    Ok(packages)
}

/// Extract the shellHook content
fn parse_shell_hook_content(content: &str) -> Result<String> {
    // Use existing helper function to find shellHook section
    let (shell_hook_start, shell_hook_end) = match find_shell_hook(content) {
        Result::Ok(result) => result,
        Err(_) => return Ok(String::new()), // Return empty if no shellHook found
    };

    // Extract the content between shellHook = '' and '';
    let hook_start = shell_hook_start + "shellHook = ''".len();
    let hook_content = &content[hook_start..shell_hook_end];

    Ok(hook_content.trim().to_string())
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
    let indent = if let Some(pckg_line) = build_inputs_content.lines().nth(2) {
        // Count leading spaces
        pckg_line.len() - pckg_line.trim_start().len()
    } else {
        12 // Default indentation
    };
    println!("{}", indent);

    let indent_str = " ".repeat(indent);
    let indent_bracket = " ".repeat(indent - 2);

    // Check if buildInputs is empty or has items
    let is_empty = build_inputs_content.trim().is_empty();
    let mut prefix = "";
    if !has_with_pkgs {
        prefix = "pkgs."
    }

    let package_entry = if is_empty {
        format!("\n{}{}{}\n          ", indent_str, prefix, package)
    } else {
        format!("  {}{}\n{}", prefix, package, indent_bracket)
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

    // Determine indentation by looking at existing entries
    let indent = if let Some(first_line) = build_inputs_content.lines().nth(1) {
        // Count leading spaces
        first_line.len() - first_line.trim_start().len()
    } else {
        12 // Default indentation
    };

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
    println!("{}", pckg);

    let pckg_start = build_inputs_content
        .find(&pckg)
        .map(|pos| pos)
        .context("Could not find package in the current list")?;

    let pckg_end = pckg_start + pckg.len();

    // Insert all content but removed package
    let absolute_pckg_start = list_start + pckg_start - (indent + 1);
    let absolute_pckg_end = list_start + pckg_end;

    result.push_str(&flake_content[..absolute_pckg_start]);
    result.push_str(&flake_content[absolute_pckg_end..]);

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
