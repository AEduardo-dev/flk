use anyhow::{Context, Result};

use crate::flake::interface::{Package, INDENT_IN, INDENT_OUT};
use crate::flake::parsers::utils::{find_matching_brace, get_default_shell_profile};

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
