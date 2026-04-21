//! # Package Section Parser
//!
//! Parser for the `packages = [ ... ];` section in profile files.
//!
//! This module provides functionality to parse, add, and remove packages
//! from profile files while preserving formatting and comments.
//!
//! ## Supported Syntax
//!
//! ```nix
//! packages = [
//!   pkgs.ripgrep
//!   pkgs.rust-bin.stable.latest.default  # From rust-overlay
//!   pkgs."openssl@3.6.0"  # Version pinned
//! ];
//! ```

use crate::flake::interfaces::profiles::Package;
use crate::flake::parsers::utils::{
    byte_offset, detect_indentation, multiws, opt_attribute_version, opt_inline_comment,
    pkgs_suffix, ws,
};
use anyhow::{Context, Result};
use nom::Parser;
use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending},
    combinator::opt,
    sequence::delimited,
    IResult,
};

/// A parsed package entry with position information.
#[derive(Debug, Clone)]
pub struct PackageEntry {
    /// Package name (without `pkgs.` prefix)
    pub name: String,
    /// Optional version if pinned
    pub version: Option<String>,
    /// Optional inline comment
    pub _comment: Option<String>,
    /// Byte position where this entry starts
    pub start_pos: usize,
    /// Byte position where this entry ends
    pub end_pos: usize,
}

/// Parsed packages section with editing support.
#[derive(Debug)]
pub struct PackagesSection {
    /// All package entries in the section
    pub entries: Vec<PackageEntry>,
    /// Detected indentation for consistent formatting
    pub indentation: String,
    /// Byte position of the list start bracket
    pub _list_start: usize,
    /// Byte position of the list end bracket
    pub list_end: usize,
    /// Byte position where the section starts
    pub _section_start: usize,
    /// Byte position where the section ends
    pub _section_end: usize,
}

/// Parse "with pkgs;" prefix (optional)
fn with_pkgs(input: &str) -> IResult<&str, Option<&str>> {
    opt(delimited(ws, tag("with pkgs;"), ws)).parse(input)
}

/// Parse a single package entry with optional comment
fn package_entry<'a>(
    input: &'a str,
    base_offset: usize,
    original_input: &'a str,
) -> IResult<&'a str, PackageEntry> {
    let start_pos = base_offset + byte_offset(original_input, input);

    let (remaining, _) = multiws(input)?;
    let (remaining, name) = pkgs_suffix(remaining)?;
    let (remaining, version) = opt_attribute_version(remaining)?;
    let (remaining, comment) = opt_inline_comment(remaining)?;
    let (remaining, _) = opt(line_ending).parse(remaining)?;

    let end_pos = base_offset + byte_offset(original_input, remaining);

    Ok((
        remaining,
        PackageEntry {
            name: name.to_string(),
            version: version.map(|v| v.to_string()),
            _comment: comment.map(|c| c.trim().to_string()),
            start_pos,
            end_pos,
        },
    ))
}

/// Parse the full packages section with nom
fn parse_packages(input: &str, base_offset: usize) -> IResult<&str, Vec<PackageEntry>> {
    let original_input = input; // Store original for offset calculations

    let (input, _) = ws(input)?;
    let (input, _) = with_pkgs(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('[')(input)?;

    // Parse all package entries
    let mut entries = Vec::new();
    let mut remaining = input;

    loop {
        // Skip whitespace
        let (rest, _) = multiws(remaining)?;

        // Check for closing bracket
        if rest.starts_with(']') {
            remaining = rest;
            break;
        }

        // Try to parse package entry
        match package_entry(rest, base_offset, original_input) {
            Ok((rest, entry)) => {
                entries.push(entry);
                remaining = rest;
            }
            Err(_) => {
                // Skip this line if it doesn't parse
                if let Some(newline_pos) = rest.find('\n') {
                    remaining = &rest[newline_pos + 1..];
                } else {
                    break;
                }
            }
        }
    }

    let (input, _) = char(']')(remaining)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(';')(input)?;

    Ok((input, entries))
}

/// Parse the packages section from profile file content.
///
/// # Arguments
///
/// * `content` - The full profile file content
///
/// # Returns
///
/// A `PackagesSection` containing all parsed entries with position information.
///
/// # Errors
///
/// Returns an error if the `packages =` section cannot be found or parsed.
pub fn parse_packages_section(content: &str) -> Result<PackagesSection> {
    let section_start = content
        .find("packages =")
        .context("Could not find 'packages ='")?;

    let parse_from = section_start + "packages =".len();
    let to_parse = &content[parse_from..];

    match parse_packages(to_parse, parse_from) {
        Ok((remaining, entries)) => {
            let list_start = content[parse_from..]
                .find('[')
                .context("Could not find '['")?
                + parse_from
                + 1;

            let section_end = parse_from + byte_offset(to_parse, remaining);

            let list_end = content[list_start..section_end]
                .rfind(']')
                .context("Could not find ']'")?
                + list_start;

            let list_content = &content[list_start..list_end];
            let indentation = detect_indentation(list_content);

            Ok(PackagesSection {
                entries,
                indentation,
                _list_start: list_start,
                list_end,
                _section_start: section_start,
                _section_end: section_end,
            })
        }
        Err(e) => Err(anyhow::anyhow!("Failed to parse packages section: {:?}", e)),
    }
}

impl PackagesSection {
    /// Convert parsed entries to a list of [`Package`] structs.
    pub fn to_packages(&self) -> Vec<Package> {
        self.entries
            .iter()
            .map(|e| Package::new(e.name.clone()))
            .collect()
    }

    /// Add a package to the section, returning the modified file content.
    ///
    /// If the package already exists, returns the original content unchanged.
    ///
    /// # Arguments
    ///
    /// * `original_content` - The full file content
    /// * `name` - Package name to add (with or without `pkgs.` prefix)
    /// * `comment` - Optional inline comment
    pub fn add_package(&self, original_content: &str, name: &str, comment: Option<&str>) -> String {
        // Check if exists
        if self.entries.iter().any(|e| e.name == name) {
            return original_content.to_string();
        }

        let new_entry = if let Some(cmt) = comment {
            format!("{}{} # {}\n", self.indentation, name, cmt)
        } else {
            format!("{}{}\n", self.indentation, name)
        };

        let insertion_point = original_content[..self.list_end]
            .rfind('\n')
            .map(|pos| pos + 1)
            .unwrap_or(self.list_end);

        let mut result = String::new();
        result.push_str(&original_content[..insertion_point]);
        result.push_str(&new_entry);
        result.push_str(&original_content[insertion_point..]);

        result
    }

    /// Remove a package from the section, returning the modified file content.
    ///
    /// # Arguments
    ///
    /// * `original_content` - The full file content
    /// * `name` - Package name to remove
    ///
    /// # Errors
    ///
    /// Returns an error if the package is not found.
    pub fn remove_package(&self, original_content: &str, name: &str) -> Result<String> {
        let entry = self
            .entries
            .iter()
            .find(|e| e.name == name)
            .context(format!("Package '{}' not found", name))?;

        let start_line = original_content[..entry.start_pos]
            .rfind('\n')
            .map(|pos| pos + 1)
            .unwrap_or(0);

        let before = &original_content[..start_line];
        let after = &original_content[entry.end_pos + 1..];

        let after = after.strip_prefix('\n').unwrap_or(after);

        Ok(format!("{}{}", before, after))
    }

    /// Check whether a package with the given name already exists in the section.
    pub fn package_exists(&self, name: &str) -> bool {
        self.entries
            .iter()
            .any(|e| e.name == name || e.name == format!("pkgs.{}", name))
    }
}

/// Extract package information from `nix search` output.
///
/// Used by the search command to parse nix search results into
/// a list of packages.
pub fn extract_packages_from_output(output: &str) -> Result<Vec<Package>> {
    Ok(output
        .lines()
        .skip(1)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packages() {
        let content = r#"{
  packages = [
    pkgs.rust-bin.stable.latest.default  # From rust-overlay
    pkgs.rust-analyzer
    pkgs.pkg-config
  ];
}"#;

        let section = parse_packages_section(content).unwrap();
        println!("{:#?}", section);
        assert_eq!(section.entries.len(), 3);
        assert_eq!(section.entries[0].name, "rust-bin.stable.latest.default");
        assert_eq!(
            section.entries[0]._comment,
            Some("From rust-overlay".to_string())
        );
        assert_eq!(section.entries[1].name, "rust-analyzer");
    }

    #[test]
    fn test_add_package() {
        let content = r#"{
  packages = [
    pkgs.rust-analyzer
  ];
}"#;

        let section = parse_packages_section(content).unwrap();
        let new_content = section.add_package(content, "cargo-watch", Some("For watching"));

        assert!(new_content.contains("cargo-watch # For watching"));
    }

    #[test]
    fn test_remove_package() {
        let content = r#"{
  packages = [
    pkgs.rust-analyzer
    pkgs.cargo-watch
  ];
}"#;

        let section = parse_packages_section(content).unwrap();
        let new_content = section.remove_package(content, "cargo-watch").unwrap();

        assert!(!new_content.contains("cargo-watch"));
        assert!(new_content.contains("rust-analyzer"));
    }

    #[test]
    fn test_add_package_with_version() {
        let content = r#"{
  packages = [
    pkgs.rust-analyzer
  ];
}"#;

        let section = parse_packages_section(content).unwrap();
        let new_content = section.add_package(
            content,
            "pkgs.\"versioned_pkg@1.0.0\"",
            Some("For versions"),
        );
        println!("{}", new_content);

        assert!(new_content.contains("pkgs.\"versioned_pkg@1.0.0\" # For versions"));
    }

    #[test]
    fn test_remove_package_with_version() {
        let content = r#"{
  packages = [
    pkgs.rust-analyzer
    pkgs."cargo-watch@2.0.0"
  ];
}"#;

        let section = parse_packages_section(content).unwrap();
        let new_content = section.remove_package(content, "cargo-watch").unwrap();

        assert!(!new_content.contains("cargo-watch"));
        assert!(new_content.contains("rust-analyzer"));
    }

    #[test]
    fn test_extract_packages_from_output_skips_header_and_invalid_lines() {
        let output = r#"Name Version Description
ripgrep 14.1.0 x86_64-linux Fast grep
invalid line
fd 9.0.0 x86_64-linux Fast find
"#;

        let packages = extract_packages_from_output(output).unwrap();

        assert_eq!(packages.len(), 2);
        assert_eq!(packages[0].name, "ripgrep");
        assert_eq!(packages[0].version.as_deref(), Some("14.1.0"));
        assert_eq!(packages[1].name, "fd");
        assert_eq!(packages[1].version.as_deref(), Some("9.0.0"));
    }
}
