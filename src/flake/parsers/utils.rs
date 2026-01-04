use crate::flake::interfaces::utils::INDENT_OUT;
use anyhow::{Context, Result};
use clap::builder::OsStr;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{char, multispace0, space0},
    combinator::{opt, recognize},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use std::{fs, path::PathBuf};

/// Parse whitespace (spaces and tabs, not newlines)
pub fn ws(input: &str) -> IResult<&str, &str> {
    space0(input)
}

/// Parse whitespace including newlines
pub fn multiws(input: &str) -> IResult<&str, &str> {
    multispace0(input)
}

/// Parse a Nix identifier (alphanumeric + dashes + underscores)
pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(take_while1(|c: char| {
        c.is_alphanumeric() || c == '_' || c == '-'
    }))(input)
}

/// Parse a Nix attribute path token (no whitespace), returning the whole token.
/// Example: rust-bin.stable.latest.default, rust-analyzer, pkg-config
pub fn attribute_path_token(input: &str) -> IResult<&str, &str> {
    recognize(take_while1(|c: char| {
        c.is_alphanumeric() || c == '_' || c == '-' || c == '.'
    }))(input)
}

/// Parse `pkgs.<suffix>` where suffix is either:
/// - a quoted key: pkgs."<anything>"  -> returns inner content
/// - a dotted attribute path: pkgs.rust-bin.stable.latest.default -> returns full suffix
pub fn pkgs_suffix(input: &str) -> IResult<&str, &str> {
    preceded(
        tag("pkgs."),
        alt((
            // pkgs."openssl@3.6.0" -> openssl
            preceded(char('"'), take_while1(|c: char| c != '"' && c != '@')),
            // pkgs.rust-bin.stable.latest.default -> rust-bin.stable.latest.default
            attribute_path_token,
        )),
    )(input)
}

/// Parse one pkgs entry with optional trailing spaces and optional inline comment.
/// Returns only the suffix (everything after `pkgs.`), discarding the comment.
pub fn pkgs_entry(input: &str) -> IResult<&str, &str> {
    terminated(pkgs_suffix, tuple((space0, opt_inline_comment, space0)))(input)
}

/// Parse an attribute version (e.g., "1.56.0" or 1.56.0) that comes after a "@" symbol
pub fn attribute_version(input: &str) -> IResult<&str, &str> {
    preceded(
        char('@'),
        take_while1(|c: char| c.is_alphanumeric() || c == '.' || c == '_' || c == '-'),
    )(input)
}
pub fn opt_attribute_version(input: &str) -> IResult<&str, Option<&str>> {
    opt(attribute_version)(input)
}

/// Parse a string literal in double quotes, handling escaped quotes
pub fn string_literal(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), take_until("\""), char('"'))(input)
}

/// Parse a Nix multiline string ('' ...  '')
pub fn multiline_string(input: &str) -> IResult<&str, &str> {
    delimited(tag("''"), take_until("''"), tag("''"))(input)
}

/// Parse an inline comment starting with #
pub fn inline_comment(input: &str) -> IResult<&str, &str> {
    preceded(tuple((ws, char('#'))), take_while(|c| c != '\n'))(input)
}

/// Parse optional inline comment
pub fn opt_inline_comment(input: &str) -> IResult<&str, Option<&str>> {
    opt(inline_comment)(input)
}

/// Detect indentation pattern from content
pub fn detect_indentation(content: &str) -> String {
    for line in content.lines() {
        if !line.trim().is_empty() {
            let indent = line.len() - line.trim_start().len();
            if indent > 0 {
                return line[..indent].to_string();
            }
        }
    }
    INDENT_OUT.to_string()
}

/// Find the byte position of a substring in the original content
pub fn _find_position(original: &str, substring: &str) -> Option<usize> {
    Some(original.as_ptr() as usize - substring.as_ptr() as usize)
}

/// Calculate byte offset between two string slices
pub fn byte_offset(original: &str, remaining: &str) -> usize {
    remaining.as_ptr() as usize - original.as_ptr() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier() {
        assert_eq!(identifier("rust-analyzer").unwrap().1, "rust-analyzer");
        assert_eq!(identifier("my_var").unwrap().1, "my_var");
    }

    #[test]
    fn test_attribute_path() {
        assert_eq!(
            attribute_path_token("rust-bin.stable.latest.default")
                .unwrap()
                .1,
            "rust-bin.stable.latest.default"
        );
    }

    #[test]
    fn test_string_literal() {
        assert_eq!(string_literal("\"hello world\"").unwrap().1, "hello world");
    }

    #[test]
    fn test_inline_comment() {
        assert_eq!(
            inline_comment("# This is a comment").unwrap().1,
            " This is a comment"
        );
    }
}

/// Get the default shell profile from default.nix helper
pub fn get_default_shell_profile() -> Result<String> {
    let content = fs::read_to_string(".flk/default.nix").context("Failed to read flake.nix")?;
    if let Some(default_start) = content.find("defaultShell = \"") {
        let search_start = default_start + "defaultShell = \"".len();
        if let Some(end) = content[search_start..].find('"') {
            return Ok(content[search_start..search_start + end].to_string());
        }
    }
    // Fallback to first profile if no defaultShell set
    get_first_profile_name()
}

/// Get first profile name from pofiles directory
fn get_first_profile_name() -> Result<String> {
    let profiles = list_profiles()?;
    if let Some(first_profile) = profiles.first() {
        if let Some(name) = first_profile.file_stem() {
            return Ok(name.to_string_lossy().to_string());
        }
    }
    Err(anyhow::anyhow!("No profiles found in .flk/profiles/"))
}

/// List all profile names
pub fn list_profiles() -> Result<Vec<PathBuf>> {
    Ok(std::fs::read_dir(".flk/profiles")
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() == Some(&OsStr::from("nix")))
        .filter(|e| e.path().file_name() != Some(&OsStr::from("default.nix")))
        .map(|e| e.path())
        .collect())
}
