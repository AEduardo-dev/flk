use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod flake;
mod nix;

use commands::{add, add_command, init, remove_command, search};

use crate::commands::{list, remove, show, update};

#[derive(Parser)]
#[command(name = "flk")]
#[command(author = "AEduardo-dev")]
#[command(version = "0.1.0")]
#[command(about = "A CLI tool for managing flake.nix files like Jetify Devbox", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new flake.nix in the current directory
    Init {
        /// Project type (rust, python, node, go, or generic)
        #[arg(short, long)]
        template: Option<String>,

        /// Force overwrite if flake.nix already exists
        #[arg(short, long)]
        force: bool,
    },

    /// Search for packages in nixpkgs
    Search {
        /// Package name to search for
        query: String,

        /// Limit number of results
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },

    /// Get detailed information about a specific package
    DeepSearch {
        /// Full package name
        package: String,

        /// Show version history
        #[arg(short, long)]
        versions: bool,
    },

    /// List the packages of the flake.nix
    List {},
    /// Show flake.nix content in pretty print format
    Show {},

    /// Add a package to the flake.nix
    Add {
        /// Package name to add
        package: String,

        /// Pin to a specific version
        #[arg(short, long)]
        version: Option<String>,
    },

    /// Remove a package from the flake.nix
    Remove { package: String },

    /// Add a custom command to the dev shell
    AddCommand {
        /// Command name
        name: String,

        /// Command definition (bash code)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,

        /// Source from a file instead
        #[arg(short, long)]
        file: Option<String>,
    },

    /// Remove a custom command from the dev shell
    RemoveCommand {
        /// Command name to remove
        name: String,
    },

    /// Update packages to latest version
    /// TODO: manage version pinning after implementing #7
    Update {
        /// Specific packages to update
        packages: Vec<String>,

        /// Show what would be updated without actually updating
        #[arg(short, long)]
        show: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { template, force } => {
            init::run(template, force)?;
        }
        Commands::Search { query, limit } => {
            search::run_search(&query, limit).await?;
        }
        Commands::DeepSearch { package, versions } => {
            search::run_deep_search(&package, versions).await?;
        }
        Commands::List {} => {
            list::run_list()?;
        }
        Commands::Show {} => {
            show::run_show()?;
        }
        Commands::Add { package, version } => {
            add::run_add(&package, version)?;
        }
        Commands::Remove { package } => {
            remove::run_remove(&package)?;
        }
        Commands::AddCommand {
            name,
            command,
            file,
        } => {
            let cmd = command.join(" ");
            add_command::run(&name, &cmd, file)?;
        }
        Commands::RemoveCommand { name } => {
            remove_command::run(&name)?;
        }
        Commands::Update { packages, show } => {
            update::run_update(packages, show)?;
        }
    }

    Ok(())
}
