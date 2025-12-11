use anyhow::{Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending},
    combinator::opt,
    IResult,
};
use std::fs;

use crate::flake::interface::{FlakeConfig, Profile};
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
        let shell_hook = shell_hook_section.content;

        // Create profile
        let profile_name = profile_path
            .file_stem()
            .context("Failed to get profile name")?
            .to_string_lossy()
            .to_string();

        let mut profile = Profile::new(profile_name.clone());
        profile.packages = packages;
        profile.env_vars = env_vars;
        profile.shell_hook = shell_hook;

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
pub fn parse_profile_file(path: &str) -> Result<Profile> {
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
    profile.shell_hook = shell_hook_section.content;

    Ok(profile)
}

#[derive(Debug, Clone)]
pub struct InputEntry {
    pub name: String,
    pub url: String,
    pub start_pos: usize,
    pub end_pos: usize,
}

#[derive(Debug)]
pub struct InputsSection {
    pub entries: Vec<InputEntry>,
    pub section_start: usize,
    pub content_start: usize,
    pub content_end: usize,
    pub section_end: usize,
    pub indentation: String,
}

/// Parse a single input entry:   name. url = "value";
fn input_entry<'a>(input: &'a str, base_offset: usize) -> IResult<&'a str, InputEntry> {
    let start_pos = base_offset + byte_offset(input, input);

    let (input, _) = multiws(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('.')(input)?;
    let (input, _) = tag("url")(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = ws(input)?;
    let (input, url) = string_literal(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(';')(input)?;
    let (input, _) = opt(line_ending)(input)?;

    let end_pos = base_offset + byte_offset(input, input);

    Ok((
        input,
        InputEntry {
            name: name.to_string(),
            url: url.to_string(),
            start_pos,
            end_pos,
        },
    ))
}

/// Parse the full inputs section with nom
fn parse_inputs_with_nom(input: &str, base_offset: usize) -> IResult<&str, Vec<InputEntry>> {
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
        match input_entry(rest, base_offset) {
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
                section_start,
                content_start,
                content_end,
                section_end,
                indentation,
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
    pub fn add_input(&self, original_content: &str, name: &str, url: &str) -> String {
        // Check if exists
        if self.entries.iter().any(|e| e.name == name) {
            return original_content.to_string();
        }

        let new_entry = format!("{}{}. url = \"{}\";\n", self.indentation, name, url);

        let mut result = String::new();
        result.push_str(&original_content[..self.content_end]);
        result.push_str(&new_entry);
        result.push_str(&original_content[self.content_end..]);

        result
    }

    /// Remove an input
    pub fn remove_input(&self, original_content: &str, name: &str) -> Result<String> {
        let entry = self
            .entries
            .iter()
            .find(|e| e.name == name)
            .context(format!("Input '{}' not found", name))?;

        let before = &original_content[..entry.start_pos];
        let after = &original_content[entry.end_pos..];

        let after = if after.starts_with('\n') {
            &after[1..]
        } else {
            after
        };

        Ok(format!("{}{}", before, after))
    }

    /// Update an input's URL
    pub fn update_input(
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

        let new_line = format!("{}{}.url = \"{}\";\n", self.indentation, name, new_url);

        let mut result = String::new();
        result.push_str(&original_content[..entry.start_pos]);
        result.push_str(&new_line);
        result.push_str(&original_content[entry.end_pos..]);

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
    flake-utils. url = "github:numtide/flake-utils";
    profile-lib.url = "github:AEduardo-dev/nix-profile-lib";
  };

  outputs = inputs:  import . /. flk/default.nix inputs;
}"#;

        let section = parse_inputs_section(content).unwrap();

        assert_eq!(section.entries.len(), 3);
        assert_eq!(section.entries[0].name, "nixpkgs");
        assert_eq!(
            section.entries[0].url,
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
    nixpkgs. url = "github:NixOS/nixpkgs/nixos-unstable";
  };
}"#;

        let section = parse_inputs_section(content).unwrap();
        let new_content = section.add_input(content, "rust-overlay", "github:oxalica/rust-overlay");

        assert!(new_content.contains("rust-overlay.url"));
        assert!(new_content.contains("oxalica/rust-overlay"));
    }

    #[test]
    fn test_remove_input() {
        let content = r#"{
  inputs = {
    nixpkgs. url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
}"#;

        let section = parse_inputs_section(content).unwrap();
        let new_content = section.remove_input(content, "flake-utils").unwrap();

        assert!(!new_content.contains("flake-utils"));
        assert!(new_content.contains("nixpkgs"));
    }

    #[test]
    fn test_update_input() {
        let content = r#"{
  inputs = {
    nixpkgs. url = "github:NixOS/nixpkgs/nixos-unstable";
  };
}"#;

        let section = parse_inputs_section(content).unwrap();
        let new_content = section
            .update_input(content, "nixpkgs", "github:NixOS/nixpkgs/nixos-24.05")
            .unwrap();

        assert!(new_content.contains("nixos-24.05"));
        assert!(!new_content.contains("nixos-unstable"));
    }
}
