use std::borrow::Cow;

/// Nix identifiers are more restrictive than your keys.
/// When a key isn't a simple identifier, emit it as a quoted attribute name:  "pkgs-f720de5" = ...
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
/// Handles backslash, quotes, newlines, tabs.
/// NOTE: If you need to prevent `${...}` interpolation, you'd need extra handling.
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

pub fn nix_string(s: &str) -> String {
    format!("\"{}\"", nix_escape_string(s))
}

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

pub fn indent_line(out: &mut String, indent: &str, level: usize) {
    for _ in 0..level {
        out.push_str(indent);
    }
}
