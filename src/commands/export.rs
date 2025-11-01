use std::process::Command;

use crate::{nix::run_nix_command, utils::visual::with_spinner};
use anyhow::{Context, Ok, Result};
use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum ExportType {
    Docker,
    // Json,
}

pub fn run_export(export_type: &ExportType) -> Result<()> {
    match export_type {
        ExportType::Docker => {
            println!("Exporting flake.nix to Docker image...");
            let (_, _, success) = with_spinner("<export-docker>", || {
                run_nix_command(&["build", ".#docker", "--out-link", ".flk/result"])
                    .context("Failed to build Docker image from flake.nix")
            })?;
            Command::new("docker")
                .args(&["load", "<", ".flk/result"])
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
        } // ExportType::Json => {
          //     //TODO: Implement JSON export logic for packages, shellhooks, and devcontainers
          //     Ok(());
          // }
    }
    Ok(())
}
