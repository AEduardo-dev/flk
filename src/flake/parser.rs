use anyhow::{Context, Result};
use std::fs;

use crate::flake::interface::{EnvVar, FlakeConfig, Package, Profile};

pub fn parse_flake(path: &str) -> Result<FlakeConfig> {
    let content = fs::read_to_string(path).context("Failed to read flake.nix file")?;

    let profiles_list = list_profiles(&content).context("Failed to list profiles")?;

    let mut profiles = Vec::new();
    for profile_name in profiles_list {
        let packages = parse_packages_from_profile(&content, Some(&profile_name))?;
        let env_vars = parse_env_vars_from_profile(&content, Some(&profile_name))?;
        let shell_hook =
            parse_shell_hook_from_profile(&content, Some(&profile_name)).unwrap_or_default(); // Use empty string if no shell hook

        let env_vars: Vec<EnvVar> = env_vars
            .into_iter()
            .map(|(name, value)| EnvVar::new(name, value))
            .collect();

        let mut profile = Profile::new(profile_name.clone());
        profile.packages = packages;
        profile.env_vars = env_vars;
        profile.shell_hook = shell_hook;

        profiles.push(profile);
    }

    let config = FlakeConfig {
        description: parse_description(&content),
        inputs: parse_inputs(&content),
        profiles,
    };

    Ok(config)
}
/// Extract the description from the flake
pub fn parse_description(content: &str) -> String {
    if let Some(start) = content.find("description = \"") {
        let search_start = start + "description = \"".len();
        if let Some(end) = content[search_start..].find("\";") {
            return content[search_start..search_start + end].to_string();
        }
    }
    String::new()
}

/// Extract flake inputs
fn parse_inputs(content: &str) -> Vec<String> {
    let mut inputs = Vec::new();

    if let Some(inputs_start) = content.find("inputs = {") {
        let search_start = inputs_start + "inputs = {".len();
        if let Some(inputs_end) = content[search_start..].find("};") {
            let inputs_section = &content[search_start..search_start + inputs_end];

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

/// Find a specific profile definition section
pub fn find_profile(content: &str, profile_name: &str) -> Result<(usize, usize)> {
    let pattern = format!("{} = {{", profile_name);

    let profile_start = content.find(&pattern).context(format!(
        "Could not find profile '{}' in profileDefinitions",
        profile_name
    ))?;

    let brace_start = profile_start + pattern.len() - 1;
    let profile_end = find_matching_brace(content, brace_start)
        .context("Could not find closing brace for profile")?;

    Ok((profile_start, profile_end + 1))
}

/// Find matching closing brace for an opening brace
fn find_matching_brace(content: &str, start: usize) -> Result<usize> {
    let mut depth = 0;
    let bytes = content.as_bytes();

    for i in start..bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(i);
                }
            }
            _ => {}
        }
    }

    Err(anyhow::anyhow!("No matching closing brace found"))
}

/// Find the profileDefinitions section
pub fn find_profile_definitions(content: &str) -> Result<(usize, usize)> {
    let profile_defs_start = content
        .find("profileDefinitions = {")
        .context("Could not find 'profileDefinitions' section")?;

    let brace_start = profile_defs_start + "profileDefinitions = {".len() - 1;
    let profile_defs_end = find_matching_brace(content, brace_start)
        .context("Could not find closing brace for profileDefinitions")?;

    Ok((profile_defs_start, profile_defs_end + 1))
}

/// Find packages section within a profile
pub fn find_packages_in_profile(content: &str, profile_name: &str) -> Result<(usize, usize, bool)> {
    let (profile_start, profile_end) = find_profile(content, profile_name)?;
    let profile_content = &content[profile_start..profile_end];

    // Look for "packages = with pkgs; [" or "packages = ["
    let (packages_start, has_with_pkgs) =
        if let Some(pos) = profile_content.find("packages = with pkgs; [") {
            (pos, true)
        } else if let Some(pos) = profile_content.find("packages = [") {
            (pos, false)
        } else {
            return Err(anyhow::anyhow!(
                "Could not find packages section in profile '{}'",
                profile_name
            ));
        };

    let bracket_pos = profile_content[packages_start..]
        .find('[')
        .context("Could not find opening bracket for packages")?;

    let list_start = profile_start + packages_start + bracket_pos + 1;

    let closing_bracket = profile_content[packages_start + bracket_pos..]
        .find("];")
        .context("Could not find closing bracket for packages")?;

    let list_end = profile_start + packages_start + bracket_pos + closing_bracket;

    Ok((list_start, list_end, has_with_pkgs))
}

/// Parse packages from a specific profile (or first one if None)
pub fn parse_packages_from_profile(
    content: &str,
    profile_name: Option<&str>,
) -> Result<Vec<Package>> {
    let mut packages = Vec::new();

    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile(content)?,
    };

    let (list_start, list_end, has_with_pkgs) =
        match find_packages_in_profile(content, &profile_to_parse) {
            Ok(result) => result,
            Err(_) => return Ok(packages),
        };

    let packages_content = &content[list_start..list_end];

    for line in packages_content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

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

/// Get the default shell profile from flake.nix
fn get_default_shell_profile(content: &str) -> Result<String> {
    if let Some(default_start) = content.find("defaultShell = \"") {
        let search_start = default_start + "defaultShell = \"".len();
        if let Some(end) = content[search_start..].find('"') {
            return Ok(content[search_start..search_start + end].to_string());
        }
    }
    // Fallback to first profile if no defaultShell set
    get_first_profile_name(content)
}

/// Get first profile name from profileDefinitions
fn get_first_profile_name(content: &str) -> Result<String> {
    let (defs_start, defs_end) = find_profile_definitions(content)?;
    let defs_content = &content[defs_start..defs_end];

    // Look for pattern "name = {"
    for line in defs_content.lines() {
        let trimmed = line.trim();
        if let Some(eq_pos) = trimmed.find(" = {") {
            let name = trimmed[..eq_pos].trim();
            if !name.is_empty() && name != "profileDefinitions" {
                return Ok(name.to_string());
            }
        }
    }

    Err(anyhow::anyhow!("No profiles found in profileDefinitions"))
}

/// List all profile names
pub fn list_profiles(content: &str) -> Result<Vec<String>> {
    let (defs_start, defs_end) = find_profile_definitions(content)?;
    let defs_content = &content[defs_start..defs_end];
    let mut profiles = Vec::new();

    // Profile names should be simple identifiers at the right indentation level
    let reserved = [
        "packages",
        "envVars",
        "shellHook",
        "containerConfig",
        "scripts",
    ];

    for line in defs_content.lines() {
        let trimmed = line.trim();
        let indent = line.len() - trimmed.len();

        if let Some(eq_pos) = trimmed.find(" = {") {
            let name = trimmed[..eq_pos].trim();

            // Only top-level profiles (indent 8-12 spaces) and not reserved keywords
            if !name.is_empty() && indent >= 8 && indent <= 12 && !reserved.contains(&name) {
                profiles.push(name.to_string());
            }
        }
    }

    Ok(profiles)
}

/// Find envVars section within a profile
pub fn find_env_vars_in_profile(content: &str, profile_name: &str) -> Result<(usize, usize)> {
    let (profile_start, profile_end) = find_profile(content, profile_name)?;
    let profile_content = &content[profile_start..profile_end];

    let envvars_start = profile_content.find("envVars = {").context(format!(
        "Could not find 'envVars' section in profile '{}'",
        profile_name
    ))?;

    let brace_start = envvars_start + "envVars = {".len() - 1;
    let search_start = profile_start + brace_start;

    let envvars_end = find_matching_brace(content, search_start)
        .context("Could not find closing brace for envVars")?;

    let section_start = profile_start + envvars_start + "envVars = {".len();

    Ok((section_start, envvars_end))
}

/// Parse environment variables from a specific profile (or first one if None)
pub fn parse_env_vars_from_profile(
    content: &str,
    profile_name: Option<&str>,
) -> Result<Vec<(String, String)>> {
    let mut env_vars = Vec::new();

    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile(content)?,
    };

    let (envvars_start, envvars_end) = match find_env_vars_in_profile(content, &profile_to_parse) {
        Ok(result) => result,
        Err(_) => return Ok(env_vars),
    };

    let envvars_content = &content[envvars_start..envvars_end];

    for line in envvars_content.lines() {
        let trimmed = line.trim();

        if let Some(eq_pos) = trimmed.find('=') {
            let name = trimmed[..eq_pos].trim();
            let value_part = trimmed[eq_pos + 1..].trim();

            let value = value_part
                .trim_end_matches(';')
                .trim_start_matches('"')
                .trim_end_matches('"')
                .trim_start_matches('\'')
                .trim_end_matches('\'');

            env_vars.push((name.to_string(), value.to_string()));
        }
    }

    Ok(env_vars)
}

/// Find shellHook section within a profile
pub fn find_shell_hook_in_profile(content: &str, profile_name: &str) -> Result<(usize, usize)> {
    let (profile_start, profile_end) = find_profile(content, profile_name)?;
    let profile_content = &content[profile_start..profile_end];

    let shell_hook_start = profile_content
        .find("shellHook = ''")
        .or_else(|| profile_content.find("shellHook=''"))
        .context(format!(
            "Could not find 'shellHook' in profile '{}'",
            profile_name
        ))?;

    let search_start = shell_hook_start + "shellHook = ''".len();
    let shell_hook_end = profile_content[search_start..]
        .find("'';")
        .context("Could not find closing \"'';\" for shellHook")?;

    let absolute_start = profile_start + shell_hook_start;
    let absolute_end = profile_start + search_start + shell_hook_end;

    Ok((absolute_start, absolute_end))
}

/// Parse shellHook from a specific profile (or first one if None)
pub fn parse_shell_hook_from_profile(content: &str, profile_name: Option<&str>) -> Result<String> {
    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile(content)?,
    };

    let (shell_hook_start, shell_hook_end) =
        find_shell_hook_in_profile(content, &profile_to_parse)?;

    let hook_start = shell_hook_start + "shellHook = ''".len();
    let hook_content = &content[hook_start..shell_hook_end];

    Ok(hook_content.trim().to_string())
}

/// Check if a package exists in a profile
pub fn package_exists(
    flake_content: &str,
    package: &str,
    profile_name: Option<&str>,
) -> Result<bool> {
    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile(flake_content)?,
    };
    let (start, end, _) = find_packages_in_profile(flake_content, profile_to_parse.as_str())?;
    let packages_content = &flake_content[start..end];

    for line in packages_content.lines() {
        let trimmed = line.trim();
        if trimmed == package || trimmed.starts_with(&format!("{}.", package)) {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Add a package to a profile
pub fn add_package_to_profile(
    flake_content: &str,
    package: &str,
    profile_name: Option<&str>,
) -> Result<String> {
    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile(flake_content)?,
    };
    let (list_start, list_end, has_with_pkgs) =
        find_packages_in_profile(flake_content, profile_to_parse.as_str())?;

    let packages_content = &flake_content[list_start..list_end];

    let indent = if let Some(pkg_line) = packages_content.lines().nth(1) {
        pkg_line.len() - pkg_line.trim_start().len()
    } else {
        12
    };

    let indent_str = " ".repeat(indent);
    let indent_bracket = " ".repeat(indent - 2);

    let is_empty = packages_content.trim().is_empty();
    let prefix = if !has_with_pkgs { "pkgs." } else { "" };

    let package_entry = if is_empty {
        format!("\n{}{}{}\n{}", indent_str, prefix, package, indent_bracket)
    } else {
        format!("{}{}\n{}", prefix, package, indent_bracket)
    };

    let mut result = String::new();
    result.push_str(&flake_content[..list_end]);
    result.push_str(&package_entry);
    result.push_str(&flake_content[list_end..]);

    Ok(result)
}

/// Remove a package from a profile
pub fn remove_package_from_profile(
    flake_content: &str,
    package: &str,
    profile_name: Option<&str>,
) -> Result<String> {
    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile(flake_content)?,
    };
    let (list_start, list_end, has_with_pkgs) =
        find_packages_in_profile(flake_content, profile_to_parse.as_str())?;

    let packages_content = &flake_content[list_start..list_end];
    let is_empty = packages_content.trim().is_empty();

    if is_empty {
        return Ok(flake_content.to_string());
    }

    let prefix = if !has_with_pkgs { "pkgs." } else { "" };
    let pkg = format!("{}{}", prefix, package);

    let pkg_start = packages_content
        .find(&pkg)
        .context("Could not find package in the current list")?;

    let pkg_end = pkg_start + pkg.len();

    let indent = if let Some(first_line) = packages_content.lines().nth(0) {
        first_line.len() - first_line.trim_start().len()
    } else {
        12
    };

    let absolute_pkg_start = list_start + pkg_start - (indent + 1);
    let absolute_pkg_end = list_start + pkg_end;

    let mut result = String::new();
    result.push_str(&flake_content[..absolute_pkg_start]);
    result.push_str(&flake_content[absolute_pkg_end..]);

    Ok(result)
}

/// Check if an environment variable exists in a profile
pub fn env_var_exists(flake_content: &str, name: &str, profile_name: &str) -> Result<bool> {
    let export_pattern = format!("{} = ", name);
    let (envvars_start, envvars_end) = find_env_vars_in_profile(flake_content, profile_name)?;
    let envvars_content = &flake_content[envvars_start..envvars_end];

    for line in envvars_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&export_pattern) {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Add an environment variable to a profile
pub fn add_env_var_to_profile(
    flake_content: &str,
    name: &str,
    value: &str,
    profile_name: Option<&str>,
) -> Result<String> {
    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile(flake_content)?,
    };

    env_var_exists(flake_content, name, profile_to_parse.as_str())
        .context("Environtment variable already exists")?;
    let (_, insertion_point) = find_env_vars_in_profile(flake_content, profile_to_parse.as_str())?;

    let escaped_value = value.replace('"', "\\\"");
    let env_block = format!("      {} = \"{}\";\n", name, escaped_value);

    let mut result = String::new();
    result.push_str(&flake_content[..insertion_point]);
    result.push_str(&env_block);
    result.push_str(indent_lines("    }", 0).as_str());
    result.push_str(&flake_content[insertion_point + 5..]);

    Ok(result)
}

/// Remove an environment variable from a profile
pub fn remove_env_var_from_profile(
    flake_content: &str,
    name: &str,
    profile_name: Option<&str>,
) -> Result<String> {
    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile(flake_content)?,
    };
    let (envvars_start, envvars_end) =
        match find_env_vars_in_profile(flake_content, profile_to_parse.as_str()) {
            Ok(result) => result,
            Err(_) => return Ok(flake_content.to_string()),
        };

    if !env_var_exists(flake_content, name, profile_to_parse.as_str())? {
        return Ok(flake_content.to_string());
    }

    let envvars_content = &flake_content[envvars_start..envvars_end];
    let var_pattern = format!("{} = ", name);

    let var_start = envvars_content
        .find(&var_pattern)
        .context("Could not find environment variable in envVars section")?;

    let var_end = envvars_content[var_start..]
        .find(';')
        .context("Could not find semicolon ending for environment variable")?;

    let var_line_end = var_start + var_end + 1;

    let indent = if let Some(line_start_in_section) = envvars_content[..var_start].rfind('\n') {
        var_start - line_start_in_section - 1
    } else {
        var_start
    };

    let absolute_var_start = envvars_start + var_start - (indent + 1);
    let absolute_var_end = envvars_start + var_line_end;

    let mut result = String::new();
    result.push_str(&flake_content[..absolute_var_start]);
    result.push_str(&flake_content[absolute_var_end..]);

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

// Command functions remain similar but need to target specific profiles
// Keeping the old ones for now for backward compatibility

pub fn find_command(flake_content: &str, name: &str, profile_name: &str) -> Option<(usize, usize)> {
    let (shell_hook_start, shell_hook_end) =
        find_shell_hook_in_profile(flake_content, profile_name).ok()?;
    let marker = format!("# flk-command: {}", name);
    let shell_hook_content = &flake_content[shell_hook_start..shell_hook_end];
    let marker_start = shell_hook_content.find(&marker)? + shell_hook_start;
    let line_start = if marker_start > 0 {
        flake_content[..marker_start].rfind('\n').unwrap_or(0)
    } else {
        0
    };
    let search_start = marker_start + marker.len();
    let function_end = flake_content[search_start..].find("            }\n")?;
    let end_point = search_start + function_end + "            }\n".len();
    Some((line_start, end_point))
}

pub fn command_exists(flake_content: &str, name: &str) -> bool {
    flake_content.contains(&format!("# flk-command: {}", name))
}

pub fn add_command_to_shell_hook(
    flake_content: &str,
    name: &str,
    command: &str,
    profile_name: Option<&str>,
) -> Result<String> {
    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile(flake_content)?,
    };
    let (_, insertion_point) =
        find_shell_hook_in_profile(flake_content, profile_to_parse.as_str())?;

    let command_block = format!(
        "\n{}{}\n{} () {{\n{}\n{}\n",
        indent_lines("# flk-command: ", 12),
        name,
        indent_lines(name, 12),
        indent_lines(command.trim(), 14),
        indent_lines("}", 12)
    );

    let mut result = String::new();
    result.push_str(&flake_content[..insertion_point]);
    result.push_str(&command_block);
    result.push_str(indent_lines("'';", 8).as_str());
    result.push_str(&flake_content[insertion_point + 3..]);

    Ok(result)
}

pub fn remove_command_from_shell_hook(
    flake_content: &str,
    name: &str,
    profile_name: Option<&str>,
) -> Result<String> {
    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile(flake_content)?,
    };
    let (line_start, end_point) = find_command(flake_content, name, profile_to_parse.as_str())
        .context("Command marker not found")?;

    let mut result = String::new();
    result.push_str(&flake_content[..line_start]);
    result.push_str(&flake_content[end_point..]);

    Ok(result)
}
