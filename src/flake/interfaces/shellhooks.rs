//! # Shell Hook Types
//!
//! Data structures for managing custom shell commands in development environments.
//!
//! Custom commands are defined in profile files and become available as shell
//! functions when the development environment is activated.
//!
//! ## Structure in Profile
//!
//! ```nix
//! {
//!   commands = [
//!     { name = "dev"; script = ''npm run dev''; }
//!     { name = "test"; script = ''cargo test --all''; }
//!   ];
//! }
//! ```

use crate::flake::parsers::utils::{multiline_string, ws};
use anyhow::Result;
use nom::{character::complete::char, IResult};
use serde::{Deserialize, Serialize};

/// A collection of shell hook entries with position tracking for editing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShellHookSection {
    /// Custom command entries
    pub entries: Vec<ShellHookEntry>,
    /// Detected indentation for consistent formatting
    pub indentation: String,
    /// Byte position where the section starts in the source file
    pub section_start: usize,
    /// Byte position where the section ends in the source file
    pub section_end: usize,
}

/// A single custom command definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShellHookEntry {
    /// Command name (becomes the shell function name)
    pub name: String,
    /// Bash script to execute when the command is invoked
    pub script: String,
}

/// Parse a shellHook value from Nix syntax.
///
/// Expects input starting after the `shellHook` identifier.
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
    /// Check if a command with the given name exists.
    pub fn command_exists(&self, name: &str) -> bool {
        self.entries.iter().any(|entry| entry.name == name)
    }

    /// Add a new command to the shell hook section.
    ///
    /// # Errors
    ///
    /// Returns an error if a command with the same name already exists.
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

    /// Remove a command from the shell hook section.
    ///
    /// # Errors
    ///
    /// Returns an error if the command doesn't exist.
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
    /// Apply modifications back to the original file content.
    ///
    /// Replaces the section between `section_start` and `section_end`
    /// with the rendered section content.
    pub fn apply_to_content(&self, original_content: &str, rendered_section: &str) -> String {
        let mut result = String::new();
        result.push_str(&original_content[..self.section_start]);
        result.push_str(rendered_section);
        result.push_str(&original_content[self.section_end..]);
        result
    }
}
