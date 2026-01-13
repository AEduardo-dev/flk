use crate::flake::interfaces::shellhooks::{ShellHookEntry, ShellHookSection};
use crate::flake::nix_render::{indent_line, nix_multiline_string, nix_string};
use crate::flake::parsers::utils::{detect_indentation, multiline_string, multiws, string_literal};
use anyhow::{Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::map,
    multi::many0,
    sequence::{delimited, preceded, tuple},
    IResult,
};

// ============================================================================
// SHELL HOOK ENTRY PARSERS
// ============================================================================

/// Parse a single command entry:  { name = "... "; script = ''.. .''; }
fn shell_hook_entry(input: &str) -> IResult<&str, ShellHookEntry> {
    map(
        delimited(
            multiws,
            delimited(
                char('{'),
                delimited(
                    multiws,
                    tuple((
                        preceded(
                            tuple((tag("name"), multiws, char('='), multiws)),
                            string_literal,
                        ),
                        preceded(
                            tuple((
                                multiws,
                                char(';'),
                                multiws,
                                tag("script"),
                                multiws,
                                char('='),
                                multiws,
                            )),
                            multiline_string,
                        ),
                        preceded(
                            tuple((multiws, char(';'), multiws)),
                            nom::combinator::success(()),
                        ),
                    )),
                    multiws,
                ),
                char('}'),
            ),
            multiws,
        ),
        |(name, script, _)| ShellHookEntry {
            name: name.to_string(),
            script: script.to_string(),
        },
    )(input)
}

/// Parse a list of command entries: [ { ...  } { ... } ]
fn shell_hook_entry_list(input: &str) -> IResult<&str, Vec<ShellHookEntry>> {
    delimited(
        delimited(multiws, char('['), multiws),
        many0(shell_hook_entry),
        delimited(multiws, char(']'), multiws),
    )(input)
}

/// Parse the full commands section content
fn parse_commands_content(input: &str) -> IResult<&str, Vec<ShellHookEntry>> {
    shell_hook_entry_list(input)
}

/// Main parser for commands section: commands = [ ...  ];
pub fn parse_shell_hook_section(content: &str) -> Result<ShellHookSection> {
    let commands_start = content
        .find("commands")
        .context("Could not find 'commands'")?;

    // Consider line start as section start for indentation consistency
    let section_start = content[..commands_start]
        .rfind('\n')
        .map(|i| i + 1)
        .unwrap_or(0);

    let after_commands = &content[section_start..];
    let bracket_offset = after_commands
        .find('[')
        .context("Could not find '[' after 'commands'")?;

    let list_start = section_start + bracket_offset;

    // Find the matching closing bracket
    let after_bracket = &content[list_start + 1..];
    let mut bracket_count = 1usize;
    let mut list_end = list_start + 1;

    for (i, ch) in after_bracket.char_indices() {
        match ch {
            '[' => bracket_count += 1,
            ']' => {
                bracket_count -= 1;
                if bracket_count == 0 {
                    list_end = list_start + 1 + i + 1; // Include the closing bracket
                    break;
                }
            }
            _ => {}
        }
    }

    if bracket_count != 0 {
        return Err(anyhow::anyhow!("Unmatched brackets in commands section"));
    }

    // Find the semicolon after the closing bracket to get full section end
    let section_end = content[list_end..]
        .find(';')
        .map(|i| list_end + i + 1)
        .unwrap_or(list_end);

    let to_parse = &content[list_start..list_end];

    // Detect indentation from the original content
    let indentation = detect_indentation(&content[section_start..section_end]);

    match parse_commands_content(to_parse) {
        Ok((_, entries)) => Ok(ShellHookSection {
            entries,
            indentation,
            section_start,
            section_end,
        }),
        Err(e) => Err(anyhow::anyhow!(
            "Failed to parse commands section:  {:?}",
            e
        )),
    }
}

// ============================================================================
// RENDER HELPERS
// ============================================================================

pub fn render_commands_section(
    out: &mut String,
    indent: &str,
    level: usize,
    entries: &[ShellHookEntry],
) {
    indent_line(out, indent, level);
    out.push_str("commands = [\n");

    for entry in entries {
        indent_line(out, indent, level + 1);
        out.push_str("{\n");

        indent_line(out, indent, level + 2);
        out.push_str("name = ");
        out.push_str(&nix_string(&entry.name));
        out.push_str(";\n");

        indent_line(out, indent, level + 2);
        out.push_str("script = ");
        out.push_str(&nix_multiline_string(&entry.script, indent, level + 2));
        out.push_str(";\n");

        indent_line(out, indent, level + 1);
        out.push_str("}\n");
    }

    indent_line(out, indent, level);
    out.push_str("];");
}

/// Render just the commands section (for splicing back into a file)
pub fn render_shell_hook_section(section: &ShellHookSection) -> String {
    let indent = if section.indentation.is_empty() {
        "  "
    } else {
        section.indentation.as_str()
    };

    let mut out = String::new();
    render_commands_section(&mut out, indent, 1, &section.entries);
    out
}

// ============================================================================
// COMBINED OPERATIONS (parse -> modify -> render)
// ============================================================================

/// Add a command to the shell hook section and return the updated file content
pub fn add_shell_hook_command(content: &str, name: &str, script: &str) -> Result<String> {
    let mut section = parse_shell_hook_section(content)?;

    section.add_command(name, script)?;

    let rendered = render_shell_hook_section(&section);
    Ok(section.apply_to_content(content, &rendered))
}

/// Remove a command from the shell hook section and return the updated file content
pub fn remove_shell_hook_command(content: &str, name: &str) -> Result<String> {
    let mut section = parse_shell_hook_section(content)?;

    section.remove_command(name)?;

    let rendered = render_shell_hook_section(&section);
    Ok(section.apply_to_content(content, &rendered))
}
//
// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shell_hook_section() {
        let content = r#"{
  packages = [ pkgs.hello ];

  commands = [
    {
      name = "greet";
      script = ''
        echo "Hello, World!"
      '';
    }
    {
      name = "build";
      script = ''
        cargo build --release
      '';
    }
  ];

  env = { FOO = "bar"; };
}"#;

        let section = parse_shell_hook_section(content).unwrap();
        assert_eq!(section.entries.len(), 2);
        assert_eq!(section.entries[0].name, "greet");
        assert!(section.entries[0].script.contains("Hello, World!"));
        assert_eq!(section.entries[1].name, "build");

        // Verify section bounds are captured
        assert!(section.section_start > 0);
        assert!(section.section_end > section.section_start);
    }

    #[test]
    fn test_add_command() {
        let content = r#"{
  packages = [ pkgs.hello ];

  commands = [
    {
      name = "greet";
      script = ''
        echo "Hello"
      '';
    }
  ];

  env = { FOO = "bar"; };
}"#;

        let updated = add_shell_hook_command(content, "test", "echo 'test command'").unwrap();

        assert!(updated.contains("packages = [ pkgs.hello ]"));
        assert!(updated.contains("greet"));
        assert!(updated.contains("test"));
        assert!(updated.contains("echo 'test command'"));
        assert!(updated.contains("env = { FOO = \"bar\"; }"));
    }

    #[test]
    fn test_remove_command() {
        let content = r#"{
  commands = [
    {
      name = "greet";
      script = ''
        echo "Hello"
      '';
    }
    {
      name = "build";
      script = ''
        cargo build
      '';
    }
  ];
}"#;

        let updated = remove_shell_hook_command(content, "greet").unwrap();

        assert!(!updated.contains("greet"));
        assert!(updated.contains("build"));
        assert!(updated.contains("cargo build"));
    }

    #[test]
    fn test_section_bounds_preserved() {
        let content = r#"# Header comment
{
  packages = [ pkgs.hello ];

  commands = [
    {
      name = "greet";
      script = ''
        echo "Hello"
      '';
    }
  ];

  env = { FOO = "bar"; };
}
# Footer comment"#;

        let updated = add_shell_hook_command(content, "new", "echo 'new'").unwrap();

        // Verify content before and after section is preserved
        assert!(updated.starts_with("# Header comment"));
        assert!(updated.contains("packages = [ pkgs.hello ]"));
        assert!(updated.contains("env = { FOO = \"bar\"; }"));
        assert!(updated.ends_with("# Footer comment"));
    }
}
