use anyhow::{bail, Context, Result};

use crate::flake::{
    interface::{INDENT_IN, INDENT_OUT},
    parsers::utils::find_matching_brace,
};

// ============================================================================
// SOURCES SECTION (pins.nix - sources = { ... })
// ============================================================================

/// Check if a source exists in the sources section
pub fn source_exists(content: &str, source_name: &str) -> Result<bool> {
    let (section_start, section_end) = find_sources_section(content)?;
    let section_content = &content[section_start..section_end];

    Ok(section_content
        .lines()
        .any(|line| line.trim_start().starts_with(&format!("{} =", source_name))))
}

/// Add a new source to the sources section
pub fn add_source(content: &str, source_name: &str, source_ref: &str) -> Result<String> {
    let (_, section_end) = find_sources_section(content)?;
    let insertion_point = section_end - INDENT_OUT.len(); // before the closing brace

    let mut result = String::new();
    result.push_str(&content[..insertion_point]);
    result.push_str(&format!(
        "{}{} = \"{}\";\n{}",
        INDENT_IN, source_name, source_ref, INDENT_OUT
    ));
    result.push_str(&content[insertion_point..]);

    Ok(result)
}

/// Remove a source from the sources section
pub fn remove_source(content: &str, source_name: &str) -> Result<String> {
    let source_pattern = format!("{} =", source_name);
    let source_pos = content
        .find(&source_pattern)
        .context("Could not find source in sources section")?;

    let line_start = content[..source_pos]
        .rfind('\n')
        .map(|pos| pos + 1)
        .unwrap_or(0);
    let line_end = content[source_pos..]
        .find('\n')
        .map(|pos| source_pos + pos + 1)
        .unwrap_or(content.len());

    let mut result = String::new();
    result.push_str(&content[..line_start]);
    result.push_str(&content[line_end..]);

    Ok(result)
}

// ============================================================================
// PINNED PACKAGES SECTION (pins. nix - pinnedPackages = { ... })
// ============================================================================

/// Check if a pin entry exists in pinnedPackages
pub fn pin_entry_exists(content: &str, pin_name: &str) -> Result<bool> {
    let (section_start, section_end) = find_pinned_packages_section(content)?;
    let section_content = &content[section_start..section_end];

    Ok(section_content
        .lines()
        .any(|line| line.trim_start().starts_with(&format!("{} = [", pin_name))))
}

/// Add a new pin entry with an empty package list
pub fn add_pin_entry(content: &str, pin_name: &str) -> Result<String> {
    let (_, section_end) = find_pinned_packages_section(content)?;
    let insertion_point = section_end - INDENT_OUT.len(); // before the closing brace

    let mut result = String::new();
    result.push_str(&content[..insertion_point]);
    result.push_str(&format!(
        "{}{} = [\n{}];\n{}",
        INDENT_IN, pin_name, INDENT_IN, INDENT_OUT
    ));
    result.push_str(&content[insertion_point..]);

    Ok(result)
}

/// Remove a pin entry from pinnedPackages
pub fn remove_pin_entry(content: &str, pin_name: &str) -> Result<String> {
    let (section_start, section_end) = find_pinned_packages_section(content)?;
    let section_content = &content[section_start..section_end];

    let entry_pattern = format!("{} = [", pin_name);
    let entry_pos = section_content
        .find(&entry_pattern)
        .context("Could not find pin entry in pinnedPackages section")?;

    let absolute_entry_pos = section_start + entry_pos;

    // Find the line start (including indentation)
    let line_start = content[..absolute_entry_pos]
        .rfind('\n')
        .map(|pos| pos + 1)
        .unwrap_or(0);

    // Find the closing bracket and semicolon
    let bracket_start = absolute_entry_pos + entry_pattern.len() - 1;
    let bracket_end = find_matching_brace(content, bracket_start, b'[', b']')
        .context("Could not find closing bracket for pin entry")?;

    let line_end = content[bracket_end..]
        .find('\n')
        .map(|pos| bracket_end + pos + 1)
        .unwrap_or(content.len());

    let mut result = String::new();
    result.push_str(&content[..line_start]);
    result.push_str(&content[line_end..]);

    Ok(result)
}

/// Check if a specific package exists in a pin's package list
pub fn package_in_pin_exists(content: &str, pin_name: &str, package_name: &str) -> Result<bool> {
    let (list_start, list_end) = find_pin_package_list(content, pin_name)?;
    let list_content = &content[list_start..list_end];

    Ok(list_content.contains(&format!("name = \"{}\"", package_name)))
}

/// Add a package to a pin's package list
pub fn add_package_to_pin(
    content: &str,
    pin_name: &str,
    package: &str,
    package_alias: &str, // e.g., "git@2. 51.2"
) -> Result<String> {
    let (_, list_end) = find_pin_package_list(content, pin_name)?;
    let insertion_point = list_end; // before the closing bracket

    let package_entry = format!(
        "{}{{ pkg = \"{}\"; name = \"{}\"; }}\n{}",
        INDENT_IN.repeat(2), // Double indent for nested list items
        package,
        package_alias,
        INDENT_IN
    );

    let mut result = String::new();
    result.push_str(&content[..insertion_point]);
    result.push_str(&package_entry);
    result.push_str(&content[insertion_point..]);

    Ok(result)
}

/// Remove a package from a pin's package list
pub fn remove_package_from_pin(
    content: &str,
    pin_name: &str,
    package_alias: &str,
) -> Result<String> {
    let (list_start, list_end) = find_pin_package_list(content, pin_name)?;
    let list_content = &content[list_start..list_end];

    let package_pattern = format!("name = \"{}\"", package_alias);
    let package_pos = list_content.find(&package_pattern).context(format!(
        "Could not find package '{}' in pin '{}'",
        package_alias, pin_name
    ))?;

    let absolute_package_pos = list_start + package_pos;

    // Find the start of the line containing this package
    let line_start = content[..absolute_package_pos]
        .rfind('\n')
        .map(|pos| pos + 1)
        .unwrap_or(0);

    // Find the end of the line (after the closing brace)
    let line_end = content[absolute_package_pos..]
        .find('\n')
        .map(|pos| absolute_package_pos + pos + 1)
        .unwrap_or(content.len());

    let mut result = String::new();
    result.push_str(&content[..line_start]);
    result.push_str(&content[line_end..]);

    Ok(result)
}

/// Check if a pin entry has any packages
pub fn pin_has_packages(content: &str, pin_name: &str) -> Result<bool> {
    let (list_start, list_end) = find_pin_package_list(content, pin_name)?;
    let list_content = &content[list_start..list_end].trim();

    Ok(!list_content.is_empty())
}

/// Find which pin a package belongs to (by package alias)
pub fn find_pin_for_package(content: &str, package_alias: &str) -> Result<String> {
    let (section_start, section_end) = find_pinned_packages_section(content)?;
    let section_content = &content[section_start..section_end];

    let package_pattern = format!("name = \"{}\"", package_alias);
    let package_pos = section_content.find(&package_pattern).context(format!(
        "Could not find package '{}' in any pin",
        package_alias
    ))?;

    // Search backwards to find the pin name
    let before_package = &section_content[..package_pos];

    // Find the most recent "pin-name = [" pattern
    let pin_pattern_end = before_package
        .rfind(" = [")
        .context("Could not find pin entry before package")?;

    let pin_name_start = before_package[..pin_pattern_end]
        .rfind('\n')
        .map(|pos| pos + 1)
        .unwrap_or(0);

    let pin_name = before_package[pin_name_start..pin_pattern_end]
        .trim()
        .to_string();

    Ok(pin_name)
}

// ============================================================================
// COMBINED OPERATIONS
// ============================================================================

/// Add a pinned package (adds source if needed, creates pin entry if needed, adds package)
pub fn add_pinned_package(
    content: &str,
    pin_hash: &str,
    source_ref: &str,
    package: &str,
    version: &str,
) -> Result<String> {
    let pin_name = format!("pkgs-{}", pin_hash);
    let package_alias = format!("{}@{}", package, version);

    // Step 1: Ensure source exists
    let mut result = if !source_exists(content, &pin_name)? {
        add_source(content, &pin_name, source_ref)?
    } else {
        content.to_string()
    };

    // Step 2: Ensure pin entry exists
    result = if !pin_entry_exists(&result, &pin_name)? {
        add_pin_entry(&result, &pin_name)?
    } else {
        result
    };

    // Step 3: Check if package already exists
    if package_in_pin_exists(&result, &pin_name, &package_alias)? {
        bail!(
            "Package '{}' already exists in pin '{}'",
            package_alias,
            pin_name
        );
    }

    // Step 4: Add the package
    add_package_to_pin(&result, &pin_name, package, &package_alias)
}

/// Remove a pinned package and cleanup if pin is empty
pub fn remove_pinned_package_with_cleanup(content: &str, package_alias: &str) -> Result<String> {
    // Step 1: Find which pin this package belongs to
    let pin_name = find_pin_for_package(content, package_alias)?;

    // Step 2: Remove the package
    let mut result = remove_package_from_pin(content, &pin_name, package_alias)?;

    // Step 3: If pin has no more packages, remove pin entry and source
    if !pin_has_packages(&result, &pin_name)? {
        result = remove_pin_entry(&result, &pin_name)?;
        result = remove_source(&result, &pin_name)?;
    }

    Ok(result)
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn find_sources_section(content: &str) -> Result<(usize, usize)> {
    let sources_start = content
        .find("sources = {")
        .context("Could not find sources section in pins.nix")?;

    let brace_pos = content[sources_start..]
        .find('{')
        .context("Could not find opening brace for sources")?;

    let section_start = sources_start + brace_pos;
    let section_end = find_matching_brace(content, section_start, b'{', b'}')
        .context("Could not find closing brace for sources")?;

    Ok((section_start + 1, section_end)) // +1 to move past opening brace
}

fn find_pinned_packages_section(content: &str) -> Result<(usize, usize)> {
    let section_start = content
        .find("pinnedPackages = {")
        .context("Could not find pinnedPackages section in pins. nix")?;

    let brace_pos = content[section_start..]
        .find('{')
        .context("Could not find opening brace for pinnedPackages")?;

    let section_start_abs = section_start + brace_pos;
    let section_end = find_matching_brace(content, section_start_abs, b'{', b'}')
        .context("Could not find closing brace for pinnedPackages")?;

    Ok((section_start_abs + 1, section_end)) // +1 to move past opening brace
}

fn find_pin_package_list(content: &str, pin_name: &str) -> Result<(usize, usize)> {
    let (section_start, section_end) = find_pinned_packages_section(content)?;
    let section_content = &content[section_start..section_end];

    let entry_pattern = format!("{} = [", pin_name);
    let entry_pos = section_content.find(&entry_pattern).context(format!(
        "Could not find pin entry '{}' in pinnedPackages",
        pin_name
    ))?;

    let list_start_pos = section_content[entry_pos..]
        .find('[')
        .context("Could not find opening bracket for pin's package list")?;

    let absolute_list_start = section_start + entry_pos + list_start_pos;
    let list_end = find_matching_brace(content, absolute_list_start, b'[', b']')
        .context("Could not find closing bracket for pin's package list")?;

    Ok((absolute_list_start + 1, list_end)) // +1 to move past opening bracket
}
