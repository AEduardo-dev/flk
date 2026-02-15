//! # Profile Data Types
//!
//! Core data structures representing flake configuration elements.
//! These types are used throughout the codebase for parsing, manipulation,
//! serialization, and display of flake configurations.

use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::flake::interfaces::shellhooks::ShellHookSection;

/// Complete configuration parsed from a flake project.
///
/// This is the top-level structure representing all configuration
/// extracted from a flk-managed project, including inputs from the
/// root `flake.nix` and profiles from `.flk/profiles/*.nix`.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FlakeConfig {
    /// Names of flake inputs (e.g., "nixpkgs", "flake-utils")
    pub inputs: Vec<String>,
    /// Development environment profiles
    pub profiles: Vec<Profile>,
}

/// A development environment profile.
///
/// Profiles are individual development environment configurations stored in
/// `.flk/profiles/<name>.nix`. Each profile can have its own set of packages,
/// environment variables, and custom commands.
///
/// # Example Profile Structure (Nix)
///
/// ```nix
/// {
///   packages = [ pkgs.nodejs pkgs.typescript ];
///   envVars = { NODE_ENV = "development"; };
///   commands = [
///     { name = "dev"; script = ''npm run dev''; }
///   ];
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    /// Profile name (derived from filename, e.g., "rust" from "rust.nix")
    pub name: String,
    /// Packages included in this profile
    pub packages: Vec<Package>,
    /// Environment variables set when this profile is active
    pub env_vars: Vec<EnvVar>,
    /// Custom shell commands available in this profile
    pub shell_hook: ShellHookSection,
}

impl Profile {
    /// Create a new empty profile with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The profile name (typically matches the filename without `.nix`)
    pub fn new(name: String) -> Self {
        Self {
            name,
            packages: Vec::new(),
            env_vars: Vec::new(),
            shell_hook: ShellHookSection {
                entries: Vec::new(),
                indentation: "  ".to_string(),
                section_start: 0,
                section_end: 0,
            },
        }
    }
}

/// A package in the development environment.
///
/// Packages can be either unpinned (latest from nixpkgs) or pinned
/// to a specific version using the nix-versions mechanism.
///
/// # Representation in Nix
///
/// - Unpinned: `pkgs.ripgrep`
/// - Pinned: `pkgs."ripgrep@15.1.0"` (with corresponding entry in `pins.nix`)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Package {
    /// Package name (e.g., "ripgrep", "python312")
    pub name: String,
    /// Optional version string if pinned
    pub version: Option<String>,
}

/// An environment variable in the development environment.
///
/// Environment variables are set when entering the dev shell and
/// are available to all commands and processes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvVar {
    /// Variable name (e.g., "DATABASE_URL", "NODE_ENV")
    pub name: String,
    /// Variable value
    pub value: String,
}

impl EnvVar {
    /// Create a new environment variable.
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

impl fmt::Display for EnvVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.name.cyan().bold(), self.value.green())
    }
}

impl Package {
    /// Create a new package with "latest" as the default version.
    ///
    /// # Arguments
    ///
    /// * `name` - The package name as it appears in nixpkgs
    pub fn new(name: String) -> Self {
        Self {
            name,
            version: Some("latest".to_string()),
        }
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(version) = &self.version {
            write!(
                f,
                "{} {}",
                self.name.green(),
                format!("({})", version).dimmed()
            )
        } else {
            write!(f, "{}", self.name.green())
        }
    }
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name.bold().magenta())?;

        if !self.packages.is_empty() {
            writeln!(
                f,
                "  {} {}",
                "Packages:".dimmed(),
                format!("({})", self.packages.len()).dimmed()
            )?;
            for pkg in &self.packages {
                writeln!(f, "    {} {}", "•".green(), pkg)?;
            }
        }

        if !self.env_vars.is_empty() {
            writeln!(
                f,
                "  {} {}",
                "Environment Variables:".dimmed(),
                format!("({})", self.env_vars.len()).dimmed()
            )?;
            for env in &self.env_vars {
                writeln!(f, "    {} {}", "•".green(), env)?;
            }
        }

        if !self.shell_hook.entries.is_empty() {
            writeln!(f, "  {}", "Commands:".dimmed())?;
            for entry in &self.shell_hook.entries {
                writeln!(f, "    {} {}", "•".green(), entry.name.bold())?;
            }
        }

        Ok(())
    }
}

impl FlakeConfig {
    /// Display all packages grouped by profile. (Internal use)
    pub fn _display_packages(&self) {
        if self.profiles.is_empty() {
            println!("{}", "No profiles defined".yellow());
            return;
        }

        println!(
            "{} {}",
            "Profiles:".bold().cyan(),
            format!("({})", self.profiles.len()).dimmed()
        );
        println!();

        for profile in &self.profiles {
            println!("{}", profile.name.bold().magenta());

            if !profile.packages.is_empty() {
                println!(
                    "  {} {}",
                    "Packages:".dimmed(),
                    format!("({})", profile.packages.len()).dimmed()
                );
                for pkg in &profile.packages {
                    println!("    {} {}", "•".green(), pkg);
                }
            } else {
                println!("  {}", "No packages".dimmed());
            }
            println!();
        }
    }

    /// Display environment variables grouped by profile.
    ///
    /// Used by the `flk env list` command to show all configured
    /// environment variables across all profiles.
    pub fn display_env_vars(&self) {
        if self.profiles.is_empty() {
            println!("{}", "No profiles defined".yellow());
            return;
        }

        println!("{}", "Environment Variables by Profile:".bold().cyan());
        println!();

        for profile in &self.profiles {
            if !profile.env_vars.is_empty() {
                println!(
                    "{} {}",
                    profile.name.bold().magenta(),
                    format!("({})", profile.env_vars.len()).dimmed()
                );
                for env_var in &profile.env_vars {
                    println!("  {} {}", "•".green(), env_var);
                }
                println!();
            }
        }
    }

    /// Display shell hooks (custom commands) grouped by profile.
    ///
    /// Used by the `flk command list` command to show all configured
    /// custom commands across all profiles.
    pub fn display_shell_hooks(&self) {
        if self.profiles.is_empty() {
            println!("{}", "No profiles defined".yellow());
            return;
        }

        println!("{}", "Shell Hooks by Profile:".bold().cyan());
        println!();

        for profile in &self.profiles {
            if !profile.shell_hook.entries.is_empty() {
                println!("{}", profile.name.bold().magenta());
                for entry in &profile.shell_hook.entries {
                    println!("  {} {}", "•".green(), entry.name.bold());
                }
            }
        }
    }
}

impl fmt::Display for FlakeConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", "Flake Configuration".bold().cyan())?;
        writeln!(f, "{}", "===================".cyan())?;
        writeln!(f)?;

        if !self.inputs.is_empty() {
            writeln!(f, "{}", "Inputs:".bold().yellow())?;
            for input in &self.inputs {
                writeln!(f, "  {} {}", "•".green(), input)?;
            }
            writeln!(f)?;
        }

        if !self.profiles.is_empty() {
            writeln!(
                f,
                "{} {}",
                "Profiles:".bold().yellow(),
                format!("({})", self.profiles.len()).dimmed()
            )?;
            for profile in &self.profiles {
                writeln!(f, "{}", profile)?;
            }
        }

        Ok(())
    }
}
