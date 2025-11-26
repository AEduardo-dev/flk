use std::{path::Path, process::Command};

use crate::{flake::parser::parse_flake, nix::run_nix_command, utils::visual::with_spinner};
use anyhow::{Context, Ok, Result};
use clap::ValueEnum;

use crate::flake::parser;

#[derive(Debug, Clone, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum ExportType {
    Docker,
    Podman,
    Json,
}

pub fn run_export(export_type: &ExportType) -> Result<()> {
    let profile: String = parser::get_default_shell_profile()
        .context("Could not find default shell profile (flake.nix)")?;
    match export_type {
        ExportType::Docker => {
            println!("Exporting flake.nix to Docker image...");
            let (_, _, success) = with_spinner("<export-docker>", || {
                run_nix_command(&[
                    "build",
                    &format!(".#docker-{}", profile.as_str()),
                    "--out-link",
                    ".flk/result",
                ])
                .context("Failed to build Docker image from flake.nix")
            })?;
            Command::new("docker")
                .args(["load", "<", ".flk/result"])
                .output()
                .context("Failed to load Docker image")?;
            println!(
                "Docker image export {}",
                if success {
                    "succeeded ✅"
                } else {
                    "failed ❌"
                }
            );
        }
        ExportType::Podman => {
            println!("Exporting flake.nix to Podman image...");
            let (_, _, success) = with_spinner("<export-podman>", || {
                run_nix_command(&[
                    "build",
                    &format!(".#podman-{}", profile.as_str()),
                    "--out-link",
                    ".flk/result",
                ])
                .context("Failed to build Podman image from flake.nix")
            })?;
            Command::new("podman")
                .args(["load", "<", ".flk/result"])
                .output()
                .context("Failed to load Podman image")?;
            println!(
                "Podman image export {}",
                if success {
                    "succeeded ✅"
                } else {
                    "failed ❌"
                }
            );
        }
        ExportType::Json => {
            let flake_path = Path::new("flake.nix");
            let flake_content = parse_flake(flake_path.to_str().unwrap())?;

            // Serialize the flake content to JSON file
            let json_output = serde_json::to_string_pretty(&flake_content)
                .context("Failed to serialize flake content to JSON")?;
            std::fs::write("flake.json", json_output).context("Failed to write flake.json file")?;
            println!("Flake export to JSON succeeded ✅");
        }
    }
    Ok(())
}
