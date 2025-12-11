use crate::flake::interface::Package;
use crate::flake::parsers::utils::{
    attribute_path, byte_offset, detect_indentation, multiws, opt_inline_comment, ws,
};
use anyhow::{Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending},
    combinator::opt,
    sequence::delimited,
    IResult,
};

#[derive(Debug, Clone)]
pub struct PackageEntry {
    pub name: String,
    pub comment: Option<String>,
    pub start_pos: usize,
    pub end_pos: usize,
}

#[derive(Debug)]
pub struct PackagesSection {
    pub entries: Vec<PackageEntry>,
    pub section_start: usize,
    pub list_start: usize,
    pub list_end: usize,
    pub section_end: usize,
    pub indentation: String,
}

/// Parse "with pkgs;" prefix (optional)
fn with_pkgs(input: &str) -> IResult<&str, Option<&str>> {
    opt(delimited(ws, tag("with pkgs;"), ws))(input)
}

/// Parse a single package entry with optional comment
fn package_entry<'a>(input: &'a str, base_offset: usize) -> IResult<&'a str, PackageEntry> {
    let start_pos = base_offset + byte_offset(input, input);

    let (input, _) = multiws(input)?;
    let (input, name) = attribute_path(input)?;
    let (input, comment) = opt_inline_comment(input)?;
    let (input, _) = opt(line_ending)(input)?;

    let end_pos = base_offset + byte_offset(input, input);

    Ok((
        input,
        PackageEntry {
            name: name.to_string(),
            comment: comment.map(|c| c.trim().to_string()),
            start_pos,
            end_pos,
        },
    ))
}

/// Parse the full packages section with nom
fn parse_packages(input: &str, base_offset: usize) -> IResult<&str, Vec<PackageEntry>> {
    let (input, _) = ws(input)?;
    let (input, _) = with_pkgs(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('[')(input)?;

    let _list_content_start = base_offset + byte_offset(input, input);

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
        match package_entry(rest, base_offset) {
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

/// Main parser for packages section
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
                section_start,
                list_start,
                list_end,
                section_end,
                indentation,
            })
        }
        Err(e) => Err(anyhow::anyhow!("Failed to parse packages section: {:?}", e)),
    }
}

impl PackagesSection {
    /// Convert to Package structs for FlakeConfig
    pub fn to_packages(&self) -> Vec<Package> {
        self.entries
            .iter()
            .map(|e| Package::new(e.name.clone()))
            .collect()
    }

    /// Add a package
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

        let mut result = String::new();
        result.push_str(&original_content[..self.list_end]);
        result.push_str(&new_entry);
        result.push_str(&original_content[self.list_end..]);

        result
    }

    /// Remove a package
    pub fn remove_package(&self, original_content: &str, name: &str) -> Result<String> {
        let entry = self
            .entries
            .iter()
            .find(|e| e.name == name)
            .context(format!("Package '{}' not found", name))?;

        let before = &original_content[..entry.start_pos];
        let after = &original_content[entry.end_pos..];

        let after = if after.starts_with('\n') {
            &after[1..]
        } else {
            after
        };

        Ok(format!("{}{}", before, after))
    }

    /// Check if a package exists
    pub fn package_exists(&self, name: &str) -> bool {
        self.entries.iter().any(|e| e.name == name)
    }
}

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
  packages = with pkgs; [
    rust-bin.stable.latest.default # From rust-overlay
    rust-analyzer
    pkg-config
  ];
}"#;

        let section = parse_packages_section(content).unwrap();
        assert_eq!(section.entries.len(), 3);
        assert_eq!(section.entries[0].name, "rust-bin. stable.latest.default");
        assert_eq!(
            section.entries[0].comment,
            Some("From rust-overlay".to_string())
        );
        assert_eq!(section.entries[1].name, "rust-analyzer");
    }

    #[test]
    fn test_add_package() {
        let content = r#"{
  packages = with pkgs; [
    rust-analyzer
  ];
}"#;

        let section = parse_packages_section(content).unwrap();
        let new_content = section.add_package(content, "cargo-watch", Some("For watching"));

        assert!(new_content.contains("cargo-watch # For watching"));
    }

    #[test]
    fn test_remove_package() {
        let content = r#"{
  packages = with pkgs; [
    rust-analyzer
    cargo-watch
  ];
}"#;

        let section = parse_packages_section(content).unwrap();
        let new_content = section.remove_package(content, "cargo-watch").unwrap();

        assert!(!new_content.contains("cargo-watch"));
        assert!(new_content.contains("rust-analyzer"));
    }
}
