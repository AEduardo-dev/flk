use crate::flake::parsers::utils::{multiline_string, ws};
use anyhow::Result;
use nom::{character::complete::char, IResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShellHookSection {
    pub entries: Vec<ShellHookEntry>,
    pub indentation: String,
    pub section_start: usize,
    pub section_end: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShellHookEntry {
    pub name: String,
    pub script: String,
}

/// Parse shellHook with nom
pub fn parse_shell_hook(input: &str) -> IResult<&str, &str> {
    let (input, _) = ws(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = ws(input)?;
    let (input, content) = multiline_string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(';')(input)?;

    Ok((input, content))
}

impl ShellHookSection {
    /// Check if command exists
    pub fn command_exists(&self, name: &str) -> bool {
        self.entries.iter().any(|entry| entry.name == name)
    }

    /// Add a command to shell hook
    pub fn add_command(&mut self, name: &str, script: &str) -> Result<()> {
        if self.command_exists(name) {
            Err(anyhow::anyhow!(
                "Command '{}' already exists in shellHook",
                name
            ))
        } else {
            self.entries.push(ShellHookEntry {
                name: name.to_string(),
                script: script.to_string(),
            });
            Ok(())
        }
    }

    /// Remove a command from shell hook
    pub fn remove_command(&mut self, name: &str) -> Result<()> {
        if !self.command_exists(name) {
            Err(anyhow::anyhow!(
                "Command '{}' does not exist in shellHook",
                name
            ))
        } else {
            self.entries.retain(|entry| entry.name != name);
            Ok(())
        }
    }
    /// Apply modifications back to the original file content
    pub fn apply_to_content(&self, original_content: &str, rendered_section: &str) -> String {
        let mut result = String::new();
        result.push_str(&original_content[..self.section_start]);
        result.push_str(rendered_section);
        result.push_str(&original_content[self.section_end..]);
        result
    }
}
