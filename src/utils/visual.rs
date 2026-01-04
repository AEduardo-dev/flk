use crate::flake::interfaces::profiles::Package;
use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

pub fn with_spinner<F, T>(message: &str, f: F) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg} [{elapsed_precise}]")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    let result = f();

    spinner.finish_and_clear();
    result
}

pub fn display_list(packages: &[Package]) -> String {
    if packages.is_empty() {
        return String::new();
    }

    let mut output = String::new();

    for (i, pkg) in packages.iter().enumerate() {
        output.push_str(&format!(
            "{} {}\n",
            format!("[{}]", i + 1).cyan().bold(),
            pkg.name.green()
        ));

        if let Some(version) = &pkg.version {
            output.push_str(&format!("  {} {}\n", "Version:".bold(), version.yellow()));
        }

        output.push('\n');
    }

    output
}

pub fn display_table(packages: &[Package]) -> String {
    if packages.is_empty() {
        return String::new();
    }

    let max_name_len = packages
        .iter()
        .map(|p| p.name.len())
        .max()
        .unwrap_or(0)
        .max(4);

    let mut output = format!("{:<width$}  Version\n", "Name", width = max_name_len);

    for pkg in packages {
        if let Some(version) = &pkg.version {
            output.push_str(&format!(
                "{:<width$}  {}\n",
                pkg.name.cyan().bold(),
                version.bright_black(),
                width = max_name_len
            ));
        } else {
            output.push_str(&format!("{}\n", pkg.name.cyan().bold()));
        }
    }

    output
}
