use crate::flake::parsers::utils::{identifier, multiws, string_literal};
use anyhow::{Context, Result};
use nom::sequence::preceded;
use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::map,
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::flake::interfaces::{
    overlays::{OverlayEntry, OverlaysSection, PinnedPackage, SourceEntry, SourcesSection},
    utils::INDENT_OUT,
};
use crate::flake::nix_render::{indent_line, nix_attr_key, nix_string};

// ============================================================================
// PINNED PACKAGE PARSERS
// ============================================================================

/// Parse a single pinned package entry:  { pkg = "... "; name = "..."; }
fn pinned_package_entry(input: &str) -> IResult<&str, PinnedPackage> {
    map(
        delimited(
            multiws,
            delimited(
                char('{'),
                delimited(
                    multiws,
                    tuple((
                        preceded(
                            tuple((tag("pkg"), multiws, char('='), multiws)),
                            string_literal,
                        ),
                        preceded(
                            tuple((
                                multiws,
                                char(';'),
                                multiws,
                                tag("name"),
                                multiws,
                                char('='),
                                multiws,
                            )),
                            string_literal,
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
        |(pkg, pin_name, _)| PinnedPackage {
            name: pkg.to_string(),
            pin_name: pin_name.to_string(),
        },
    )(input)
}

fn pinned_package_list(input: &str) -> IResult<&str, Vec<PinnedPackage>> {
    delimited(
        delimited(multiws, char('['), multiws),
        many0(pinned_package_entry),
        delimited(multiws, char(']'), multiws),
    )(input)
}

/// Parse a pin entry: pin_name = [ ... ];
fn overlay_entry(input: &str) -> IResult<&str, OverlayEntry> {
    let (remaining, (_, name, _, _, _, packages, _, _)) = tuple((
        multiws,
        identifier,
        multiws,
        char('='),
        multiws,
        pinned_package_list,
        multiws,
        char(';'),
    ))(input)?;

    Ok((
        remaining,
        OverlayEntry {
            name: name.to_string(),
            packages,
        },
    ))
}

/// Parse the full overlays section: everything between braces `{ ... }`
fn parse_overlays_content(input: &str) -> IResult<&str, Vec<OverlayEntry>> {
    many0(overlay_entry)(input)
}

/// Main parser for overlays section: pinnedPackages = { ... }
pub fn parse_overlay_section(content: &str) -> Result<OverlaysSection> {
    let section_start = content
        .find("pinnedPackages")
        .context("Could not find 'pinnedPackages'")?;

    let after_pinned = &content[section_start..];
    let brace_offset = after_pinned
        .find('{')
        .context("Could not find '{' after 'pinnedPackages'")?;

    let list_start = section_start + brace_offset + 1;

    // Find the matching closing brace
    let after_brace = &content[list_start..];
    let mut brace_count = 1usize;
    let mut list_end = list_start;

    for (i, ch) in after_brace.char_indices() {
        match ch {
            '{' => brace_count += 1,
            '}' => {
                brace_count -= 1;
                if brace_count == 0 {
                    list_end = list_start + i;
                    break;
                }
            }
            _ => {}
        }
    }

    if brace_count != 0 {
        return Err(anyhow::anyhow!(
            "Unmatched braces in pinnedPackages section"
        ));
    }

    let to_parse = &content[list_start..list_end];
    match parse_overlays_content(to_parse) {
        Ok((_, entries)) => Ok(OverlaysSection {
            entries,
            indentation: INDENT_OUT.to_string(),
        }),
        Err(e) => Err(anyhow::anyhow!("Failed to parse overlays section: {:?}", e)),
    }
}

// ============================================================================
// SOURCE ENTRY PARSERS
// ============================================================================

/// Parse a single source entry:  name = "reference";
fn source_entry(input: &str) -> IResult<&str, SourceEntry> {
    let (remaining, (_, name, _, _, _, reference, _, _)) = tuple((
        multiws,
        identifier,
        multiws,
        char('='),
        multiws,
        string_literal,
        multiws,
        char(';'),
    ))(input)?;

    Ok((
        remaining,
        SourceEntry {
            name: name.to_string(),
            reference: reference.to_string(),
        },
    ))
}

/// Parse the full sources section: everything between braces `{ ... }`
fn parse_sources_content(input: &str) -> IResult<&str, Vec<SourceEntry>> {
    many0(source_entry)(input)
}

/// Main parser for sources section: sources = { ... }
pub fn parse_sources_section(content: &str) -> Result<SourcesSection> {
    let section_start = content
        .find("sources")
        .context("Could not find 'sources'")?;

    let after_sources = &content[section_start..];
    let brace_offset = after_sources
        .find('{')
        .context("Could not find '{' after 'sources'")?;

    let list_start = section_start + brace_offset + 1;

    // Find the matching closing brace
    let after_brace = &content[list_start..];
    let mut brace_count = 1usize;
    let mut list_end = list_start;

    for (i, ch) in after_brace.char_indices() {
        match ch {
            '{' => brace_count += 1,
            '}' => {
                brace_count -= 1;
                if brace_count == 0 {
                    list_end = list_start + i;
                    break;
                }
            }
            _ => {}
        }
    }

    if brace_count != 0 {
        return Err(anyhow::anyhow!("Unmatched braces in sources section"));
    }

    let to_parse = &content[list_start..list_end];

    match parse_sources_content(to_parse) {
        Ok((_, entries)) => Ok(SourcesSection {
            entries,
            indentation: INDENT_OUT.to_string(),
        }),
        Err(e) => Err(anyhow::anyhow!("Failed to parse sources section: {:?}", e)),
    }
}

// ============================================================================
// COMBINED OPERATIONS (parse -> modify -> render)
// ============================================================================

/// Normalize indentation between sections so `render_file` produces consistent output.
fn normalize_indentation(sources: &mut SourcesSection, overlays: &mut OverlaysSection) {
    let indent = if !sources.indentation.is_empty() {
        sources.indentation.clone()
    } else if !overlays.indentation.is_empty() {
        overlays.indentation.clone()
    } else {
        "  ".to_string()
    };

    sources.indentation = indent.clone();
    overlays.indentation = indent;
}

/// Add a pinned package (adds source if needed, creates pin entry if needed, adds package)
pub fn add_pinned_package(
    content: &str,
    pin_hash: &str,
    source_ref: &str,
    package: &str,
    version: &str,
) -> Result<String> {
    let pin_name = format!("pkgs-{}", pin_hash);
    let package_alias = format!("{}@{}", package, version);

    let mut sources_section = parse_sources_section(content)?;
    let mut overlays_section = parse_overlay_section(content)?;
    normalize_indentation(&mut sources_section, &mut overlays_section);

    // Step 1: Add source if it doesn't exist
    if !sources_section.source_exists(&pin_name) {
        sources_section.add_source(&pin_name, source_ref)?;
    }

    // Step 2: Add pin entry if it doesn't exist
    if !overlays_section.pin_entry_exists(&pin_name) {
        overlays_section.add_pin_entry(&pin_name)?;
    }

    // Step 3: Add package to pin if it doesn't exist
    if !overlays_section.package_in_pin_exists(&pin_name, &package_alias) {
        overlays_section.add_package_to_pin(&pin_name, package, &package_alias)?;
    }

    Ok(render_file(&sources_section, &overlays_section))
}

/// Remove a pinned package and cleanup if pin is empty
pub fn remove_pinned_package_with_cleanup(content: &str, package: &str) -> Result<String> {
    let mut sources_section = parse_sources_section(content)?;
    let mut overlays_section = parse_overlay_section(content)?;
    normalize_indentation(&mut sources_section, &mut overlays_section);

    // Find the pin entry that contains the package alias
    println!("Current overlays: {:?}", overlays_section.entries);
    let pin_name = overlays_section
        .entries
        .iter()
        .find(|entry| entry.packages.iter().any(|pkg| pkg.name == package))
        .map(|e| e.name.clone())
        .context(format!(
            "Could not find pin entry containing package '{}'",
            package
        ))?;

    // Step 1: Remove the package from the pin entry
    overlays_section.remove_package_from_pin(&pin_name, package)?;

    // Step 2: Remove pin entry and its source if pin is now empty
    let pin_is_empty = overlays_section
        .entries
        .iter()
        .find(|e| e.name == pin_name)
        .is_some_and(|e| e.packages.is_empty());

    if pin_is_empty {
        overlays_section.remove_pin_entry(&pin_name)?;
        sources_section.remove_source(&pin_name)?;
    }

    Ok(render_file(&sources_section, &overlays_section))
}

// ============================================================================
// RENDER HELPERS
// ============================================================================
//
pub fn render_sources_section(
    out: &mut String,
    indent: &str,
    level: usize,
    entries: &[crate::flake::interfaces::overlays::SourceEntry],
) {
    indent_line(out, indent, level);
    out.push_str("sources = {\n");

    for e in entries {
        indent_line(out, indent, level + 1);
        out.push_str(&nix_attr_key(&e.name));
        out.push_str(" = ");
        out.push_str(&nix_string(&e.reference));
        out.push_str(";\n");
    }

    indent_line(out, indent, level);
    out.push_str("};\n");
}

pub fn render_pinned_packages_section(
    out: &mut String,
    indent: &str,
    level: usize,
    overlays: &[crate::flake::interfaces::overlays::OverlayEntry],
) {
    indent_line(out, indent, level);
    out.push_str("pinnedPackages = {\n");

    for overlay in overlays {
        indent_line(out, indent, level + 1);
        out.push_str(&nix_attr_key(&overlay.name));
        out.push_str(" = [\n");

        for pkg in &overlay.packages {
            indent_line(out, indent, level + 2);
            out.push_str("{\n");

            indent_line(out, indent, level + 3);
            out.push_str("pkg = ");
            out.push_str(&nix_string(&pkg.name));
            out.push_str(";\n");

            indent_line(out, indent, level + 3);
            out.push_str("name = ");
            out.push_str(&nix_string(&pkg.pin_name));
            out.push_str(";\n");

            indent_line(out, indent, level + 2);
            out.push_str("}\n");
        }

        indent_line(out, indent, level + 1);
        out.push_str("];\n");
    }

    indent_line(out, indent, level);
    out.push_str("};\n");
}

pub fn render_file(
    sources: &crate::flake::interfaces::overlays::SourcesSection,
    overlays: &crate::flake::interfaces::overlays::OverlaysSection,
) -> String {
    // fall back to two spaces.
    let indent = if sources.indentation.is_empty() {
        "  "
    } else {
        sources.indentation.as_str()
    };

    let mut out = String::new();
    out.push_str("{\n");

    render_sources_section(&mut out, indent, 1, &sources.entries);
    out.push('\n');
    render_pinned_packages_section(&mut out, indent, 1, &overlays.entries);

    out.push_str("}\n");
    out
}

// =============================================================================
// TESTS
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_pinned_package() {
        let original_content = r#"{
  sources = {
    pkgs-abc123 = "github:user/repo/commit";
  };
    pinnedPackages = {
        pkgs-abc123 = [
        {
            pkg = "example-package";
            name = "example-package@1.0.0";
        }
        ];
    };
}"#;

        let updated_content = add_pinned_package(
            original_content,
            "def456",
            "github:another/repo/commit",
            "new-package",
            "2.0.0",
        )
        .context("Failed to add pinned package")
        .unwrap();

        assert!(updated_content.contains("pkgs-abc123"));
        assert!(updated_content.contains("pkgs-def456"));
        assert!(updated_content.contains("example-package@1.0.0"));
        assert!(updated_content.contains("new-package@2.0.0"));
    }

    #[test]
    fn test_add_pinned_package_to_existent_pin() {
        let original_content = r#"{
  sources = {
    pkgs-abc123 = "github:user/repo/commit";
  };
    pinnedPackages = {
        pkgs-abc123 = [
        {
            pkg = "example-package";
            name = "example-package@1.0.0";
        }
        ];
    };
}"#;

        let updated_content = add_pinned_package(
            original_content,
            "abc123",
            "github:user/repo/commit",
            "new-package",
            "2.0.0",
        )
        .context("Failed to add pinned package")
        .unwrap();

        assert!(updated_content.contains("pkgs-abc123"));
        assert!(updated_content.contains("example-package@1.0.0"));
        assert!(updated_content.contains("new-package@2.0.0"));
    }

    #[test]
    fn test_remove_pinned_package() {
        let original_content = r#"{
  sources = {
    pkgs-abc123 = "github:user/repo/commit";
  };
    pinnedPackages = {
        pkgs-abc123 = [
        {
            pkg = "example-package";
            name = "example-package@1.0.0";
        }
        {
            pkg = "another-package";
            name = "another-package@2.0.0";
        }
        ];
    };
}"#;

        let updated_content =
            remove_pinned_package_with_cleanup(original_content, "example-package")
                .context("Failed to remove pinned package")
                .unwrap();

        assert!(!updated_content.contains("example-package"));
        assert!(!updated_content.contains("example-package@1.0.0"));
        assert!(updated_content.contains("another-package@2.0.0"));
        assert!(updated_content.contains("pkgs-abc123"));
    }

    #[test]
    fn test_remove_last_pinned_package_from_existent_pin() {
        let original_content = r#"{
  sources = {
    pkgs-abc123 = "github:user/repo/commit";
  };
    pinnedPackages = {
        pkgs-abc123 = [
        {
            pkg = "example-package";
            name = "example-package@1.0.0";
        }
        ];
    };
}"#;

        let updated_content =
            remove_pinned_package_with_cleanup(original_content, "example-package")
                .context("Failed to remove pinned package")
                .unwrap();

        assert!(!updated_content.contains("pkgs-abc123"));
        assert!(!updated_content.contains("example-package"));
        assert!(!updated_content.contains("example-package@1.0.0"));
    }
}
