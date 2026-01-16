use std::{path::Path, process::Command};

use anyhow::{Context, Ok, Result};
use clap::ValueEnum;
use std::fs::File;

use crate::nix::run_nix_command;
use flk::flake::parsers::{flake::parse_flake, utils::get_default_shell_profile};
use flk::utils::visual::with_spinner;

#[derive(Debug, Clone, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum ExportType {
    Docker,
    Podman,
    Json,
}

pub fn run_export(export_type: &ExportType) -> Result<()> {
    let profile: String =
        get_default_shell_profile().context("Could not find default shell profile (flk.nix)")?;
    match export_type {
        ExportType::Docker => {
            println!("Exporting flk.nix to Docker image...");
            let (_, _, success) = with_spinner("<export-docker>", || {
                run_nix_command(&[
                    "build",
                    &format!(".#docker-{}", profile.as_str()),
                    "--out-link",
                    ".flk/result",
                    "--impure",
                ])
                .context("Failed to build Docker image from flk.nix")
            })?;
            println!("Docker image created successfully ✅");
            let file = File::open(".flk/result").context("Failed to open .flk/result")?;

            let output = with_spinner("<load-image>", || {
                Command::new("docker")
                    .args(["load"])
                    .stdin(file)
                    .output()
                    .context("Failed to load Docker image")
            })?;
            println!(
                "Docker image export {}",
                if success {
                    "succeeded ✅"
                } else {
                    "failed ❌"
                }
            );
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        ExportType::Podman => {
            println!("Exporting flk.nix to Podman image...");
            let (_, _, success) = with_spinner("<export-podman>", || {
                run_nix_command(&[
                    "build",
                    &format!(".#podman-{}", profile.as_str()),
                    "--out-link",
                    ".flk/result",
                    "--impure",
                ])
                .context("Failed to build Podman image from flk.nix")
            })?;
            println!("Podman image created successfully ✅");
            let file = File::open(".flk/result").context("Failed to open .flk/result")?;

            let output = with_spinner("<load-image>", || {
                Command::new("podman")
                    .args(["load"])
                    .stdin(file)
                    .output()
                    .context("Failed to load Podman image")
            })?;
            println!(
                "Podman image export {}",
                if success {
                    "succeeded ✅"
                } else {
                    "failed ❌"
                }
            );
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        ExportType::Json => {
            let flake_path = Path::new("flk.nix");
            let flake_content = parse_flake(flake_path.to_str().unwrap())?;

            // Serialize the flake content to JSON file
            let json_output = serde_json::to_string_pretty(&flake_content)
                .context("Failed to serialize flake content to JSON")?;
            std::fs::write("flk.json", json_output).context("Failed to write flk.json file")?;
            println!("Flake export to JSON succeeded ✅");
        }
    }
    Ok(())
}
