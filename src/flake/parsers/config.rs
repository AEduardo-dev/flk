//! # `.flk/config.nix` Parser
//!
//! Read and mutate the tiny project-level config file used by the slim
//! `flk.lib.mkProject` layout.
//!
//! Expected shape:
//!
//! ```nix
//! {
//!   defaultProfile = "rust";
//!   maxCombinations = 3;
//! }
//! ```
//!
//! The file is wholly managed by flk, so a regex-based parser is sufficient —
//! we never need to handle arbitrary user Nix expressions here.

use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::Path;

/// Parsed contents of `.flk/config.nix`.
#[derive(Debug, Clone)]
pub struct Config {
    pub default_profile: String,
    pub max_combinations: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_profile: String::new(),
            max_combinations: 3,
        }
    }
}

/// Return the path to `.flk/config.nix` relative to the current dir.
pub fn config_path() -> &'static Path {
    Path::new(".flk/config.nix")
}

/// True if a slim-layout config file exists in the current project.
pub fn exists() -> bool {
    config_path().exists()
}

/// Read and parse `.flk/config.nix`.
///
/// Missing keys fall back to defaults (empty `defaultProfile`, `maxCombinations = 3`).
pub fn read_config() -> Result<Config> {
    let path = config_path();
    let content = fs::read_to_string(path).with_context(|| {
        format!(
            "Failed to read {}. Is this a slim flk project?",
            path.display()
        )
    })?;
    parse_config(&content)
}

/// Parse `.flk/config.nix` text into a [`Config`].
pub fn parse_config(content: &str) -> Result<Config> {
    let mut cfg = Config::default();

    let default_re = Regex::new(r#"(?m)defaultProfile\s*=\s*"([^"]*)"\s*;"#).unwrap();
    if let Some(caps) = default_re.captures(content) {
        cfg.default_profile = caps[1].to_string();
    }

    let max_re = Regex::new(r#"(?m)maxCombinations\s*=\s*([0-9]+)\s*;"#).unwrap();
    if let Some(caps) = max_re.captures(content) {
        cfg.max_combinations = caps[1]
            .parse::<u32>()
            .context("maxCombinations is not a valid integer")?;
    }

    Ok(cfg)
}

/// Write the given default profile into `.flk/config.nix`.
///
/// If the file already has a `defaultProfile = "...";` line, it is replaced
/// in place. Otherwise the key is inserted just after the opening `{`.
pub fn write_default_profile(profile: &str) -> Result<()> {
    let path = config_path();
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
    let new = set_default_profile_in_text(&content, profile)?;
    fs::write(path, new).with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

/// Pure helper: returns updated content with `defaultProfile` set to `profile`.
pub fn set_default_profile_in_text(content: &str, profile: &str) -> Result<String> {
    let re = Regex::new(r#"(?m)defaultProfile\s*=\s*"[^"]*"\s*;"#).unwrap();
    let replacement = format!(r#"defaultProfile = "{}";"#, profile);

    if re.is_match(content) {
        return Ok(re.replace(content, replacement.as_str()).to_string());
    }

    // Insert just after the opening `{`.
    let open = content.find('{').context("config.nix has no opening '{'")?;
    let (before, after) = content.split_at(open + 1);
    let after_trimmed = after.trim_start_matches('\n');
    let needs_leading_newline = !before.ends_with('\n');
    let joiner = if needs_leading_newline { "\n" } else { "" };
    Ok(format!(
        "{}{}  {}\n{}",
        before, joiner, replacement, after_trimmed
    ))
}

/// Read just the default profile name. Returns `None` if empty.
pub fn read_default_profile() -> Result<Option<String>> {
    let cfg = read_config()?;
    if cfg.default_profile.trim().is_empty() {
        Ok(None)
    } else {
        Ok(Some(cfg.default_profile))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
  defaultProfile = "rust";
  maxCombinations = 3;
}
"#;

    #[test]
    fn parse_basic() {
        let cfg = parse_config(SAMPLE).unwrap();
        assert_eq!(cfg.default_profile, "rust");
        assert_eq!(cfg.max_combinations, 3);
    }

    #[test]
    fn parse_missing_default_profile() {
        let cfg = parse_config("{ maxCombinations = 5; }").unwrap();
        assert_eq!(cfg.default_profile, "");
        assert_eq!(cfg.max_combinations, 5);
    }

    #[test]
    fn parse_empty() {
        let cfg = parse_config("{ }").unwrap();
        assert_eq!(cfg.default_profile, "");
        assert_eq!(cfg.max_combinations, 3);
    }

    #[test]
    fn set_default_profile_replaces_existing() {
        let out = set_default_profile_in_text(SAMPLE, "node").unwrap();
        let cfg = parse_config(&out).unwrap();
        assert_eq!(cfg.default_profile, "node");
        assert_eq!(cfg.max_combinations, 3);
    }

    #[test]
    fn set_default_profile_inserts_when_missing() {
        let input = "{\n  maxCombinations = 4;\n}\n";
        let out = set_default_profile_in_text(input, "go").unwrap();
        let cfg = parse_config(&out).unwrap();
        assert_eq!(cfg.default_profile, "go");
        assert_eq!(cfg.max_combinations, 4);
    }

    #[test]
    fn round_trip_preserves_other_keys() {
        let input = r#"{
  defaultProfile = "old";
  maxCombinations = 7;
}
"#;
        let out = set_default_profile_in_text(input, "new").unwrap();
        assert!(out.contains("maxCombinations = 7;"));
        assert!(out.contains("defaultProfile = \"new\";"));
    }

    #[test]
    fn parse_falls_back_when_max_combinations_is_non_numeric() {
        // Doesn't match the regex (non-digits) → treated as "missing" → default 3.
        let cfg = parse_config("{ maxCombinations = abc; }").unwrap();
        assert_eq!(cfg.max_combinations, 3);
    }
}
