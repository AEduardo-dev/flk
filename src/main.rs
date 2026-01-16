use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod nix;

use crate::commands::{
    activate, add, command, completions, direnv, env,
    export::{self, ExportType},
    init, list, lock, remove, search, show, update,
};

#[derive(Parser)]
#[command(name = "flk")]
#[command(author = "AEduardo-dev")]
#[command(version)]
#[command(about = "A CLI tool for managing flake.nix devShell environments", long_about = None)]
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

    /// Update packages to latest version
    /// TODO: manage version pinning after implementing #7
    Update {
        /// Specific packages to update
        packages: Vec<String>,

        /// Show what would be updated without actually updating
        #[arg(short, long)]
        show: bool,
    },

    /// Manage custom commands in the dev shell
    Command {
        #[command(subcommand)]
        action: CommandAction,
    },
    /// Manage environment variables in the dev shell
    Env {
        #[command(subcommand)]
        action: EnvAction,
    },

    /// Manage flake.lock file
    Lock {
        #[command(subcommand)]
        action: LockAction,
    },

    /// Generate and install shell completions
    Completions {
        /// Install the completions automatically
        #[arg(long)]
        install: bool,

        /// Manually specify shell (if not auto-detected)
        #[arg(value_enum)]
        shell: Option<clap_complete::shells::Shell>,
    },

    /// Reload the current shell environment
    Activate {},

    /// Export flake configurations (Docker, JSON, etc.)
    Export {
        #[arg(short, long)]
        format: ExportType,
    },

    /// Direnv integration
    Direnv {
        #[command(subcommand)]
        action: DirenvAction,
    },
}

#[derive(Subcommand)]
enum CommandAction {
    /// Add a custom command to the dev shell
    Add {
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
    Remove {
        /// Command name to remove
        name: String,
    },
    /// List all custom commands
    List,
}

#[derive(Subcommand)]
enum EnvAction {
    /// Add an environment variable
    Add {
        /// Variable name
        name: String,
        /// Variable value
        value: String,
    },
    /// Remove an environment variable
    Remove {
        /// Variable name
        name: String,
    },
    /// List all environment variables
    List,
}
#[derive(Subcommand)]
enum LockAction {
    /// Show detailed lock file information
    Show,

    /// Show lock file backup history
    History,

    /// Restore lock file from a backup
    Restore {
        /// Backup timestamp or identifier (e.g., "2025-01-27_14-30-00" or "latest")
        backup: String,
    },
}
#[derive(Subcommand)]
enum DirenvAction {
    /// Create .envrc with flake activation
    Init,
    /// Add flake activation to existing .envrc
    Attach,
    /// Remove flake activation from .envrc
    Detach,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { template, force } => {
            init::run(template, force)?;
        }
        Commands::Search { query, limit } => {
            search::run_search(&query, limit)?;
        }
        Commands::DeepSearch { package } => {
            search::run_deep_search(&package)?;
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
        Commands::Update { packages, show } => {
            update::run_update(packages, show)?;
        }
        Commands::Command { action } => match action {
            CommandAction::Add {
                name,
                command,
                file,
            } => {
                let cmd = command.join(" ");
                command::run_add(&name, &cmd, file)?;
            }
            CommandAction::Remove { name } => {
                command::run_remove(&name)?;
            }
            CommandAction::List => {
                command::list()?;
            }
        },
        Commands::Env { action } => match action {
            EnvAction::Add { name, value } => {
                env::add(&name, &value)?;
            }
            EnvAction::Remove { name } => {
                env::remove(&name)?;
            }
            EnvAction::List => {
                env::list()?;
            }
        },
        Commands::Lock { action } => match action {
            LockAction::Show => {
                lock::show()?;
            }
            LockAction::History => {
                lock::history()?;
            }
            LockAction::Restore { backup } => {
                lock::restore(&backup)?;
            }
        },
        Commands::Completions { install, shell } => {
            completions::handle_completions(install, shell)?;
        }
        Commands::Activate {} => {
            activate::run_activate()?;
        }
        Commands::Export { format } => {
            export::run_export(&format)?;
        }
        Commands::Direnv { action } => match action {
            DirenvAction::Init => {
                direnv::direnv_init()?;
            }
            DirenvAction::Attach => {
                direnv::direnv_attach()?;
            }
            DirenvAction::Detach => {
                direnv::direnv_detach()?;
            }
        },
    }

    Ok(())
}
