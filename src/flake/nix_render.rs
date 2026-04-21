//! # Nix String and Attribute Rendering
//!
//! Safe rendering utilities for generating valid Nix syntax.
//!
//! These functions handle proper escaping and formatting when generating
//! Nix code, ensuring that special characters don't break the output.

use std::borrow::Cow;

/// Render a key as a Nix attribute name, quoting if necessary.
///
/// Simple identifiers (alphanumeric, underscore, hyphen) are returned as-is.
/// Keys with special characters are quoted: `"pkgs-f720de5"`.
///
/// # Examples
///
/// ```rust,ignore
/// assert_eq!(nix_attr_key("myKey"), "myKey");
/// assert_eq!(nix_attr_key("my.key"), "\"my.key\"");
/// ```
pub fn nix_attr_key(key: &str) -> Cow<'_, str> {
    let mut chars = key.chars();
    let ok_first = chars
        .next()
        .map(|c| c.is_ascii_alphabetic() || c == '_')
        .unwrap_or(false);

    let ok_rest = chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '\'' || c == '-');

    // Nix actually allows `-` in identifiers in some contexts, but quoted keys are always safe,
    // and avoid edge cases. Here we allow '-' but still quote if something is odd.
    if ok_first && ok_rest {
        Cow::Borrowed(key)
    } else {
        Cow::Owned(format!("\"{}\"", nix_escape_string(key)))
    }
}

/// Escape content for a Nix double-quoted string.
///
/// Handles backslash, quotes, newlines, carriage returns, and tabs.
fn nix_escape_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out
}

/// Render a value as a Nix double-quoted string with proper escaping.
///
/// # Examples
///
/// ```rust,ignore
/// assert_eq!(nix_string("hello"), "\"hello\"");
/// assert_eq!(nix_string("say \"hi\""), "\"say \\\"hi\\\"\"");
/// ```
pub fn nix_string(s: &str) -> String {
    format!("\"{}\"", nix_escape_string(s))
}

/// Render a value as a Nix multiline string (`'' ... ''`).
///
/// # Arguments
///
/// * `s` - The string content
/// * `indent` - The indentation unit (e.g., "  ")
/// * `level` - The current nesting level
///
/// # Example Output
///
/// ```nix
/// ''
///     echo "hello"
///     echo "world"
/// ''
/// ```
pub fn nix_multiline_string(s: &str, indent: &str, level: usize) -> String {
    let inner_indent = indent.repeat(level + 1);
    let lines: Vec<&str> = s.lines().collect();

    if lines.is_empty() {
        return "''''".to_string();
    }

    let mut out = String::from("''\n");
    for line in lines {
        out.push_str(&inner_indent);
        out.push_str(line);
        out.push('\n');
    }
    out.push_str(&indent.repeat(level));
    out.push_str("''");
    out
}

/// Append indentation to a string builder.
///
/// # Arguments
///
/// * `out` - The output string to append to
/// * `indent` - The indentation unit (e.g., "  ")
/// * `level` - Number of indentation levels to add
pub fn indent_line(out: &mut String, indent: &str, level: usize) {
    for _ in 0..level {
        out.push_str(indent);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nix_attr_key_leaves_simple_identifier_unquoted() {
        assert_eq!(nix_attr_key("simple_key"), "simple_key");
        assert_eq!(nix_attr_key("hello-world"), "hello-world");
    }

    #[test]
    fn test_nix_attr_key_quotes_special_identifier() {
        assert_eq!(nix_attr_key("my.key"), "\"my.key\"");
        assert_eq!(nix_attr_key("123key"), "\"123key\"");
    }

    #[test]
    fn test_nix_string_escapes_special_characters() {
        assert_eq!(
            nix_string("say \"hi\"\n\tpath\\file"),
            "\"say \\\"hi\\\"\\n\\tpath\\\\file\""
        );
    }

    #[test]
    fn test_nix_multiline_string_formats_with_indentation() {
        assert_eq!(
            nix_multiline_string("echo hello\necho world", "  ", 1),
            "''\n    echo hello\n    echo world\n  ''"
        );
    }

    #[test]
    fn test_nix_multiline_string_empty_content() {
        assert_eq!(nix_multiline_string("", "  ", 0), "''''");
    }

    #[test]
    fn test_indent_line_repeats_indent_level() {
        let mut output = String::new();
        indent_line(&mut output, "  ", 3);
        assert_eq!(output, "      ");
    }
}
