use anyhow::{Context, Result};
use std::fs;

use crate::flake::interface::{EnvVar, FlakeConfig, Profile};
use crate::flake::parsers::{
    commands::parse_shell_hook_from_profile, env::parse_env_vars_from_profile,
    packages::parse_packages_from_profile, utils::list_profiles,
};

//
pub fn parse_flake(path: &str) -> Result<FlakeConfig> {
    let content = fs::read_to_string(path).context("Failed to read flake.nix file")?;

    let profiles_list = list_profiles().context("Failed to list profiles")?;

    let mut profiles = Vec::new();
    for profile_path in profiles_list {
        let profile_data = fs::read_to_string(&profile_path).with_context(|| {
            format!(
                "Failed to read profile file: {}",
                profile_path.to_string_lossy()
            )
        })?;
        let packages =
            parse_packages_from_profile(&profile_data, Some(profile_path.to_str().unwrap()))?;
        let env_vars =
            parse_env_vars_from_profile(&profile_data, Some(profile_path.to_str().unwrap()))?;
        let shell_hook =
            parse_shell_hook_from_profile(&profile_data, Some(profile_path.to_str().unwrap()))
                .unwrap_or_default(); // Use empty string if no shell hook

        let env_vars: Vec<EnvVar> = env_vars
            .into_iter()
            .map(|(name, value)| EnvVar::new(name, value))
            .collect();

        let mut profile = Profile::new(profile_data.to_string().clone());
        profile.name = profile_path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();
        profile.packages = packages;
        profile.env_vars = env_vars;
        profile.shell_hook = shell_hook;

        profiles.push(profile);
    }

    let config = FlakeConfig {
        inputs: parse_inputs(&content),
        profiles,
    };

    Ok(config)
}

/// Extract flake inputs
fn parse_inputs(content: &str) -> Vec<String> {
    let mut inputs = Vec::new();

    if let Some(inputs_start) = content.find("inputs = {") {
        let search_start = inputs_start + "inputs = {".len();
        if let Some(inputs_end) = content[search_start..].find("};") {
            let inputs_section = &content[search_start..search_start + inputs_end];

            for line in inputs_section.lines() {
                let trimmed = line.trim();
                if let Some(dot_pos) = trimmed.find('.') {
                    let input_name = trimmed[..dot_pos].trim();
                    if !input_name.is_empty() && !inputs.contains(&input_name.to_string()) {
                        inputs.push(input_name.to_string());
                    }
                }
            }
        }
    }

    inputs
}
