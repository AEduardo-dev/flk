use anyhow::{Context, Result};
use colored::Colorize;

use crate::flake::interface::{INDENT_IN, INDENT_OUT};
use crate::flake::parsers::utils::{find_matching_brace, get_default_shell_profile};

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
