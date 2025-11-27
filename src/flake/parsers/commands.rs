use anyhow::{Context, Result};

use crate::flake::interface::{INDENT_IN, INDENT_OUT};
use crate::flake::parsers::utils::{get_default_shell_profile, indent_lines};

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
