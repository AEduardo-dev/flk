use anyhow::{Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending},
    combinator::opt,
    IResult,
};
use std::fs;

use crate::flake::interfaces::profiles::{FlakeConfig, Profile};
use crate::flake::parsers::{
    commands::parse_shell_hook_section,
    env::parse_env_vars_section,
    packages::parse_packages_section,
    utils::{
        byte_offset, detect_indentation, identifier, list_profiles, multiws, string_literal, ws,
    },
};

/// Parse the entire flake configuration
pub fn parse_flake(path: &str) -> Result<FlakeConfig> {
    let content = fs::read_to_string(path).context("Failed to read flake.nix file")?;

    // Parse inputs from flake.nix
    let inputs_section =
        parse_inputs_section(&content).context("Failed to parse inputs section")?;

    // Parse profiles from individual profile files
    let profiles_list = list_profiles().context("Failed to list profiles")?;

    let mut profiles = Vec::new();

    for profile_path in profiles_list {
        let profile_data = fs::read_to_string(&profile_path).with_context(|| {
            format!(
                "Failed to read profile file: {}",
                profile_path.to_string_lossy()
            )
        })?;

        // Parse each section using nom parsers
        let packages_section = parse_packages_section(&profile_data).with_context(|| {
            format!(
                "Failed to parse packages in profile: {}",
                profile_path.to_string_lossy()
            )
        })?;

        let env_vars_section = parse_env_vars_section(&profile_data).with_context(|| {
            format!(
                "Failed to parse envVars in profile: {}",
                profile_path.to_string_lossy()
            )
        })?;

        let shell_hook_section = parse_shell_hook_section(&profile_data).with_context(|| {
            format!(
                "Failed to parse shellHook in profile: {}",
                profile_path.to_string_lossy()
            )
        })?;

        // Convert parsed sections to FlakeConfig types
        let packages = packages_section.to_packages();
        let env_vars = env_vars_section.to_env_vars();

        // Create profile
        let profile_name = profile_path
            .file_stem()
            .context("Failed to get profile name")?
            .to_string_lossy()
            .to_string();

        let mut profile = Profile::new(profile_name.clone());
        profile.packages = packages;
        profile.env_vars = env_vars;
        profile.shell_hook = shell_hook_section;

        profiles.push(profile);
    }

    // Build final config
    let config = FlakeConfig {
        inputs: inputs_section.to_input_names(),
        profiles,
    };

    Ok(config)
}

/// Parse a single profile file (useful for testing or individual operations)
pub fn _parse_profile_file(path: &str) -> Result<Profile> {
    let content = fs::read_to_string(path).context("Failed to read profile file")?;

    let packages_section =
        parse_packages_section(&content).context("Failed to parse packages section")?;

    let env_vars_section =
        parse_env_vars_section(&content).context("Failed to parse envVars section")?;

    let shell_hook_section =
        parse_shell_hook_section(&content).context("Failed to parse shellHook section")?;

    let profile_name = std::path::Path::new(path)
        .file_stem()
        .context("Failed to get profile name")?
        .to_string_lossy()
        .to_string();

    let mut profile = Profile::new(profile_name);
    profile.packages = packages_section.to_packages();
    profile.env_vars = env_vars_section.to_env_vars();
    profile.shell_hook = shell_hook_section;

    Ok(profile)
}

#[derive(Debug, Clone)]
pub struct InputEntry {
    pub name: String,
    pub _url: String,
    pub _start_pos: usize,
    pub _end_pos: usize,
}

#[derive(Debug)]
pub struct InputsSection {
    pub entries: Vec<InputEntry>,
    pub _section_start: usize,
    pub _content_start: usize,
    pub _content_end: usize,
    pub _section_end: usize,
    pub _indentation: String,
}

/// Parse a single input entry:   name. url = "value";
fn input_entry<'a>(
    input: &'a str,
    base_offset: usize,
    original_input: &'a str,
) -> IResult<&'a str, InputEntry> {
    let start_pos = base_offset + byte_offset(original_input, input);

    let (remaining, _) = multiws(input)?;
    let (remaining, name) = identifier(remaining)?;
    let (remaining, _) = ws(remaining)?;
    let (remaining, _) = char('.')(remaining)?;
    let (remaining, _) = tag("url")(remaining)?;
    let (remaining, _) = ws(remaining)?;
    let (remaining, _) = char('=')(remaining)?;
    let (remaining, _) = ws(remaining)?;
    let (remaining, url) = string_literal(remaining)?;
    let (remaining, _) = ws(remaining)?;
    let (remaining, _) = char(';')(remaining)?;
    let (remaining, _) = opt(line_ending)(remaining)?;

    let end_pos = base_offset + byte_offset(original_input, remaining);

    Ok((
        remaining,
        InputEntry {
            name: name.to_string(),
            _url: url.to_string(),
            _start_pos: start_pos,
            _end_pos: end_pos,
        },
    ))
}

/// Parse the full inputs section with nom
fn parse_inputs_with_nom(input: &str, base_offset: usize) -> IResult<&str, Vec<InputEntry>> {
    let original_input = input; // Store original for offset calculations
    let (input, _) = ws(input)?;
    let (input, _) = char('{')(input)?;

    let mut entries = Vec::new();
    let mut remaining = input;

    loop {
        // Skip whitespace
        let (rest, _) = multiws(remaining)?;

        // Check for closing brace
        if rest.starts_with('}') {
            remaining = rest;
            break;
        }

        // Try to parse input entry
        match input_entry(rest, base_offset, original_input) {
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

    let (input, _) = char('}')(remaining)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(';')(input)?;

    Ok((input, entries))
}

/// Main parser for inputs section
pub fn parse_inputs_section(content: &str) -> Result<InputsSection> {
    let section_start = content
        .find("inputs =")
        .context("Could not find 'inputs ='")?;

    let parse_from = section_start + "inputs =".len();
    let to_parse = &content[parse_from..];

    match parse_inputs_with_nom(to_parse, parse_from) {
        Ok((remaining, entries)) => {
            let content_start = content[parse_from..]
                .find('{')
                .context("Could not find '{'")?
                + parse_from
                + 1;

            let section_end = parse_from + byte_offset(to_parse, remaining);

            let content_end = content[content_start..section_end]
                .rfind('}')
                .context("Could not find '}'")?
                + content_start;

            let inputs_content = &content[content_start..content_end];
            let indentation = detect_indentation(inputs_content);

            Ok(InputsSection {
                entries,
                _section_start: section_start,
                _content_start: content_start,
                _content_end: content_end,
                _section_end: section_end,
                _indentation: indentation,
            })
        }
        Err(e) => Err(anyhow::anyhow!("Failed to parse inputs section: {:?}", e)),
    }
}

impl InputsSection {
    /// Convert to the list of input names for FlakeConfig
    pub fn to_input_names(&self) -> Vec<String> {
        self.entries.iter().map(|e| e.name.clone()).collect()
    }

    /// Add a new input
    pub fn _add_input(&self, original_content: &str, name: &str, url: &str) -> String {
        // Check if exists
        if self.entries.iter().any(|e| e.name == name) {
            return original_content.to_string();
        }

        let new_entry = format!("{}{}.url = \"{}\";\n", self._indentation, name, url);

        let mut result = String::new();
        result.push_str(&original_content[..self._content_end]);
        result.push_str(&new_entry);
        result.push_str(&original_content[self._content_end..]);

        result
    }

    /// Remove an input
    pub fn _remove_input(&self, original_content: &str, name: &str) -> Result<String> {
        let entry = self
            .entries
            .iter()
            .find(|e| e.name == name)
            .context(format!("Input '{}' not found", name))?;

        let before = &original_content[..entry._start_pos];
        let after = &original_content[entry._end_pos..];

        let after = after.strip_prefix('\n').unwrap_or(after);

        Ok(format!("{}{}", before, after))
    }

    /// Update an input's URL
    pub fn _update_input(
        &self,
        original_content: &str,
        name: &str,
        new_url: &str,
    ) -> Result<String> {
        let entry = self
            .entries
            .iter()
            .find(|e| e.name == name)
            .context(format!("Input '{}' not found", name))?;

        let new_line = format!("{}{}.url = \"{}\";\n", self._indentation, name, new_url);

        let mut result = String::new();
        result.push_str(&original_content[..entry._start_pos]);
        result.push_str(&new_line);
        result.push_str(&original_content[entry._end_pos..]);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_inputs() {
        let content = r#"{
  description = "Development environment managed by flk";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    profile-lib.url = "github:AEduardo-dev/nix-profile-lib";
  };

  outputs = inputs:  import ./.flk/default.nix inputs;
}"#;

        let section = parse_inputs_section(content).unwrap();

        assert_eq!(section.entries.len(), 3);
        assert_eq!(section.entries[0].name, "nixpkgs");
        assert_eq!(
            section.entries[0]._url,
            "github:NixOS/nixpkgs/nixos-unstable"
        );
        assert_eq!(section.entries[1].name, "flake-utils");
        assert_eq!(section.entries[2].name, "profile-lib");

        let names = section.to_input_names();
        assert_eq!(names, vec!["nixpkgs", "flake-utils", "profile-lib"]);
    }

    #[test]
    fn test_add_input() {
        let content = r#"{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
}"#;

        let section = parse_inputs_section(content).unwrap();
        let new_content =
            section._add_input(content, "rust-overlay", "github:oxalica/rust-overlay");

        assert!(new_content.contains("rust-overlay.url"));
        assert!(new_content.contains("oxalica/rust-overlay"));
    }

    #[test]
    fn test_remove_input() {
        let content = r#"{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
}"#;

        let section = parse_inputs_section(content).unwrap();
        let new_content = section._remove_input(content, "flake-utils").unwrap();

        assert!(!new_content.contains("flake-utils"));
        assert!(new_content.contains("nixpkgs"));
    }

    #[test]
    fn test_update_input() {
        let content = r#"{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
}"#;

        let section = parse_inputs_section(content).unwrap();
        let new_content = section
            ._update_input(content, "nixpkgs", "github:NixOS/nixpkgs/nixos-24.05")
            .unwrap();

        assert!(new_content.contains("nixos-24.05"));
        assert!(!new_content.contains("nixos-unstable"));
    }
}
