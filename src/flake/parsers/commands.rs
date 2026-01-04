use crate::flake::interfaces::utils::INDENT_IN;
use crate::flake::parsers::utils::{byte_offset, detect_indentation, multiline_string, ws};
use anyhow::{Context, Result};
use nom::{character::complete::char, IResult};

#[derive(Debug)]
pub struct ShellHookSection {
    pub content: String,
    pub content_start: usize,
    pub content_end: usize,
    pub _indentation: String,
    pub _section_start: usize,
    pub _section_end: usize,
}

/// Parse shellHook with nom
fn parse_shell_hook(input: &str) -> IResult<&str, &str> {
    let (input, _) = ws(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = ws(input)?;
    let (input, content) = multiline_string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(';')(input)?;

    Ok((input, content))
}

/// Main parser for shellHook section
pub fn parse_shell_hook_section(content: &str) -> Result<ShellHookSection> {
    let section_start = content
        .find("shellHook")
        .context("Could not find 'shellHook'")?;

    let parse_from = section_start + "shellHook".len();
    let to_parse = &content[parse_from..];

    match parse_shell_hook(to_parse) {
        Ok((remaining, hook_content)) => {
            // Find positions of '' delimiters
            let content_start_marker = content[parse_from..]
                .find("''")
                .context("Could not find opening \"''\"")?;
            let content_start = parse_from + content_start_marker + 2;

            let section_end = parse_from + byte_offset(to_parse, remaining);

            let content_end = content[content_start..section_end]
                .rfind("''")
                .context("Could not find closing \"''\"")?
                + content_start;

            let indentation = detect_indentation(hook_content);

            Ok(ShellHookSection {
                content: hook_content.trim().to_string(),
                content_start,
                content_end,
                _indentation: indentation,
                _section_start: section_start,
                _section_end: section_end,
            })
        }
        Err(e) => Err(anyhow::anyhow!(
            "Failed to parse shellHook section: {:?}",
            e
        )),
    }
}

impl ShellHookSection {
    /// Find a command within the shell hook
    pub fn find_command(&self, full_content: &str, name: &str) -> Option<(usize, usize)> {
        let marker = format!("# flk-command: {}", name);
        let hook_content = &full_content[self.content_start..self.content_end];

        let marker_pos = hook_content.find(&marker)?;
        let marker_start = self.content_start + marker_pos;

        // Find line start
        let line_start = if marker_start > 0 {
            full_content[..marker_start].rfind('\n').unwrap_or(0)
        } else {
            0
        };

        // Find end of function block
        let search_from = marker_start + marker.len();
        let function_end = full_content[search_from..].find(&format!("{}}}", INDENT_IN))?;

        let end_point = search_from + function_end + format!("{}}}", INDENT_IN).len();

        Some((line_start, end_point))
    }

    /// Check if command exists
    pub fn command_exists(&self, full_content: &str, name: &str) -> bool {
        let marker = format!("# flk-command: {}", name);
        full_content.contains(&marker)
    }

    /// Add a command to shell hook
    pub fn add_command(&self, original_content: &str, name: &str, command: &str) -> String {
        let insertion_point = original_content[..self.content_end]
            .rfind('\n')
            .unwrap_or(self.content_end);

        let command_block = format!(
            "\n{indent_in}# flk-command: {name}\n{indent_in}{name} () {{\n{indent_cmd}{cmd}\n{indent_in}}}",
            indent_in = INDENT_IN,
            indent_cmd = " ".repeat(INDENT_IN.len() + 2),
            name = name,
            cmd = command.trim(),
        );

        let mut result = String::new();
        result.push_str(&original_content[..insertion_point]);
        result.push_str(&command_block);
        result.push_str(&original_content[insertion_point..]);

        result
    }

    /// Remove a command from shell hook
    pub fn remove_command(&self, original_content: &str, name: &str) -> Result<String> {
        let (line_start, end_point) = self
            .find_command(original_content, name)
            .context(format!("Command '{}' not found in shellHook", name))?;

        let mut result = String::new();
        result.push_str(&original_content[..line_start]);
        result.push_str(&original_content[end_point..]);

        Ok(result)
    }

    /// Replace entire shell hook content
    pub fn _replace_content(&self, original_content: &str, new_content: &str) -> String {
        let mut result = String::new();
        result.push_str(&original_content[..self.content_start]);
        result.push('\n');
        result.push_str(new_content.trim());
        result.push_str("\n  ");
        result.push_str(&original_content[self.content_end..]);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shell_hook() {
        let content = r#"{
  shellHook = ''
    echo "🦀 Rust development environment ready!"
    echo "Rust version: $(rustc --version)"
  '';
}"#;

        let section = parse_shell_hook_section(content).unwrap();
        assert!(section.content.contains("Rust development environment"));
        assert!(section.content.contains("rustc --version"));
    }

    #[test]
    fn test_add_command() {
        let content = r#"{
  shellHook = ''
    echo "Hello"
  '';
}"#;

        let section = parse_shell_hook_section(content).unwrap();
        let new_content = section.add_command(content, "test", "echo 'test command'");

        assert!(new_content.contains("# flk-command: test"));
        assert!(new_content.contains("test () {"));
    }
}
