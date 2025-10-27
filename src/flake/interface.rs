use colored::Colorize;
use std::fmt;

#[derive(Debug, Default)]
pub struct FlakeConfig {
    pub description: String,
    pub inputs: Vec<String>,
    pub packages: Vec<Package>,
    pub env_vars: Vec<EnvVar>,
    pub shell_hook: String,
}
/// Represents a package in the flake
#[derive(Debug, Clone, PartialEq)]
pub struct Package {
    pub name: String,
    pub version: Option<String>, // For future version pinning support
}

/// Represents an environment variable in the flake
#[derive(Debug, Clone, PartialEq)]
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
            version: None,
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

impl FlakeConfig {
    /// Display just the packages list (for `flk list`)
    pub fn display_packages(&self) {
        if self.packages.is_empty() {
            println!("{}", "No packages installed".yellow());
            return;
        }

        println!(
            "{} {}",
            "Installed Packages:".bold().cyan(),
            format!("({})", self.packages.len()).dimmed()
        );
        println!();

        for (i, package) in self.packages.iter().enumerate() {
            println!("  {}. {}", (i + 1).to_string().dimmed(), package);
        }
    }

    /// Display just the environment variables list (for `flk env list`)
    pub fn display_env_vars(&self) {
        if self.env_vars.is_empty() {
            println!("{}", "No environment variables set".yellow());
            return;
        }

        println!(
            "{} {}",
            "Environment Variables:".bold().cyan(),
            format!("({})", self.env_vars.len()).dimmed()
        );
        println!();

        for (i, env_var) in self.env_vars.iter().enumerate() {
            println!("  {}. {}", (i + 1).to_string().dimmed(), env_var);
        }
    }
}

impl fmt::Display for FlakeConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", "Flake Configuration".bold().cyan())?;
        writeln!(f, "{}", "===================".cyan())?;
        writeln!(f)?;

        if !self.description.is_empty() {
            writeln!(f, "{} {}", "Description:".bold(), self.description)?;
            writeln!(f)?;
        }

        if !self.inputs.is_empty() {
            writeln!(f, "{}", "Inputs:".bold().yellow())?;
            for input in &self.inputs {
                writeln!(f, "  {} {}", "•".green(), input)?;
            }
            writeln!(f)?;
        }

        if !self.packages.is_empty() {
            writeln!(
                f,
                "{} {}",
                "Packages:".bold().yellow(),
                format!("({})", self.packages.len()).dimmed()
            )?;
            for package in &self.packages {
                writeln!(f, "  {} {}", "•".green(), package)?;
            }
            writeln!(f)?;
        }

        // Add this section for environment variables
        if !self.env_vars.is_empty() {
            writeln!(
                f,
                "{} {}",
                "Environment Variables:".bold().yellow(),
                format!("({})", self.env_vars.len()).dimmed()
            )?;
            for env_var in &self.env_vars {
                writeln!(f, "  {} {}", "•".green(), env_var)?;
            }
            writeln!(f)?;
        }

        if !self.shell_hook.is_empty() {
            writeln!(f, "{}", "Shell Hook:".bold().yellow())?;
            writeln!(f, "{}", self.shell_hook.dimmed())?;
        }

        Ok(())
    }
}
