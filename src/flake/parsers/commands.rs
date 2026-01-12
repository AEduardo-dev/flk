use crate::flake::interfaces::shellhooks::{parse_shell_hook, ShellHookSection};
use crate::flake::parsers::utils::{byte_offset, detect_indentation};
use anyhow::{Context, Result};

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
