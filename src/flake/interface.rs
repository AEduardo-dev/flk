use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;

pub const INDENT_IN: &str = "    "; // 4 spaces
pub const INDENT_OUT: &str = "  "; // 2 spaces

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FlakeConfig {
    pub inputs: Vec<String>,
    pub profiles: Vec<Profile>,
}

/// Represents a profile in the flake
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub packages: Vec<Package>,
    pub env_vars: Vec<EnvVar>,
    pub shell_hook: String,
}

impl Profile {
    pub fn new(name: String) -> Self {
        Self {
            name,
            packages: Vec::new(),
            env_vars: Vec::new(),
            shell_hook: String::new(),
        }
    }
}

/// Represents a package in the flake
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: Option<String>,
}

/// Represents an environment variable in the flake
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}

impl EnvVar {
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

        if !self.shell_hook.is_empty() {
            writeln!(f, "  {}", "Shell Hook:".dimmed())?;
            // Show first 100 chars of shell hook
            let preview = if self.shell_hook.len() > 100 {
                format!("{}...", &self.shell_hook[..100])
            } else {
                self.shell_hook.clone()
            };
            writeln!(f, "    {}", preview.dimmed())?;
        }

        Ok(())
    }
}

impl FlakeConfig {
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

    /// Display just the environment variables list (for `flk env list`)
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

    /// Display shell hooks by profile (for `flk hooks` or similar)
    pub fn display_shell_hooks(&self) {
        if self.profiles.is_empty() {
            println!("{}", "No profiles defined".yellow());
            return;
        }

        println!("{}", "Shell Hooks by Profile:".bold().cyan());
        println!();

        for profile in &self.profiles {
            if !profile.shell_hook.is_empty() {
                println!("{}", profile.name.bold().magenta());
                println!("{}", profile.shell_hook.dimmed());
                println!();
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
