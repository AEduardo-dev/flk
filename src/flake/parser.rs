use anyhow::{Context, Result};
use clap::builder::OsStr;
use colored::Colorize;
use std::{fs, path::PathBuf};

use crate::flake::interface::{EnvVar, FlakeConfig, Package, Profile};

const INDENT_IN: &str = "    "; // 4 spaces
const INDENT_OUT: &str = "  "; // 2 spaces

pub fn parse_flake(path: &str) -> Result<FlakeConfig> {
    let content = fs::read_to_string(path).context("Failed to read flake.nix file")?;

    let profiles_list = list_profiles().context("Failed to list profiles")?;

    let mut profiles = Vec::new();
    for profile_path in profiles_list {
        let profile_data = fs::read_to_string(&profile_path).with_context(|| {
            format!(
                "Failed to read profile file: {}",
                profile_path.to_string_lossy()
            )
        })?;
        let packages =
            parse_packages_from_profile(&profile_data, Some(profile_path.to_str().unwrap()))?;
        let env_vars =
            parse_env_vars_from_profile(&profile_data, Some(profile_path.to_str().unwrap()))?;
        let shell_hook =
            parse_shell_hook_from_profile(&profile_data, Some(profile_path.to_str().unwrap()))
                .unwrap_or_default(); // Use empty string if no shell hook

        let env_vars: Vec<EnvVar> = env_vars
            .into_iter()
            .map(|(name, value)| EnvVar::new(name, value))
            .collect();

        let mut profile = Profile::new(profile_data.to_string().clone());
        profile.name = profile_path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();
        profile.packages = packages;
        profile.env_vars = env_vars;
        profile.shell_hook = shell_hook;

        profiles.push(profile);
    }

    let config = FlakeConfig {
        inputs: parse_inputs(&content),
        profiles,
    };

    Ok(config)
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

/// Find matching closing brace for an opening brace
fn find_matching_brace(
    content: &str,
    start: usize,
    open_char: u8,
    close_char: u8,
) -> Result<usize> {
    let mut depth = 0;
    let bytes = content.as_bytes();
    for (i, _) in bytes.iter().enumerate().skip(start) {
        match bytes[i] {
            c if c == open_char => {
                depth += 1;
            }
            c if c == close_char => {
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

/// Find packages section within a profile
pub fn find_packages_in_profile(content: &str, profile_name: &str) -> Result<(usize, usize, bool)> {
    // Look for "packages = with pkgs; [" or "packages = ["

    let (packages_start, has_with_pkgs) = if let Some(pos) = content.find("packages = with") {
        (pos, true)
    } else if let Some(pos) = content.find("packages =") {
        (pos, false)
    } else {
        return Err(anyhow::anyhow!(
            "Could not find packages section in profile '{}'",
            profile_name
        ));
    };

    let bracket_pos = content[packages_start..]
        .find('[')
        .context("Could not find opening bracket for packages")?;

    let list_start = packages_start + bracket_pos;

    let closing_bracket = find_matching_brace(content, list_start, b'[', b']')
        .context("Could not find closing bracket for packages")?;

    let list_end = closing_bracket;

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
        None => get_default_shell_profile()?,
    };

    let (list_start, list_end, has_with_pkgs) =
        match find_packages_in_profile(content, &profile_to_parse) {
            Ok(result) => result,
            Err(_) => return Ok(packages),
        };

    let packages_content = &content[list_start..list_end];

    for line in packages_content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed == "[" || trimmed == "]" {
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

/// Get the default shell profile from default.nix helper
pub fn get_default_shell_profile() -> Result<String> {
    let content = fs::read_to_string(".flk/default.nix").context("Failed to read flake.nix")?;
    if let Some(default_start) = content.find("defaultShell = \"") {
        let search_start = default_start + "defaultShell = \"".len();
        if let Some(end) = content[search_start..].find('"') {
            return Ok(content[search_start..search_start + end].to_string());
        }
    }
    // Fallback to first profile if no defaultShell set
    get_first_profile_name()
}

/// Get first profile name from pofiles directory
fn get_first_profile_name() -> Result<String> {
    let profiles = list_profiles()?;
    if let Some(first_profile) = profiles.first() {
        if let Some(name) = first_profile.file_stem() {
            return Ok(name.to_string_lossy().to_string());
        }
    }
    Err(anyhow::anyhow!("No profiles found in .flk/profiles/"))
}

/// List all profile names
pub fn list_profiles() -> Result<Vec<PathBuf>> {
    Ok(std::fs::read_dir(".flk/profiles")
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() == Some(&OsStr::from("nix")))
        .filter(|e| e.path().file_name() != Some(&OsStr::from("default.nix")))
        .map(|e| e.path())
        .collect())
}

/// Find envVars section within a profile
pub fn find_env_vars_in_profile(content: &str, profile_name: &str) -> Result<(usize, usize)> {
    let envvars_start = content.find("envVars = {").context(format!(
        "Could not find 'envVars' section in profile '{}'",
        profile_name
    ))?;

    let brace_start = envvars_start + "envVars = ".len();
    let search_start = brace_start;

    let envvars_end = find_matching_brace(content, search_start, b'{', b'}')
        .context("Could not find closing brace for envVars")?;

    let section_start = envvars_start + "envVars = {".len();
    let section_end = envvars_end - INDENT_OUT.len();

    Ok((section_start, section_end))
}

/// Parse environment variables from a specific profile (or first one if None)
pub fn parse_env_vars_from_profile(
    content: &str,
    profile_name: Option<&str>,
) -> Result<Vec<(String, String)>> {
    let mut env_vars = Vec::new();

    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile()?,
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
    let shell_hook_start = content
        .find("shellHook = ''")
        .or_else(|| content.find("shellHook=''"))
        .context(format!(
            "Could not find 'shellHook' in profile '{}'",
            profile_name
        ))?;

    let search_start = shell_hook_start + "shellHook = ''".len();
    let shell_hook_end = content[search_start..]
        .find("'';")
        .context("Could not find closing \"'';\" for shellHook")?;

    let absolute_start = shell_hook_start;
    let absolute_end = search_start + shell_hook_end;

    Ok((absolute_start, absolute_end))
}

/// Parse shellHook from a specific profile (or first one if None)
pub fn parse_shell_hook_from_profile(content: &str, profile_name: Option<&str>) -> Result<String> {
    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile()?,
    };

    let (shell_hook_start, shell_hook_end) =
        find_shell_hook_in_profile(content, &profile_to_parse)?;

    let hook_start = shell_hook_start + "shellHook = ''".len();
    let hook_content = &content[hook_start..shell_hook_end];

    Ok(hook_content.trim().to_string())
}

/// Check if a package exists in a profile
pub fn package_exists(content: &str, package: &str, profile_name: Option<&str>) -> Result<bool> {
    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile()?,
    };
    let (start, end, _) = find_packages_in_profile(content, profile_to_parse.as_str())?;
    let packages_content = &content[start..end];

    for line in packages_content.lines() {
        let trimmed = line.trim();
        if trimmed == package || trimmed.ends_with(&format!(".{}", package)) {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Add a package to a profile
pub fn add_package_to_profile(
    content: &str,
    package: &str,
    profile_name: Option<&str>,
) -> Result<String> {
    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile()?,
    };
    let (_, list_end, has_with_pkgs) =
        find_packages_in_profile(content, profile_to_parse.as_str())?;

    let prefix = if !has_with_pkgs { "pkgs." } else { "" };

    let package_entry = format!("{}{}{}\n{}", INDENT_IN, prefix, package, INDENT_OUT);
    let insertion_point = list_end - INDENT_OUT.len();

    let mut result = String::new();
    result.push_str(&content[..insertion_point]);
    result.push_str(&package_entry);
    result.push_str(&content[list_end..]);

    Ok(result)
}

/// Remove a package from a profile
pub fn remove_package_from_profile(
    content: &str,
    package: &str,
    profile_name: Option<&str>,
) -> Result<String> {
    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile()?,
    };
    let (list_start, list_end, has_with_pkgs) =
        find_packages_in_profile(content, profile_to_parse.as_str())?;

    let packages_content = &content[list_start..list_end];
    let is_empty = packages_content.trim().is_empty();

    if is_empty {
        return Ok(content.to_string());
    }

    let prefix = if !has_with_pkgs { "pkgs." } else { "" };
    let pkg = format!("{}{}", prefix, package);

    let pkg_start = packages_content
        .find(&pkg)
        .context("Could not find package in the current list")?;

    let pkg_end = pkg_start + pkg.len();

    let absolute_pkg_start = list_start + pkg_start - (INDENT_IN.len() + 1);
    let absolute_pkg_end = list_start + pkg_end;

    let mut result = String::new();
    result.push_str(&content[..absolute_pkg_start]);
    result.push_str(&content[absolute_pkg_end..]);

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
// TODO: check all other methods with indentation, try and avoid carrying indent over from
// previou lines
// NOTE: For that one can change the insertion point to right after the last newline, not the
// closing braces
pub fn add_env_var_to_profile(
    content: &str,
    name: &str,
    value: &str,
    profile_name: Option<&str>,
) -> Result<String> {
    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile()?,
    };

    if env_var_exists(content, name, profile_to_parse.as_str())? {
        println!(
            "Environment variable '{}' already exists in profile '{}'",
            name.cyan(),
            profile_to_parse.yellow()
        );
        println!("Skipping addition.");
        return Ok(content.to_string());
    }

    let (_, insertion_point) = find_env_vars_in_profile(content, profile_to_parse.as_str())?;

    let escaped_value = value.replace('"', "\\\"");
    let env_block = format!("{}{} = \"{}\";\n", INDENT_IN, name, escaped_value);

    let mut result = String::new();
    result.push_str(&content[..insertion_point]);
    result.push_str(&env_block);
    result.push_str(&content[insertion_point..]);

    println!(
        "{} Environment variable '{}' added successfully!",
        "✓".green().bold(),
        name
    );

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
        None => get_default_shell_profile()?,
    };
    let (envvars_start, envvars_end) =
        match find_env_vars_in_profile(flake_content, profile_to_parse.as_str()) {
            Ok(result) => result,
            Err(_) => return Ok(flake_content.to_string()),
        };

    let envvars_content = &flake_content[envvars_start..envvars_end];
    let var_pattern = format!("{} = ", name);

    let var_start = envvars_content
        .find(&var_pattern)
        .context("Could not find environment variable in envVars section")?;

    let var_end = envvars_content[var_start..]
        .find(';')
        .context("Could not find semicolon ending for environment variable")?;

    let var_line_end = var_start + var_end + 1;

    let absolute_var_start = envvars_start + var_start - (INDENT_IN.len() + 1);
    let absolute_var_end = envvars_start + var_line_end;

    let mut result = String::new();
    result.push_str(&flake_content[..absolute_var_start]);
    result.push_str(&flake_content[absolute_var_end..]);

    println!(
        "{} Environment variable '{}' removed successfully!",
        "✓".green().bold(),
        name
    );

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

pub fn find_command(content: &str, name: &str, profile_name: &str) -> Option<(usize, usize)> {
    let (shell_hook_start, shell_hook_end) =
        find_shell_hook_in_profile(content, profile_name).ok()?;
    let marker = format!("# flk-command: {}", name);
    let shell_hook_content = &content[shell_hook_start..shell_hook_end];
    let marker_start = shell_hook_content.find(&marker)? + shell_hook_start;
    let line_start = if marker_start > 0 {
        content[..marker_start].rfind('\n').unwrap_or(0)
    } else {
        0
    };
    let search_start = marker_start + marker.len();
    let function_end = content[search_start..].find(&format!("{}}}\n", INDENT_IN))?;
    let end_point = search_start + function_end + format!("{}}}\n", INDENT_IN).len();
    Some((line_start, end_point))
}

pub fn command_exists(content: &str, name: &str) -> bool {
    content.contains(&format!("# flk-command: {}", name))
}

pub fn add_command_to_shell_hook(
    content: &str,
    name: &str,
    command: &str,
    profile_name: Option<&str>,
) -> Result<String> {
    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile()?,
    };
    let (_, shell_hook_end) = find_shell_hook_in_profile(content, profile_to_parse.as_str())?;

    let insertion_point = shell_hook_end - INDENT_OUT.len();

    let command_block = format!(
        "\n{}{}\n{} () {{\n{}\n{}\n",
        indent_lines("# flk-command: ", INDENT_IN.len()),
        name,
        indent_lines(name, INDENT_IN.len()),
        indent_lines(command.trim(), INDENT_IN.len() + 2),
        indent_lines("}", INDENT_IN.len())
    );

    let mut result = String::new();
    result.push_str(&content[..insertion_point]);
    result.push_str(&command_block);
    result.push_str(&content[insertion_point..]);

    Ok(result)
}

pub fn remove_command_from_shell_hook(
    content: &str,
    name: &str,
    profile_name: Option<&str>,
) -> Result<String> {
    let profile_to_parse = match profile_name {
        Some(name) => name.to_string(),
        None => get_default_shell_profile()?,
    };
    let (line_start, end_point) = find_command(content, name, profile_to_parse.as_str())
        .context("Command marker not found")?;

    let mut result = String::new();
    result.push_str(&content[..line_start]);
    result.push_str(&content[end_point..]);

    Ok(result)
}

pub fn extract_packages_from_output(output: &str) -> Result<Vec<Package>> {
    Ok(output
        .lines()
        .skip(1) // Skip header line
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                Some(Package {
                    name: parts[0].to_string(),
                    version: Some(parts[1].to_string()),
                })
            } else {
                None
            }
        })
        .collect())
}
