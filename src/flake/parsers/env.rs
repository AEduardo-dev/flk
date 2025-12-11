use crate::flake::interface::EnvVar;
use crate::flake::parsers::utils::{
    byte_offset, detect_indentation, identifier, multiws, string_literal, ws,
};
use anyhow::{Context, Result};
use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::opt,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone)]
pub struct EnvVarEntry {
    pub name: String,
    pub value: String,
    pub start_pos: usize,
    pub end_pos: usize,
}

#[derive(Debug)]
pub struct EnvVarsSection {
    pub entries: Vec<EnvVarEntry>,
    pub _content_start: usize,
    pub _content_end: usize,
    pub indentation: String,
    pub _section_start: usize,
    pub _section_end: usize,
}

/// Parse a value (quoted string or unquoted identifier)
fn env_value(input: &str) -> IResult<&str, &str> {
    alt((string_literal, identifier))(input)
}

/// Parse a single env var entry:   NAME = "value";
fn env_var_entry<'a>(
    input: &'a str,
    base_offset: usize,
    original_input: &'a str,
) -> IResult<&'a str, EnvVarEntry> {
    let start_pos = base_offset + byte_offset(original_input, input);

    let (remaining, _) = multiws(input)?;
    let (remaining, (name, value)) =
        separated_pair(identifier, tuple((ws, char('='), ws)), env_value)(remaining)?;
    let (remaining, _) = ws(remaining)?;
    let (remaining, _) = char(';')(remaining)?;
    let (remaining, _) = opt(line_ending)(remaining)?;

    let end_pos = base_offset + byte_offset(original_input, remaining);

    Ok((
        remaining,
        EnvVarEntry {
            name: name.to_string(),
            value: value.to_string(),
            start_pos,
            end_pos,
        },
    ))
}
/// Parse the full envVars section with nom
fn parse_env_vars(input: &str, base_offset: usize) -> IResult<&str, Vec<EnvVarEntry>> {
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

        // Try to parse env var entry
        match env_var_entry(rest, base_offset, original_input) {
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

/// Main parser for envVars section
pub fn parse_env_vars_section(content: &str) -> Result<EnvVarsSection> {
    let section_start = content
        .find("envVars =")
        .context("Could not find 'envVars ='")?;

    let parse_from = section_start + "envVars =".len();
    let to_parse = &content[parse_from..];

    match parse_env_vars(to_parse, parse_from) {
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

            let env_content = &content[content_start..content_end];
            let indentation = detect_indentation(env_content);

            Ok(EnvVarsSection {
                entries,
                _content_start: content_start,
                _content_end: content_end,
                indentation,
                _section_start: section_start,
                _section_end: section_end,
            })
        }
        Err(e) => Err(anyhow::anyhow!("Failed to parse envVars section: {:?}", e)),
    }
}

impl EnvVarsSection {
    /// Convert to EnvVar structs
    pub fn to_env_vars(&self) -> Vec<EnvVar> {
        self.entries
            .iter()
            .map(|e| EnvVar::new(e.name.clone(), e.value.clone()))
            .collect()
    }

    /// Add env var
    pub fn add_env_var(&self, original_content: &str, name: &str, value: &str) -> String {
        if self.entries.iter().any(|e| e.name == name) {
            return original_content.to_string();
        }

        let new_entry = format!("{}{} = \"{}\";\n", self.indentation, name, value);

        let mut result = String::new();
        result.push_str(&original_content[..self._content_end]);
        result.push_str(&new_entry);
        result.push_str(&original_content[self._content_end..]);

        result
    }

    /// Remove env var
    pub fn remove_env_var(&self, original_content: &str, name: &str) -> Result<String> {
        let entry = self
            .entries
            .iter()
            .find(|e| e.name == name)
            .context(format!("Environment variable '{}' not found", name))?;

        let before = &original_content[..entry.start_pos];
        let after = &original_content[entry.end_pos..];

        let after = after.strip_prefix('\n').unwrap_or(after);

        Ok(format!("{}{}", before, after))
    }

    /// Check if env var exists
    pub fn env_var_exists(&self, name: &str) -> Result<bool> {
        Ok(self.entries.iter().any(|e| e.name == name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_env_vars() {
        let content = r#"{
  envVars = {
    RUST_BACKTRACE = "1";
    MY_VAR = "value";
  };
}"#;

        let section = parse_env_vars_section(content).unwrap();
        assert_eq!(section.entries.len(), 2);
        assert_eq!(section.entries[0].name, "RUST_BACKTRACE");
        assert_eq!(section.entries[0].value, "1");
    }

    #[test]
    fn test_add_env_var() {
        let content = r#"{
  envVars = {
    RUST_BACKTRACE = "1";
  };
}"#;

        let section = parse_env_vars_section(content).unwrap();
        let new_content = section.add_env_var(content, "NEW_VAR", "new_value");

        assert!(new_content.contains("NEW_VAR = \"new_value\""));
    }
}
