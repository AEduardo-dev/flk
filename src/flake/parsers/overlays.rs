use anyhow::{Context, Result};

use crate::flake::{
    interface::{INDENT_IN, INDENT_OUT},
    parsers::utils::find_matching_brace,
};

// NOTE: Process to follow when adding:
// 1. Neither pin nor overlay pin_exists
// 1.1. Pin gets generated with ref and name
// 1.2. Overlays get updated with new name/list matching the new pin
// 1.3. Package is added to the list for the overlay to process
// 2. Pin already pin_exists
// 2.1 Check if overlay pin_exists
// 2.2 Add package to list in overlay
// 3. Pin and overlay exist
// 3.1. Check if they match current ref
// 3.2. Check if package is named the exact name
// 3.2.1 If package is not the same, add to list
// 3.2.2 If package is the same, nothing changes

// NOTE: Pins shall be named pkgs-<hash>, where sha is the commit hash of the nixpkgs version to be
// pinned. This allows for more than one package to share the same pin, reducing the amount of pins
// if a match is found.

// FIXME: The list of packages would be ideal if it contained names such as <package>@<version>,
// this would allow for clear representation of pinned packages in the subsequent flakes/profiles

pub fn overlay_exists(content: &str, name: &str) -> Result<bool> {
    let is_present = content
        .lines()
        .any(|line| line.trim_start().starts_with(&format!("{} = [", name)));

    Ok(is_present)
}

pub fn add_overlay(content: &str, name: &str) -> Result<String> {
    // search for the pinnedPackages section
    let (_, section_end) = find_overlay_section(content)?;
    let insertion_point = section_end - INDENT_OUT.len(); // before the closing brace

    let mut result = String::new();
    result.push_str(&content[..insertion_point]);
    result.push_str(&format!("{}{} = [\n{}];\n", INDENT_IN, name, INDENT_IN));
    result.push_str(&content[insertion_point..]);

    Ok(result)
}

pub fn add_package_to_overlay(
    content: &str,
    name: &str,
    package: &str,
    version: &str,
) -> Result<String> {
    let (_, list_end) = find_overlay_list(content, name)?;
    let insertion_point = list_end; // before the closing bracket
    let package_entry = format!("{}{}@{}\n{}", INDENT_IN, package, version, INDENT_IN);

    let mut result = String::new();
    result.push_str(&content[..insertion_point]);
    result.push_str(&package_entry);
    result.push_str(&content[insertion_point..]);

    Ok(result)
}

pub fn remove_overlay(content: &str, name: &str) -> Result<String> {
    let (section_start, section_end) = find_overlay_list(content, name)?;

    let mut result = String::new();
    result.push_str(&content[..section_start - INDENT_IN.len() - name.len() - 4]); // -4 for " = ["
    result.push_str(&content[section_end + 2..]); // +2 to move past the closing bracket and
                                                  // semicolon

    Ok(result)
}

pub fn remove_package_from_overlay(content: &str, package: &str) -> Result<String> {
    let package_pattern = format!("{}@", package);
    let package_pos = content
        .find(&package_pattern)
        .context("Could not find package in any overlay")?;
    let line_start = content[..package_pos]
        .rfind('\n')
        .map(|pos| pos + 1)
        .unwrap_or(0);
    let line_end = content[package_pos..]
        .find('\n')
        .map(|pos| package_pos + pos + 1)
        .unwrap_or(content.len());

    let mut result = String::new();
    result.push_str(&content[..line_start]);
    result.push_str(&content[line_end..]);

    Ok(result)
}

pub fn pin_exists(content: &str, name: &str) -> Result<bool> {
    let is_present = content
        .lines()
        .any(|line| line.trim_start().starts_with(&format!("{} = ", name)));

    Ok(is_present)
}

pub fn add_pin(content: &str, name: &str, value: &str) -> Result<String> {
    let closing_brace_pos = content
        .rfind('}')
        .context("Could not find closing brace in pins.nix")?;

    let mut result = String::new();
    result.push_str(&content[..closing_brace_pos]);
    result.push_str(&format!("{}{} = {};\n", INDENT_IN, name, value));
    result.push_str(&content[closing_brace_pos..]);

    Ok(result)
}

pub fn remove_pin(content: &str, name: &str) -> Result<String> {
    let pin_pattern = format!("{} =", name);
    let pin_pos = content
        .find(&pin_pattern)
        .context("Could not find pin in pins.nix")?;
    let line_start = content[..pin_pos]
        .rfind('\n')
        .map(|pos| pos + 1)
        .unwrap_or(0);
    let line_end = content[pin_pos..]
        .find('\n')
        .map(|pos| pin_pos + pos + 1)
        .unwrap_or(content.len());

    let mut result = String::new();
    result.push_str(&content[..line_start]);
    result.push_str(&content[line_end..]);

    Ok(result)
}

fn find_overlay_section(content: &str) -> Result<(usize, usize)> {
    let overlays_start = content
        .find("pinnedPackages = {")
        .context("Could not find pinnedPackages section")?;

    let brace_pos = content[overlays_start..]
        .find('{')
        .context("Could not find opening brace for pinnedPackages")?;

    let section_start = overlays_start + brace_pos;
    let closing_brace = find_matching_brace(content, section_start, b'{', b'}')
        .context("Could not find closing brace for pinnedPackages")?;

    Ok((section_start, closing_brace))
}

fn find_overlay_list(content: &str, overlay_name: &str) -> Result<(usize, usize)> {
    let (section_start, section_end) = find_overlay_section(content)?;

    let overlay_start = content[section_start..section_end]
        .find(&format!("{} = [", overlay_name))
        .context("Could not find overlay in pinnedPackages section")?;

    let list_start_pos = content[section_start + overlay_start..]
        .find('[')
        .context("Could not find opening bracket for overlay list")?;

    let list_start = section_start + overlay_start + list_start_pos;
    let closing_bracket = find_matching_brace(content, list_start, b'[', b']')
        .context("Could not find closing bracket for overlay list")?;

    Ok((list_start + 1, closing_bracket)) // +1 to move past the opening bracket
}
