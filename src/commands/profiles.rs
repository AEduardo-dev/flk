use anyhow::{bail, Context, Result};
use colored::Colorize;
use regex::Regex;
use std::fs;
use std::path::Path;

use flk::flake::generator;

pub fn run_add(profile: String, template: Option<String>, force: Option<bool>) -> Result<()> {
    let profiles_path = Path::new(".flk/profiles");
    let profile_path = profiles_path.join(format!("{}.nix", profile));

    let template = template.unwrap_or_else(|| "base".to_string());
    let force = force.unwrap_or(false);

    // Check if profiles directory already exists
    if profile_path.exists() && !force {
        bail!(
            "Profile {} already exists! Use {} to overwrite.",
            profile.cyan(),
            "--force".yellow()
        );
    }

    let profile_content = generator::generate_flake(&template)?;

    fs::create_dir_all(profiles_path).context("Failed to create .flk and profiles directories")?;

    // Write to file
    fs::write(format!(".flk/profiles/{}.nix", profile), profile_content)
        .context("Failed to write profile file")?;

    println!("{} Created profile successfully!", "✓".green().bold());

    Ok(())
}

pub fn run_remove(profile: String) -> Result<()> {
    let profiles_path = Path::new(".flk/profiles");
    let profile_path = profiles_path.join(format!("{}.nix", profile));

    // Check if profile exists
    if !profile_path.exists() {
        bail!("Profile {} does not exist!", profile.cyan());
    }

    fs::remove_file(profile_path).context("Failed to remove profile file")?;

    println!("{} Removed profile successfully!", "✓".green().bold());

    Ok(())
}

pub fn run_list() -> Result<()> {
    let profiles_path = Path::new(".flk/profiles");

    // Check if profiles directory exists
    if !profiles_path.exists() {
        bail!("No profiles found! The .flk/profiles directory does not exist.");
    }

    println!("{} Available profiles:", "ℹ".blue());
    for entry in fs::read_dir(profiles_path).context("Failed to read profiles directory")? {
        let entry = entry.context("Failed to read profile entry")?;
        let file_name = entry.file_name();
        let profile_name = file_name
            .to_str()
            .unwrap_or_default()
            .trim_end_matches(".nix");
        println!("- {}", profile_name.cyan());
    }

    Ok(())
}

// Insert in the let block, not at the end of file
pub fn run_set_default(profile: String) -> Result<()> {
    let importer_path = Path::new(".flk/default.nix");
    let profiles_path = Path::new(".flk/profiles");
    let profile_path = profiles_path.join(format!("{}.nix", profile));

    // Check if profile exists
    if !profile_path.exists() {
        bail!("Profile {} does not exist!", profile.cyan());
    }

    let importer_content =
        fs::read_to_string(importer_path).context("Failed to read default.nix file")?;

    // Replace existing defaultShell or insert it into mkProfileOutputs { ... };
    let default_re = Regex::new(r#"(?m)defaultShell\s*=\s*"[^"]*"\s*;"#).unwrap();
    let new_importer_content = if default_re.is_match(&importer_content) {
        default_re
            .replace(
                &importer_content,
                format!(r#"defaultShell = "{}";"#, profile),
            )
            .to_string()
    } else {
        let marker = "profileFiles = builtins.readDir ./profiles;";
        if let Some(start) = importer_content.find(marker) {
            if let Some(rel_end) = importer_content[..start].rfind('\n') {
                let insert_pos = rel_end;
                let (before, after) = importer_content.split_at(insert_pos);
                format!(
                    "{before}      defaultShell = \"{profile}\";\n{after}",
                    before = before,
                    profile = profile,
                    after = after
                )
            } else {
                bail!("Could not find the mkProfileOutputs block in default.nix");
            }
        } else {
            bail!("Could not find the mkProfileOutputs block in default.nix");
        }
    };

    fs::write(importer_path, &new_importer_content).context("Failed to write default.nix file")?;

    println!(
        "{} Set {} as the default profile successfully!",
        "✓".green().bold(),
        profile.cyan()
    );

    Ok(())
}
