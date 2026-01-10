use anyhow::{bail, Context, Result};
use colored::Colorize;
use flk::flake::{
    generator,
    parsers::utils::{get_default_shell_profile, list_profiles},
};
use std::fs;
use std::path::{Path, PathBuf};

/// Create a new profile from one of the bundled templates
pub fn add(name: &str, template: Option<String>) -> Result<()> {
    validate_profile_name(name)?;

    let profiles_dir = Path::new(".flk/profiles");
    fs::create_dir_all(profiles_dir).context("Failed to ensure .flk/profiles directory exists")?;

    let profile_path = profiles_dir.join(format!("{}.nix", name));
    if profile_path.exists() {
        bail!("Profile '{}' already exists.", name);
    }

    let template_name = template.unwrap_or_else(|| "base".to_string());
    let generator_key = normalize_template_name(&template_name)?;

    let content = generator::generate_flake(generator_key)
        .with_context(|| format!("Failed to generate '{}' profile template", template_name))?;

    fs::write(&profile_path, content)
        .with_context(|| format!("Failed to write profile file at {}", profile_path.display()))?;

    println!(
        "{} Created profile '{}' using '{}' template.",
        "✓".green().bold(),
        name.magenta(),
        template_name.cyan()
    );

    Ok(())
}

/// Remove an existing profile (and adjust default if needed)
pub fn remove(name: &str) -> Result<()> {
    validate_profile_name(name)?;

    let profile_path = Path::new(".flk/profiles").join(format!("{}.nix", name));
    if !profile_path.exists() {
        bail!("Profile '{}' does not exist.", name);
    }

    let profiles = list_profiles().context("Failed to list profiles")?;
    if profiles.len() <= 1 {
        bail!("Cannot remove the last profile. Create another profile first.");
    }

    let current_default = get_default_shell_profile().unwrap_or_default();

    fs::remove_file(&profile_path)
        .with_context(|| format!("Failed to remove profile file {}", profile_path.display()))?;

    if current_default == name {
        if let Some(new_default) = next_profile_name(&profiles, name) {
            set_default_profile(&new_default)?;
            println!(
                "{} Default profile updated to '{}'.",
                "ℹ".blue(),
                new_default.magenta()
            );
        }
    }

    println!(
        "{} Profile '{}' removed successfully!",
        "✓".green().bold(),
        name.yellow()
    );

    Ok(())
}

/// Set the default profile used by other commands
pub fn set(name: &str) -> Result<()> {
    validate_profile_name(name)?;

    let profile_path = Path::new(".flk/profiles").join(format!("{}.nix", name));
    if !profile_path.exists() {
        bail!("Profile '{}' does not exist.", name);
    }

    set_default_profile(name)?;

    println!(
        "{} Default profile set to '{}'.",
        "✓".green().bold(),
        name.magenta()
    );

    Ok(())
}

fn validate_profile_name(name: &str) -> Result<()> {
    if name.is_empty()
        || name.starts_with('-')
        || !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        bail!(
            "Invalid profile name '{}'. Use letters, numbers, hyphens, or underscores.",
            name
        );
    }

    Ok(())
}

fn normalize_template_name(template: &str) -> Result<&'static str> {
    match template.to_lowercase().as_str() {
        "base" | "generic" => Ok("generic"),
        "rust" => Ok("rust"),
        "python" => Ok("python"),
        "node" => Ok("node"),
        "go" => Ok("go"),
        other => bail!(
            "Unknown template '{}'. Available templates: base, rust, python, node, go.",
            other
        ),
    }
}

fn next_profile_name(profiles: &[PathBuf], excluded: &str) -> Option<String> {
    profiles
        .iter()
        .filter_map(|p| p.file_stem())
        .map(|s| s.to_string_lossy().to_string())
        .find(|name| name != excluded)
}

fn set_default_profile(profile: &str) -> Result<()> {
    let helper_path = Path::new(".flk/default.nix");
    let content = fs::read_to_string(helper_path).context("Failed to read .flk/default.nix")?;

    let marker = "defaultShell = \"";
    if let Some(start) = content.find(marker) {
        let value_start = start + marker.len();
        if let Some(rel_end) = content[value_start..].find('"') {
            let value_end = value_start + rel_end;
            let mut new_content = String::with_capacity(content.len() + profile.len());
            new_content.push_str(&content[..value_start]);
            new_content.push_str(profile);
            new_content.push_str(&content[value_end..]);
            fs::write(helper_path, new_content).context("Failed to write .flk/default.nix")?;
            return Ok(());
        }
    }

    let insert_marker = "      profileFiles =";
    if let Some(pos) = content.find(insert_marker) {
        let mut new_content = String::new();
        new_content.push_str(&content[..pos]);
        new_content.push_str(&format!("      defaultShell = \"{}\";\n", profile));
        new_content.push_str(&content[pos..]);
        fs::write(helper_path, new_content).context("Failed to write .flk/default.nix")?;
        return Ok(());
    }

    bail!("Could not update default shell; expected markers not found in .flk/default.nix")
}
