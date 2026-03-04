use crate::flake::interfaces::utils::INDENT_OUT;
use anyhow::{Context, Result};
use clap::builder::OsStr;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{char, multispace0, space0},
    combinator::{opt, recognize},
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};
use std::{env, fs, path::PathBuf};

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
    }))
    .parse(input)
}

/// Parse a Nix attribute path token (no whitespace), returning the whole token.
/// Example: rust-bin.stable.latest.default, rust-analyzer, pkg-config
pub fn attribute_path_token(input: &str) -> IResult<&str, &str> {
    recognize(take_while1(|c: char| {
        c.is_alphanumeric() || c == '_' || c == '-' || c == '.'
    }))
    .parse(input)
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
    )
    .parse(input)
}

/// Parse one pkgs entry with optional trailing spaces and optional inline comment.
/// Returns only the suffix (everything after `pkgs.`), discarding the comment.
pub fn pkgs_entry(input: &str) -> IResult<&str, &str> {
    terminated(pkgs_suffix, (space0, opt_inline_comment, space0)).parse(input)
}

/// Parse an attribute version (e.g., "1.56.0" or 1.56.0) that comes after a "@" symbol
pub fn attribute_version(input: &str) -> IResult<&str, &str> {
    preceded(
        char('@'),
        take_while1(|c: char| c.is_alphanumeric() || c == '.' || c == '_' || c == '-'),
    )
    .parse(input)
}
pub fn opt_attribute_version(input: &str) -> IResult<&str, Option<&str>> {
    opt(attribute_version).parse(input)
}

/// Parse a string literal in double quotes, handling escaped quotes
pub fn string_literal(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), take_until("\""), char('"')).parse(input)
}

/// Parse a Nix multiline string ('' ...  '')
pub fn multiline_string(input: &str) -> IResult<&str, &str> {
    delimited(tag("''"), take_until("''"), tag("''")).parse(input)
}

/// Parse an inline comment starting with #
pub fn inline_comment(input: &str) -> IResult<&str, &str> {
    preceded((ws, char('#')), take_while(|c| c != '\n')).parse(input)
}

/// Parse optional inline comment
pub fn opt_inline_comment(input: &str) -> IResult<&str, Option<&str>> {
    opt(inline_comment).parse(input)
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

    // ========================================================================
    // PROFILE RESOLUTION TESTS
    // ========================================================================

    #[test]
    fn test_normalize_profile_ref_simple_name() {
        assert_eq!(normalize_profile_ref("rust"), Some("rust".to_string()));
        assert_eq!(
            normalize_profile_ref("my-profile"),
            Some("my-profile".to_string())
        );
    }

    #[test]
    fn test_normalize_profile_ref_with_dot_hash_prefix() {
        assert_eq!(normalize_profile_ref(".#rust"), Some("rust".to_string()));
        assert_eq!(
            normalize_profile_ref(".#my-profile"),
            Some("my-profile".to_string())
        );
    }

    #[test]
    fn test_normalize_profile_ref_with_path_hash() {
        assert_eq!(
            normalize_profile_ref("/path/to/flake#rust"),
            Some("rust".to_string())
        );
        assert_eq!(
            normalize_profile_ref("github:user/repo#profile"),
            Some("profile".to_string())
        );
    }

    #[test]
    fn test_normalize_profile_ref_empty_inputs() {
        assert_eq!(normalize_profile_ref(""), None);
        assert_eq!(normalize_profile_ref("   "), None);
        assert_eq!(normalize_profile_ref("."), None);
        assert_eq!(normalize_profile_ref(".#"), None);
    }

    #[test]
    fn test_normalize_profile_ref_trims_whitespace() {
        assert_eq!(normalize_profile_ref("  rust  "), Some("rust".to_string()));
        assert_eq!(
            normalize_profile_ref("  .#rust  "),
            Some("rust".to_string())
        );
    }

    #[test]
    fn test_is_valid_profile_name_valid() {
        assert!(is_valid_profile_name("rust"));
        assert!(is_valid_profile_name("my-profile"));
        assert!(is_valid_profile_name("profile_1"));
        assert!(is_valid_profile_name("Profile123"));
    }

    #[test]
    fn test_is_valid_profile_name_invalid() {
        assert!(!is_valid_profile_name(""));
        assert!(!is_valid_profile_name("../etc"));
        assert!(!is_valid_profile_name("path/traversal"));
        assert!(!is_valid_profile_name("with spaces"));
        assert!(!is_valid_profile_name("."));
        assert!(!is_valid_profile_name(".."));
        assert!(!is_valid_profile_name("pro\\file"));
    }
}

/// Get the default shell profile from default.nix helper
pub fn get_default_shell_profile() -> Result<String> {
    let content =
        fs::read_to_string(".flk/default.nix").context("Failed to read .flk/default.nix")?;
    if let Some(default_start) = content.find("defaultShell = \"") {
        let search_start = default_start + "defaultShell = \"".len();
        if let Some(end) = content[search_start..].find('"') {
            let value = content[search_start..search_start + end].to_string();
            if !value.trim().is_empty() {
                // Validate profile name to prevent path traversal
                if !is_valid_profile_name(&value) {
                    anyhow::bail!(
                        "Invalid profile name '{}' in default.nix. Profile names must be alphanumeric (with - or _) and cannot contain path separators.",
                        value
                    );
                }
                return Ok(value);
            }
        }
    }
    // Fallback to first profile if no defaultShell set
    get_first_profile_name()
}

pub fn resolve_profile(target: Option<String>) -> Result<String> {
    let profile = if let Some(p) = target.and_then(|p| normalize_profile_ref(&p)) {
        p
    } else if let Ok(env_profile) = env::var("FLK_FLAKE_REF") {
        match normalize_profile_ref(&env_profile) {
            Some(p) => p,
            None => {
                return get_default_shell_profile().context("Could not find default shell profile")
            }
        }
    } else {
        return get_default_shell_profile().context("Could not find default shell profile");
    };

    // Validate profile name to prevent path traversal
    if !is_valid_profile_name(&profile) {
        anyhow::bail!(
            "Invalid profile name '{}'. Profile names must be alphanumeric (with - or _) and cannot contain path separators.",
            profile
        );
    }

    Ok(profile)
}

fn normalize_profile_ref(profile: &str) -> Option<String> {
    let trimmed = profile.trim();
    if trimmed.is_empty() || trimmed == "." || trimmed == ".#" {
        return None;
    }

    if let Some(stripped) = trimmed.strip_prefix(".#") {
        if stripped.is_empty() {
            return None;
        }
        return Some(stripped.to_string());
    }

    if let Some((_, profile_name)) = trimmed.rsplit_once('#') {
        if !profile_name.trim().is_empty() {
            return Some(profile_name.to_string());
        }
    }

    Some(trimmed.to_string())
}

/// Validate profile name for safe file system usage
pub fn is_valid_profile_name(name: &str) -> bool {
    !name.is_empty()
        && name != "."
        && name != ".."
        && !name.contains('/')
        && !name.contains('\\')
        && !name.contains(' ')
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

/// Get first profile name from pofiles directory
fn get_first_profile_name() -> Result<String> {
    let profiles = list_profiles()?;
    if let Some(first_profile) = profiles.first() {
        if let Some(name) = first_profile.file_stem() {
            let name_str = name.to_string_lossy().to_string();
            // Validate profile name to prevent path traversal
            if !is_valid_profile_name(&name_str) {
                anyhow::bail!(
                    "Invalid profile name '{}'. Profile names must be alphanumeric (with - or _) and cannot contain path separators.",
                    name_str
                );
            }
            return Ok(name_str);
        }
    }
    Err(anyhow::anyhow!("No profiles found in .flk/profiles/"))
}

/// List all profile names
pub fn list_profiles() -> Result<Vec<PathBuf>> {
    Ok(std::fs::read_dir(".flk/profiles")
        .context("Failed to read profiles directory")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() == Some(&OsStr::from("nix")))
        .filter(|e| e.path().file_name() != Some(&OsStr::from("default.nix")))
        .map(|e| e.path())
        .collect())
}
